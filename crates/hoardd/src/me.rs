//! What's each person's own in the library: the assets they've starred (favorites), the searches they've saved,
//! their dragon hoard (how much of the library they've collected), and which of what's new in Hoard they've seen. Looking is enough for all of them, since none
//! changes the library. An access token or connected app acts for the account that made it.

use crate::AppState;
use crate::auth::Who;
use crate::catalog::{Access, Operation};
use crate::db;
use crate::error::{ApiResult, AppError};
use crate::i18n::tr;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use rusqlite::{Connection, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

/// What's the signed-in person's own.
pub(crate) fn operations() -> Vec<Operation> {
    use Access::SignedIn;
    vec![
        SignedIn
            .get("/api/me/favorites", list_favorites)
            .named("list_favorites", "The assets you've starred, most recently starred first."),
        SignedIn.post("/api/me/favorites", change_favorites).named(
            "change_favorites",
            "Star or unstar assets; answers with all of them. Ids that aren't assets are passed over.",
        ),
        SignedIn
            .get("/api/me/searches", list_searches)
            .named("list_saved_searches", "Your saved searches, and which the library opens to."),
        SignedIn.post("/api/me/searches", save_search).named(
            "save_search",
            "Save a search (what the library's address holds after `?`); one with the same name is replaced. Answers \
             with all of them.",
        ),
        SignedIn.put("/api/me/searches/{id}", change_search).named(
            "change_saved_search",
            "Rename a saved search, change what it searches, or make it what the library opens to. Answers with all \
             of them.",
        ),
        SignedIn
            .delete("/api/me/searches/{id}", delete_search)
            .named("delete_saved_search", "Delete one of your saved searches."),
        SignedIn.get("/api/me/hoard", get_hoard).named(
            "get_collected",
            "What you've collected in the portal's collecting game, or `null` before you've collected anything.",
        ),
        SignedIn.put("/api/me/hoard", set_hoard).named(
            "set_collected",
            "Record what you've collected in the portal's collecting game, replacing what was recorded.",
        ),
        SignedIn.get("/api/me/news", get_news).named(
            "get_news_seen",
            "What you've seen of what's new in Hoard, or `null` before anything's been recorded.",
        ),
        SignedIn
            .put("/api/me/news", set_news)
            .named("set_news_seen", "Record what you've seen of what's new in Hoard, replacing what was recorded."),
    ]
}

/// The person `who` is, or acts for.
fn owner(who: &Who) -> ApiResult<i64> {
    who.owner().ok_or_else(|| AppError::forbidden(tr!("error-token-account-gone")))
}

/// A person's favorite assets, most recently starred first.
pub(crate) fn favorite_ids(c: &Connection, user: i64) -> anyhow::Result<Vec<String>> {
    let mut stmt = c.prepare("SELECT asset_id FROM favorites WHERE user_id = ?1 ORDER BY added_at DESC, rowid DESC")?;
    Ok(stmt.query_map([user], |r| r.get(0))?.collect::<rusqlite::Result<_>>()?)
}

async fn list_favorites(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let ids = db::call(&state.pool, move |c| favorite_ids(c, user)).await?;
    Ok(Json(json!({ "assets": ids })))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct FavoriteChange {
    #[serde(default)]
    add: Vec<String>,
    #[serde(default)]
    remove: Vec<String>,
}

/// Star or unstar assets; answers with all of them. Ids that aren't assets are passed over.
async fn change_favorites(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(change): Json<FavoriteChange>,
) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    if change.add.len() + change.remove.len() > 10_000 {
        return Err(AppError::bad_request(tr!("error-favorites-too-many")));
    }
    let ids = db::call(&state.pool, move |c| {
        let tx = c.transaction()?;
        let now = db::now();
        // Prepared once for the lot: a selection can be thousands of assets.
        let mut add = tx.prepare(
            "INSERT OR IGNORE INTO favorites (user_id, asset_id, added_at)
             SELECT ?1, id, ?3 FROM assets WHERE id = ?2",
        )?;
        for id in &change.add {
            add.execute(params![user, id, now])?;
        }
        let mut remove = tx.prepare("DELETE FROM favorites WHERE user_id = ?1 AND asset_id = ?2")?;
        for id in &change.remove {
            remove.execute(params![user, id])?;
        }
        drop((add, remove));
        let ids = favorite_ids(&tx, user)?;
        tx.commit()?;
        Ok(ids)
    })
    .await?;
    Ok(Json(json!({ "assets": ids })))
}

async fn list_searches(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let searches = db::call(&state.pool, move |c| searches(c, user)).await?;
    Ok(Json(json!({ "searches": searches })))
}

fn searches(c: &Connection, user: i64) -> anyhow::Result<Vec<Value>> {
    let mut stmt = c.prepare(
        "SELECT id, name, query, created_at, is_default FROM saved_searches WHERE user_id = ?1
          ORDER BY name COLLATE NOCASE",
    )?;
    Ok(stmt
        .query_map([user], |r| {
            Ok(json!({ "id": r.get::<_, i64>(0)?, "name": r.get::<_, String>(1)?,
                       "query": r.get::<_, String>(2)?, "created_at": r.get::<_, String>(3)?,
                       "default": r.get::<_, bool>(4)? }))
        })?
        .collect::<rusqlite::Result<_>>()?)
}

/// A search to keep: its name and the portal's query string (`q=rock&kind=texture`).
#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct NewSearch {
    name: String,
    query: String,
}

