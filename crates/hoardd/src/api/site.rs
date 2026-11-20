//! A web page in a pack, shown as it would be opened from the folder it's in: its pictures, styles, scripts and
//! other pages load from beside it. Someone else's page mustn't act as whoever opens it, so it's served sandboxed: a
//! page with no origin of its own, which can't read the portal's cookies, storage or answers, nor reach anything but
//! the pack's own files.
//!
//! Those files come from an address signed for the pack (`/api/site/{token}/{path}`), not by the account opening
//! it: a sandboxed page sends no cookies, so its pictures and scripts couldn't be asked for otherwise. The signature
//! is made with a key that lives only as long as the server, and lasts a day.

use std::sync::LazyLock;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::extract::{Path, State};
use axum::http::{HeaderMap, HeaderValue, StatusCode, header};
use axum::response::{IntoResponse, Response};
use serde_json::{Value, json};

use crate::AppState;
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;

/// How long a page's address works. The page is opened again from Hoard for a new one.
const LIFETIME: Duration = Duration::from_hours(24);

/// The files a site may run: the page's own, from its own address, scripts and styles written into it included.
/// Nothing is sent anywhere else, forms included, and only the portal may show it in a frame.
const POLICY: &str = "default-src 'self' data: blob:; script-src 'self' 'unsafe-inline' 'unsafe-eval' data: blob:; \
     style-src 'self' 'unsafe-inline' data: blob:; form-action 'none'; base-uri 'self'; frame-ancestors 'self'; \
     sandbox allow-scripts";

/// Pages that can be shown so, by extension.
pub(crate) fn is_page(ext: &str) -> bool {
    matches!(ext.to_ascii_lowercase().as_str(), "html" | "htm" | "xhtml")
}

static KEY: LazyLock<[u8; 32]> = LazyLock::new(|| {
    let mut key = [0u8; 32];
    // Without randomness from the OS the addresses can't be trusted, so none will check out.
    if getrandom::fill(&mut key).is_err() {
        tracing::warn!("no randomness from the OS; pages in packs can't be shown");
    }
    key
});

fn now() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

fn signature(pack: &str, until: u64) -> String {
    let mac = blake3::keyed_hash(&KEY, format!("site\0{pack}\0{until}").as_bytes());
    mac.to_hex()[..32].to_owned()
}

/// A token for the pack's files: the pack, when it stops working, and its signature.
fn token(pack: &str, until: u64) -> String {
    format!("{pack}.{until:x}.{}", signature(pack, until))
}

/// The pack a token is for, when it's genuine and still good.
fn check(token: &str, at: u64) -> Option<&str> {
    let mut parts = token.rsplitn(3, '.');
    let (sig, until, pack) = (parts.next()?, parts.next()?, parts.next()?);
    let until = u64::from_str_radix(until, 16).ok()?;
    if *KEY == [0u8; 32] || until < at {
        return None;
    }
    // Compared in full whatever differs, so the time taken says nothing about how close a guess was.
    let want = signature(pack, until);
    let same = want.len() == sig.len() && want.bytes().zip(sig.bytes()).fold(0u8, |d, (a, b)| d | (a ^ b)) == 0;
    same.then_some(pack)
}

/// `GET /api/assets/{id}/site`: where to open a page in a pack so it shows as it would from its folder.
pub(super) async fn asset_site(State(state): State<AppState>, Path(id): Path<String>) -> ApiResult<Json<Value>> {
    let id2 = id.clone();
    let (pack, path, ext) = db::call(&state.pool, move |c| {
        use rusqlite::OptionalExtension as _;
        Ok(c.query_row("SELECT pack_id, path, ext FROM assets WHERE id = ?1", [&id2], |r| {
            Ok((r.get::<_, String>(0)?, r.get::<_, String>(1)?, r.get::<_, String>(2)?))
        })
        .optional()?)
    })
    .await?
    .ok_or_else(|| AppError::not_found(tr!("error-asset-not-found", id = id)))?;
    if !is_page(&ext) {
        return Err(AppError::bad_request(tr!("error-site-not-a-page")));
    }
    let until = now() + LIFETIME.as_secs();
    let path = path.split('/').map(crate::store_search::percent_encode).collect::<Vec<_>>().join("/");
    Ok(Json(json!({ "url": format!("/api/site/{}/{path}", token(&pack, until)), "expires": until })))
}

