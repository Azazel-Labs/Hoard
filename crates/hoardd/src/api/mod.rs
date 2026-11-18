//! HTTP API. Changes and admin pages need an account or an access token (`auth::require_user`); reads do too,
//! unless Settings lets anyone browse (`auth::require_reader`).
//!
//! The routes are all here, in `router`; the handlers are in a module per thing they serve.

use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::{get, post};
use axum::{Json, Router};
use serde_json::json;

use crate::AppState;
use crate::locations;

mod admin;
mod assets;
mod bundles;
mod collections;
mod convert;
mod files;
mod packs;
mod settings;
mod stats;

pub(crate) use files::serve_blob;
pub(crate) use packs::default_pack_name;

pub(crate) fn router(state: &AppState) -> Router<AppState> {
    // The upload limit is a setting that can change while the server runs, so it's checked per request.
    let upload_limit = || {
        tower::ServiceBuilder::new()
            .layer(DefaultBodyLimit::disable())
            .layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), limit_upload))
    };
    // Changes to the library.
    let edits = Router::new()
        .route("/api/packs", post(packs::upload_pack).layer(upload_limit()))
        .route("/api/packs/{id}", axum::routing::patch(packs::patch_pack))
        .route("/api/tags/merge", post(assets::merge_tags))
        .route("/api/tags/aliases", get(assets::tag_aliases).delete(assets::drop_tag_alias))
        .route("/api/packs/{id}/retag", post(packs::retag_pack))
        .route("/api/packs/{id}/read-files", post(packs::read_pack_files))
        .route("/api/packs/{id}/context", axum::routing::put(packs::put_pack_context))
        .route("/api/packs/{id}/store-matches", get(crate::store_search::store_matches))
        .route("/api/packs/{id}/store/undo", post(packs::undo_store_page))
        .route("/api/packs/{id}/unreal/convert", post(convert::unreal_convert))
        .route("/api/packs/{id}/unreal/finish", post(convert::unreal_finish))
        .route("/api/assets/{id}/replace", post(convert::replace_asset).layer(upload_limit()))
        .route("/api/collections", post(collections::create_collection))
        .route(
            "/api/collections/{id}",
            axum::routing::patch(collections::patch_collection).delete(collections::delete_collection),
        )
        .route("/api/collections/{id}/assets", post(collections::collection_members))
        .route("/api/assets/{id}", axum::routing::patch(assets::patch_asset))
        .route(
            "/api/assets/{id}/thumb",
            axum::routing::put(crate::previews::put_thumb)
                .layer(DefaultBodyLimit::max(crate::previews::MAX_DRAWN_BYTES)),
        )
        .route("/api/assets/bulk", post(assets::bulk_assets))
        .route("/api/archive", post(assets::archive))
        .route("/api/archive/restore", post(assets::restore))
        .merge(crate::uploads::router())
        .merge(crate::projects::edit_router())
        .route_layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), crate::auth::require_editor));

    // Settings, accounts, maintenance, and deleting for good.
    let admin = Router::new()
        .route("/api/packs/{id}", axum::routing::delete(packs::delete_pack))
        .route("/api/archive/purge", post(assets::purge))
        .route("/api/admin/scrub", post(admin::admin_scrub))
        .route("/api/admin/gc", post(admin::admin_gc))
        .route("/api/admin/backup", post(admin::admin_backup))
        .route("/api/admin/maintenance", get(crate::maintenance::overview))
        .route("/api/admin/maintenance/{task}", post(crate::maintenance::start_route))
        .route("/api/admin/issues", get(crate::issues::list))
        .route("/api/admin/issues/retry", post(crate::issues::retry))
        .route("/api/admin/issues/dismiss", post(crate::issues::dismiss))
        .route("/api/admin/settings", get(settings::get_settings).put(settings::put_settings))
        .route("/api/admin/conversions/preview", post(convert::conversions_preview))
        .route("/api/admin/conversions/run", post(convert::conversions_run))
        .route("/api/assets/{id}/unconvert", post(convert::unconvert_asset))
        .route("/api/admin/update", get(crate::updates::status_handler))
        .route("/api/admin/update/hook", post(crate::updates::call_handler))
        .route("/api/admin/locations", get(locations::list_locations).post(locations::create_location))
        .route(
            "/api/admin/locations/{id}",
            axum::routing::patch(locations::patch_location).delete(locations::delete_location),
        )
        .route("/api/admin/locations/{id}/scan", post(locations::scan_location))
        .route("/api/admin/locations/{id}/empty", post(locations::empty_location))
        .route("/api/admin/locations/rebalance", post(locations::rebalance))
        .route("/api/admin/fs", get(locations::browse))
        .route("/api/admin/llm/models", post(settings::llm_models))
        .route("/api/admin/llm/local", post(settings::llm_local))
        .route("/api/admin/chatgpt", get(crate::chatgpt::get_status))
        .route("/api/admin/chatgpt/start", post(crate::chatgpt::start))
        .route("/api/admin/chatgpt/finish", post(crate::chatgpt::finish))
        .route("/api/admin/chatgpt/sign-out", post(crate::chatgpt::sign_out_route))
        .route("/api/admin/chatgpt/models", get(crate::chatgpt::list_models))
        .route("/api/admin/jobs", get(admin::admin_jobs))
        .route("/api/admin/profile", get(crate::profiler::handler))
        .merge(crate::format_wizard::router())
        .merge(crate::auth::admin_router())
        .merge(crate::signin::admin_router())
        .merge(crate::https::admin_router())
        .route_layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), crate::auth::require_admin));

    // Everyone's own: their password, access tokens, connected apps, favorites and saved searches.
    let own = Router::new()
        .merge(crate::auth::account_router())
        .merge(crate::me::router())
        .merge(crate::projects::report_router())
        .merge(crate::oauth::portal_router())
        .route_layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), crate::auth::require_user));

    let reads = Router::new()
        .route("/api/stats", get(stats::stats))
        .route("/api/stats/library", get(stats::library_stats))
        .route("/api/duplicates", get(assets::duplicates))
        .route("/api/packs", get(packs::list_packs))
        .route("/api/packs/{id}", get(packs::get_pack))
        .route("/api/packs/{id}/context", get(packs::get_pack_context))
        .route("/api/assets", get(assets::search_assets))
        .route("/api/assets/{id}", get(assets::get_asset))
        .route("/api/assets/{id}/file", get(files::asset_file))
        .route("/api/assets/{id}/thumb", get(files::asset_thumb))
        .route("/api/assets/{id}/lite", get(crate::previews::asset_lite))
        .route("/api/assets/{id}/frames.png", get(crate::previews::asset_frames))
        .route("/api/assets/{id}/lottie.json", get(crate::lottie::asset_lottie))
        .route("/api/blobs/{hash}", get(files::blob))
        .route("/api/tags", get(assets::tags))
        .route("/api/manifest", post(bundles::manifest))
        .route("/api/credits", post(crate::credits::credits))
        .route("/api/check", post(crate::check::check_handler))
        .route(
            "/api/imitation",
            post(crate::imitation::heard_route).layer(DefaultBodyLimit::max(crate::imitation::MAX_RECORDING_BYTES)),
        )
        .route("/api/packs/{id}/credits", get(crate::credits::pack_credits))
        .route("/api/collections/{id}/credits", get(crate::credits::collection_credits))
        .route("/api/assets/{id}/bundle.zip", get(bundles::asset_bundle))
        .route("/api/bundle.zip", get(bundles::selection_bundle))
        .route("/api/collections", get(collections::list_collections))
        .route("/api/collections/{id}", get(collections::get_collection))
        .route("/api/collections/{id}/bundle.zip", get(bundles::collection_bundle))
        .route("/api/assets/{id}/derivative/{kind}", get(files::asset_derivative))
        .route("/api/packs/{id}/bundle.zip", get(bundles::pack_bundle))
        .route("/api/packs/{id}/files/{*path}", get(files::pack_file))
        .route("/api/packs/{id}/unreal", get(convert::unreal_status))
        .route("/api/unreal/pending", get(convert::unreal_pending))
        .route("/api/assets/{id}/original", get(files::asset_original))
        .route("/api/assets/{id}/as/{format}", get(convert::asset_as))
        .route("/api/assets/{id}/as/{format}/status", get(convert::asset_as_status))
        .route("/api/activity", get(stats::activity))
        .merge(crate::projects::read_router())
        .route_layer(middleware::from_fn_with_state(std::sync::Arc::clone(state), crate::auth::require_reader));

    // Open: health, signing in, ChatGPT's sign-in redirect, OAuth for MCP apps, MCP (which checks for itself), and
    // Hoard's certificate authority (to install on a phone before signing in).
    Router::new()
        .route("/api/health", get(|| async { Json(json!({ "ok": true })) }))
        .route("/auth/callback", get(crate::chatgpt::callback))
        .merge(crate::auth::router())
        .merge(crate::oauth::router())
        .merge(crate::signin::router())
        .merge(crate::https::router())
        .merge(crate::mcp::router(state))
        .merge(reads)
        .merge(edits)
        .merge(admin)
        .merge(own)
}

/// Holds an upload to the "Largest upload" setting as it is now: refused up front when it says it's bigger, and
/// cut off if it turns out to be.
async fn limit_upload(State(state): State<AppState>, req: Request, next: Next) -> Response {
    let limits = state.settings.get();
    let max = limits.max_upload_bytes();
    let declared = req.headers().get(header::CONTENT_LENGTH).and_then(|v| v.to_str().ok()?.parse::<u64>().ok());
    if declared.is_some_and(|n| n > max) {
        let msg = format!("this upload is over the {} MiB limit (raise it in Settings)", limits.max_upload_mb);
        return (StatusCode::PAYLOAD_TOO_LARGE, msg).into_response();
    }
    let (parts, body) = req.into_parts();
    let body = Body::new(http_body_util::Limited::new(body, usize::try_from(max).unwrap_or(usize::MAX)));
    next.run(Request::from_parts(parts, body)).await
}