/// Save a search; one with the same name is replaced. Answers with all of them.
async fn save_search(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(s): Json<NewSearch>,
) -> ApiResult<(StatusCode, Json<Value>)> {
    let user = owner(&who)?;
    let name = s.name.trim().to_owned();
    let query = s.query.trim().trim_start_matches('?').to_owned();
    if name.is_empty() || name.chars().count() > 80 {
        return Err(AppError::bad_request(tr!("error-saved-search-name")));
    }
    if query.len() > 2000 {
        return Err(AppError::bad_request(tr!("error-saved-search-too-long")));
    }
    let all = db::call(&state.pool, move |c| {
        c.execute(
            "INSERT INTO saved_searches (user_id, name, query, created_at) VALUES (?1, ?2, ?3, ?4)
             ON CONFLICT (user_id, name) DO UPDATE SET query = excluded.query",
            params![user, name, query, db::now()],
        )?;
        searches(c, user)
    })
    .await?;
    Ok((StatusCode::CREATED, Json(json!({ "searches": all }))))
}

/// What to change about a saved search: its name, what it searches, or whether the library opens to it.
#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct SearchChange {
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    query: Option<String>,
    #[serde(default)]
    default: Option<bool>,
}

/// Rename a saved search, change what it searches, or make it what the library opens to (one at a time: making one
/// the default stops any other being it). Answers with all of them.
async fn change_search(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
    Json(s): Json<SearchChange>,
) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let name = s.name.map(|n| n.trim().to_owned());
    if name.as_ref().is_some_and(|n| n.is_empty() || n.chars().count() > 80) {
        return Err(AppError::bad_request(tr!("error-saved-search-name")));
    }
    let query = s.query.map(|q| q.trim().trim_start_matches('?').to_owned());
    if query.as_ref().is_some_and(|q| q.len() > 2000) {
        return Err(AppError::bad_request(tr!("error-saved-search-too-long")));
    }
    let s = SearchChange { name, query, default: s.default };
    let all = db::call(&state.pool, move |c| change_saved(c, user, id, &s)).await?;
    let Some(all) = all else {
        return Err(AppError::not_found(tr!("error-saved-search-not-found", id = id)));
    };
    Ok(Json(json!({ "searches": all })))
}

/// [`change_search`]'s change, if the search is the person's; then all of theirs.
fn change_saved(c: &Connection, user: i64, id: i64, s: &SearchChange) -> anyhow::Result<Option<Vec<Value>>> {
    let tx = c.unchecked_transaction()?;
    let mine = tx
        .query_row("SELECT 1 FROM saved_searches WHERE id = ?1 AND user_id = ?2", params![id, user], |_| Ok(()))
        .optional()?
        .is_some();
    if !mine {
        return Ok(None);
    }
    if let Some(name) = &s.name {
        // Another of theirs by that name gives way, as saving under a name does.
        tx.execute(
            "DELETE FROM saved_searches WHERE user_id = ?1 AND name = ?2 AND id != ?3",
            params![user, name, id],
        )?;
        tx.execute("UPDATE saved_searches SET name = ?1 WHERE id = ?2", params![name, id])?;
    }
    if let Some(query) = &s.query {
        tx.execute("UPDATE saved_searches SET query = ?1 WHERE id = ?2", params![query, id])?;
    }
    match s.default {
        Some(true) => {
            tx.execute("UPDATE saved_searches SET is_default = (id = ?1) WHERE user_id = ?2", params![id, user])?
        }
        Some(false) => tx.execute("UPDATE saved_searches SET is_default = 0 WHERE id = ?1", [id])?,
        None => 0,
    };
    tx.commit()?;
    Ok(Some(searches(c, user)?))
}