/// `GET /api/site/{token}/{path}`: a file of the pack the token is for, by its path in the pack. A folder shows its
/// `index.html`, as a web server would.
pub(super) async fn site_file(
    State(state): State<AppState>,
    Path((token, path)): Path<(String, String)>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let pack = check(&token, now()).ok_or_else(|| AppError::forbidden(tr!("error-site-link-expired")))?.to_owned();
    site_response(&state, &pack, &path, &headers).await
}

/// `GET /api/site/{token}/`: the pack's own front page.
pub(super) async fn site_root(
    State(state): State<AppState>,
    Path(token): Path<String>,
    headers: HeaderMap,
) -> ApiResult<Response> {
    let pack = check(&token, now()).ok_or_else(|| AppError::forbidden(tr!("error-site-link-expired")))?.to_owned();
    site_response(&state, &pack, "", &headers).await
}

async fn site_response(state: &AppState, pack: &str, path: &str, headers: &HeaderMap) -> ApiResult<Response> {
    let wanted = if path.is_empty() || path.ends_with('/') { format!("{path}index.html") } else { path.to_owned() };
    let (p, w) = (pack.to_owned(), wanted.clone());
    let found = db::call(&state.pool, move |c| {
        if let Some(f) = db::pack_file(c, &p, &w)? {
            return Ok(Some(Ok(f)));
        }
        // A folder named without its slash: sent on to it, so the page's own links resolve inside it.
        Ok(db::pack_file(c, &p, &format!("{w}/index.html"))?.map(|_| Err(())))
    })
    .await?;
    let (hash, ext) = match found {
        Some(Ok(f)) => f,
        Some(Err(())) => {
            let to = path.rsplit('/').next().unwrap_or(path);
            let to = format!("{}/", crate::store_search::percent_encode(to));
            return Ok((StatusCode::MOVED_PERMANENTLY, [(header::LOCATION, to)]).into_response());
        }
        None => return Err(AppError::not_found(tr!("error-file-in-pack-not-found", path = wanted, id = pack))),
    };
    let mime = mime_guess::from_ext(&ext).first_or_octet_stream().to_string();
    let mut resp = super::serve_blob(state, &hash, headers, &mime, None).await?;
    let h = resp.headers_mut();
    h.insert(header::CONTENT_SECURITY_POLICY, HeaderValue::from_static(POLICY));
    // The address is the key to the pack's files: a link followed out of the page doesn't tell where it came from.
    h.insert(header::REFERRER_POLICY, HeaderValue::from_static("no-referrer"));
    // A page without an origin asks for its own module scripts and data as a stranger would.
    h.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, HeaderValue::from_static("*"));
    Ok(resp)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_token_is_good_for_its_pack_until_it_runs_out() {
        let pack = "01a1f43b-6208-711c-98ac-b3ba510552dc";
        let t = token(pack, 1_000);
        assert_eq!(check(&t, 999), Some(pack));
        assert_eq!(check(&t, 1_000), Some(pack));
        assert_eq!(check(&t, 1_001), None);
        // Another pack, or a later end, with the same signature.
        let sig = t.rsplit('.').next().unwrap();
        assert_eq!(check(&format!("other.{:x}.{sig}", 1_000), 0), None);
        assert_eq!(check(&format!("{pack}.{:x}.{sig}", 2_000), 0), None);
        assert_eq!(check("nonsense", 0), None);
        assert_eq!(check("", 0), None);
    }

    #[test]
    fn pages_are_html() {
        assert!(is_page("html") && is_page("HTM") && is_page("xhtml"));
        assert!(!is_page("md") && !is_page("svg"));
    }
}