async fn delete_search(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Path(id): Path<i64>,
) -> ApiResult<StatusCode> {
    let user = owner(&who)?;
    let n = db::call(&state.pool, move |c| {
        Ok(c.execute("DELETE FROM saved_searches WHERE id = ?1 AND user_id = ?2", params![id, user])?)
    })
    .await?;
    if n == 0 {
        return Err(AppError::not_found(tr!("error-saved-search-not-found", id = id)));
    }
    Ok(StatusCode::NO_CONTENT)
}

/// How many assets of each kind (`texture`, `model`…) a person has collected into their hoard. The portal shows the
/// difference between this and what the library holds as loot still to collect.
type Hoard = BTreeMap<String, u64>;

/// The most kinds a hoard records, and the longest name one may have: far more than there are kinds of asset.
const HOARD_KINDS: usize = 32;
const HOARD_KIND_LEN: usize = 32;

fn valid_hoard(seen: &Hoard) -> bool {
    seen.len() <= HOARD_KINDS
        && seen.keys().all(|k| !k.is_empty() && k.len() <= HOARD_KIND_LEN && k.bytes().all(|b| b.is_ascii_lowercase()))
}

/// What a person has collected, or `null` before they've collected anything.
async fn get_hoard(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let seen = db::call(&state.pool, move |c| {
        let raw: Option<String> =
            c.query_row("SELECT hoard_seen FROM users WHERE id = ?1", [user], |r| r.get(0)).optional()?.flatten();
        // A record that no longer reads is as good as none: the hoard starts again.
        Ok(raw.and_then(|r| serde_json::from_str::<Hoard>(&r).ok()))
    })
    .await?;
    Ok(Json(json!({ "seen": seen })))
}

#[derive(Debug, Deserialize, schemars::JsonSchema)]
struct HoardChange {
    seen: Hoard,
}

/// Record what a person has collected, replacing what was recorded. Answers with it.
async fn set_hoard(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(change): Json<HoardChange>,
) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    if !valid_hoard(&change.seen) {
        return Err(AppError::bad_request(tr!("error-hoard-seen")));
    }
    let text = serde_json::to_string(&change.seen).map_err(anyhow::Error::from)?;
    db::call(&state.pool, move |c| {
        c.execute("UPDATE users SET hoard_seen = ?2 WHERE id = ?1", params![user, text])?;
        Ok(())
    })
    .await?;
    Ok(Json(json!({ "seen": change.seen })))
}

/// What's new in Hoard that a person has seen: each new thing's id (the portal's list names them) with when they saw
/// it, and whether they've asked not to have new things pointed out. Kept with the account so that signing in on
/// another browser doesn't point everything out again.
#[derive(Debug, Default, Deserialize, Serialize, schemars::JsonSchema)]
struct News {
    #[serde(default)]
    seen: BTreeMap<String, String>,
    #[serde(default)]
    off: bool,
}

/// The most new things a record keeps, and the longest id or time one may have: far more than are ever new at once.
const NEWS_SEEN: usize = 256;
const NEWS_ID_LEN: usize = 48;
const NEWS_WHEN_LEN: usize = 40;

fn valid_news(n: &News) -> bool {
    n.seen.len() <= NEWS_SEEN
        && n.seen.iter().all(|(id, when)| {
            !id.is_empty()
                && id.len() <= NEWS_ID_LEN
                && id.bytes().all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
                && when.len() <= NEWS_WHEN_LEN
        })
}

/// What a person has seen of what's new, or `null` before anything's been recorded.
async fn get_news(State(state): State<AppState>, Extension(who): Extension<Who>) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    let news = db::call(&state.pool, move |c| {
        let raw: Option<String> =
            c.query_row("SELECT news_seen FROM users WHERE id = ?1", [user], |r| r.get(0)).optional()?.flatten();
        // A record that no longer reads is as good as none.
        Ok(raw.and_then(|r| serde_json::from_str::<News>(&r).ok()))
    })
    .await?;
    Ok(Json(json!({ "news": news })))
}

/// Record what a person has seen of what's new, replacing what was recorded. Answers with it.
async fn set_news(
    State(state): State<AppState>,
    Extension(who): Extension<Who>,
    Json(news): Json<News>,
) -> ApiResult<Json<Value>> {
    let user = owner(&who)?;
    if !valid_news(&news) {
        return Err(AppError::bad_request(tr!("error-news-seen")));
    }
    let text = serde_json::to_string(&news).map_err(anyhow::Error::from)?;
    db::call(&state.pool, move |c| {
        c.execute("UPDATE users SET news_seen = ?2 WHERE id = ?1", params![user, text])?;
        Ok(())
    })
    .await?;
    Ok(Json(json!({ "news": news })))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_saved_search_is_renamed_changed_and_opened_to() {
        let c = db::test_library();
        c.execute_batch(
            "INSERT INTO users (id, username, password_hash, created_at) VALUES (1, 'a', 'x', 'x'), (2, 'b', 'x', 'x');
             INSERT INTO saved_searches (id, user_id, name, query, created_at) VALUES
               (1, 1, 'Rocks', 'q=rock', 'x'), (2, 1, 'Moss', 'q=moss', 'x'), (3, 2, 'Theirs', 'q=x', 'x');",
        )
        .unwrap();
        let change = |id, name: Option<&str>, query: Option<&str>, default| {
            let s = SearchChange { name: name.map(Into::into), query: query.map(Into::into), default };
            change_saved(&c, 1, id, &s).unwrap().map(|all| {
                all.iter()
                    .map(|s| format!("{} {} {} {}", s["id"], s["name"], s["query"], s["default"]))
                    .collect::<Vec<_>>()
            })
        };
        assert_eq!(
            change(1, Some("Stones"), Some("q=stone"), Some(true)).unwrap(),
            [r#"2 "Moss" "q=moss" false"#, r#"1 "Stones" "q=stone" true"#]
        );
        // One opens the library at a time.
        assert_eq!(
            change(2, None, None, Some(true)).unwrap(),
            [r#"2 "Moss" "q=moss" true"#, r#"1 "Stones" "q=stone" false"#]
        );
        // Renamed to another's name, that one gives way.
        assert_eq!(change(1, Some("Moss"), None, None).unwrap(), [r#"1 "Moss" "q=stone" false"#]);
        assert_eq!(change(1, None, None, Some(false)).unwrap(), [r#"1 "Moss" "q=stone" false"#]);
        // Someone else's isn't theirs to change.
        assert_eq!(change(3, Some("Mine"), None, None), None);
    }

    #[test]
    fn a_hoard_names_a_few_short_kinds() {
        let ok: Hoard = [("texture".to_owned(), 12), ("model".to_owned(), 0)].into();
        assert!(valid_hoard(&ok));
        assert!(valid_hoard(&Hoard::new()));
        for bad in ["", "Texture", "two words", &"x".repeat(HOARD_KIND_LEN + 1)] {
            assert!(!valid_hoard(&[(bad.to_owned(), 1)].into()), "{bad:?}");
        }
        let many: Hoard = (0..=HOARD_KINDS).map(|i| ("k".repeat(i % 8 + 1) + &"x".repeat(i / 8), 1)).collect();
        assert!(!valid_hoard(&many));
    }

    #[test]
    fn what_s_new_is_recorded_by_short_ids() {
        let news = |ids: &[&str]| News {
            seen: ids.iter().map(|id| ((*id).to_owned(), "2026-11-22T10:00:00Z".to_owned())).collect(),
            off: false,
        };
        assert!(valid_news(&news(&["themes-20", "tiles-move"])));
        assert!(valid_news(&News::default()));
        for bad in ["", "Themes", "two words", "a/b", &"x".repeat(NEWS_ID_LEN + 1)] {
            assert!(!valid_news(&news(&[bad])), "{bad:?}");
        }
        let long = News { seen: [("a".to_owned(), "x".repeat(NEWS_WHEN_LEN + 1))].into(), off: true };
        assert!(!valid_news(&long));
        let many: Vec<String> = (0..=NEWS_SEEN).map(|i| format!("n{i}")).collect();
        assert!(!valid_news(&news(&many.iter().map(String::as_str).collect::<Vec<_>>())));
        // A record from before `off` reads as pointing things out.
        let old: News = serde_json::from_str(r#"{"seen":{"a":"2026-11-01"}}"#).unwrap();
        assert!(!old.off && old.seen.len() == 1);
    }
}
