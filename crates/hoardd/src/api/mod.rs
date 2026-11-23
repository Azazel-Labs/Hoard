//! HTTP API. Every route but signing in and what's open before then is an operation of the catalog
//! ([`crate::catalog`]): declared once, in [`operations`], with who may call it, and the router is built from that.
//!
//! The handlers are in a module per thing they serve.

use axum::body::Body;
use axum::extract::{DefaultBodyLimit, Request, State};
use axum::http::{HeaderMap, StatusCode, header};
use axum::middleware::{self, Next};
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::{Json, Router};
use serde_json::{Value, json};

use std::sync::Arc;

use crate::AppState;
use crate::catalog::{Access, Limit, Operation};
use crate::locations;

mod admin;
mod assets;
mod bundles;
mod collections;
mod convert;
mod files;
mod github;
mod packs;
mod settings;
mod site;
mod stats;

pub(crate) use files::serve_blob;
pub(crate) use github::keep_fresh as keep_github_fresh;
pub(crate) use packs::{default_pack_name, pack_name_from_files};
pub(crate) use stats::keep_reports;

/// Every operation of the API, in the order they're listed: the library's, then administering it, then what's each
/// person's own.
#[expect(clippy::too_many_lines, reason = "a list, one operation after another")]
pub(crate) fn operations() -> Vec<Operation> {
    use Access::{Admin, Editor, Reader};
    let mut ops = vec![
        // Packs.
        Reader.get("/api/packs", packs::list_packs).named(
            "list_packs",
            "Every pack in the library (with `archived`, in the archive instead): name, source, status, how many \
             assets and how big. `busy` lists only those being uploaded, read or analyzed.",
        ),
        Reader.get("/api/packs/{id}", packs::get_pack).named(
            "get_pack",
            "One pack: its details, where its files came from and what's been worked out about it.",
        ),
        Editor
            .post("/api/packs", packs::upload_pack)
            .named(
                "upload_pack",
                "Send a pack in one request: a multipart form with optional `name` and `source` text fields, then for \
                 each file an optional `relpath` text field and a `file`. Archives are unpacked. For a pack too big \
                 for one request, see `create_upload`.",
            )
            .upload(),
        Editor.patch("/api/packs/{id}", packs::patch_pack).named(
            "change_pack",
            "Rename a pack, or turn its models to face another way (`facing`). Everything refers to packs by id, so \
             searches, links and collections carry on as they were.",
        ),
        Admin.delete("/api/packs/{id}", packs::delete_pack).named(
            "delete_pack",
            "Delete a pack and its assets for good, with no archive to bring them back from (`archive` keeps them \
             until purged).",
        ),
        Reader.get("/api/packs/{id}/context", packs::get_pack_context).named(
            "get_pack_context",
            "What's known about a pack beyond its files, which the AI tagger reads: what its seller says, its store \
             page and its readme.",
        ),
        Editor.put("/api/packs/{id}/context", packs::put_pack_context).named(
            "set_pack_context",
            "Say what a pack's seller says about it (its store page and text), for the AI tagger to read. A store \
             address without text fetches the page's title and description.",
        ),
        Editor.post("/api/packs/{id}/retag", packs::retag_pack).named(
            "retag_pack",
            "Tag a pack's assets again, by the rules (`rules`, unless false) and with `llm`, by the AI tagger too. A \
             person's own tags stay. Runs as a job.",
        ),
        Editor.post("/api/packs/{id}/read-files", packs::read_pack_files).named(
            "read_pack_store_files",
            "Read again the files a store put in a pack (a Megascans JSON, say) for its name, tags and description.",
        ),
        Editor.get("/api/packs/{id}/store-matches", crate::store_search::store_matches).named(
            "find_store_pages",
            "Likely store pages for a pack, best first. `q` searches for something other than the pack's name; \
             without `q` or `fresh`, what the last lookup found is given.",
        ),
        Editor.post("/api/packs/{id}/store/undo", packs::undo_store_page).named(
            "undo_store_page",
            "Take off a store page that was found for a pack automatically. The pack isn't looked up again.",
        ),
        Reader.get("/api/packs/{id}/credits", crate::credits::pack_credits).named(
            "get_pack_credits",
            "One pack's credits (who made it, where from, its license), as JSON or with `format=md` as Markdown.",
        ),
        Reader
            .get("/api/packs/{id}/bundle.zip", bundles::pack_bundle)
            .named("download_pack", "A whole pack as one zip, in its original layout."),
        Reader.get("/api/packs/{id}/files/{*path}", files::pack_file).named(
            "get_pack_file",
            "One of a pack's files by its path in the pack, which is what a model's references to its textures lead to.",
        ),
        // Assets.
        Reader.get("/api/assets", assets::search_assets).named(
            "search_assets",
            "Search the library, a page at a time (`limit`, `offset`): `q` is what the search box takes, and `kind`, \
             `tags` (comma-separated, all must match), `pack`, `ext`, `collection` and `project` narrow it. \
             `facets` also counts what's found by kind, tag and pack. `archived` searches the archive.",
        ),
        Reader.get("/api/assets/{id}", assets::get_asset).named(
            "get_asset",
            "One asset: its file, kind, tags, measurements and details, its pack, and what goes with it.",
        ),
        Editor.patch("/api/assets/{id}", assets::patch_asset).named(
            "change_asset",
            "Change an asset: its name, category, description, license, rating, fields, and tags to add, take off or \
             keep. What's left out stays; an empty text clears. Its history records it, for `revert_change`.",
        ),
        Editor.post("/api/assets/bulk", assets::bulk_assets).named(
            "change_assets",
            "The same change as `change_asset` takes, to many assets at once: those in `ids`, or everything `query` \
             (a search) finds, however many pages of it.",
        ),
        Reader.get("/api/assets/{id}/history", assets::asset_history).named(
            "get_asset_history",
            "An asset's history: what was changed, from what to what, by whom and how, the latest first.",
        ),
        Editor.post("/api/assets/{id}/history/{change}/revert", assets::revert_change).named(
            "revert_change",
            "Put back what one change in an asset's history changed (`change` is its id there).",
        ),
        Editor.put("/api/assets/{id}/skeleton-body", assets::choose_skeleton_body).named(
            "choose_skeleton_body",
            "Make this character the one its skeleton's animations play on, wherever they're shown.",
        ),
        Editor
            .put("/api/assets/{id}/thumb", crate::previews::put_thumb)
            .named(
                "set_asset_thumbnail",
                "A model's thumbnail as a browser drew it (an image, as the body), scaled to size. Kept unless the one \
                 there came from something better.",
            )
            .limit(crate::previews::MAX_DRAWN_BYTES),
        Editor
            .post("/api/assets/{id}/replace", convert::replace_asset)
            .named(
                "replace_asset_file",
                "Swap an asset's file for a converted one, its bytes as the body (where `hoard unreal-worker` sends \
                 what the Unreal editor exports).",
            )
            .upload(),
        Admin.post("/api/assets/{id}/unconvert", convert::unconvert_asset).named(
            "unconvert_asset",
            "Put back the file a conversion rule replaced, while its original is kept.",
        ),
        Reader
            .get("/api/assets/{id}/file", files::asset_file)
            .named("get_asset_file", "An asset's file, as the library has it."),
        Reader
            .get("/api/assets/{id}/thumb", files::asset_thumb)
            .named("get_asset_thumbnail", "An asset's thumbnail image."),
        Reader.get("/api/assets/{id}/lite", crate::previews::asset_lite).named(
            "get_asset_preview",
            "An image's light preview: the image itself when it's small enough, a smaller copy when not.",
        ),
        Reader.get("/api/assets/{id}/frames.png", crate::previews::asset_frames).named(
            "get_asset_frames",
            "An Aseprite file's frames side by side, as many rows as columns, each the asset's `width`×`height`, to \
             play with its `frame_ms`.",
        ),
        Reader
            .get("/api/assets/{id}/lottie.json", crate::lottie::asset_lottie)
            .named("get_asset_lottie", "A Lottie animation's JSON, to play."),
        Reader.get("/api/assets/{id}/derivative/{kind}", files::asset_derivative).named(
            "get_asset_derivative",
            "A file made from an asset, by its kind (a model's preview, a sound's waveform, and so on).",
        ),
        Reader.get("/api/assets/{id}/original", files::asset_original).named(
            "get_asset_original",
            "The original file of an asset a conversion rule replaced, when it was kept.",
        ),
        Reader.get("/api/assets/{id}/site", site::asset_site).named(
            "get_asset_site",
            "Where to open a web page in a pack so that it shows as it would from its folder.",
        ),
        Reader.get("/api/assets/{id}/as/{format}", convert::asset_as).named(
            "convert_asset",
            "An asset converted to another format (images, audio, models, fonts), made on first request and kept. \
             Waits up to `wait` seconds for it, then answers 202 to ask again.",
        ),
        Reader.get("/api/assets/{id}/as/{format}/status", convert::asset_as_status).named(
            "get_conversion",
            "Start or look in on a conversion without downloading it: `state` is ready, queued, converting or failed. \
             `wait` answers as soon as anything changes, or after that many seconds (at most 10).",
        ),
        Reader.get("/api/assets/{id}/bundle.zip", bundles::asset_bundle).named(
            "download_asset",
            "An asset and everything it needs (a model's textures, say), zipped.",
        ),
        Reader.get("/api/bundle.zip", bundles::selection_bundle).named(
            "download_assets",
            "Several assets (`ids`, comma-separated) and what they need, in one zip with a folder per pack.",
        ),
        Reader.get("/api/blobs/{hash}", files::blob).named("get_blob", "A stored file, by its content hash."),
        Reader.get("/api/duplicates", assets::duplicates).named(
            "find_duplicates",
            "The library's sets of near copies, those whose extra copies take the most space first.",
        ),
        Reader.post("/api/imitation", crate::imitation::heard_route)
            .named(
                "read_imitation",
                "A recording of someone making a noise (WAV, as the body), read as the shape of a sound: what \
                 `search_assets` takes as `imitation` to find sounds shaped like it.",
            )
            .limit(crate::imitation::MAX_RECORDING_BYTES),
        // Tags.
        Reader.get("/api/tags", assets::tags).named(
            "list_tags",
            "The tags in use and how many assets have each, the most used first. `prefix` narrows them (anywhere in \
             the tag, with `anywhere`).",
        ),
        Reader.get("/api/tags/suggest", assets::suggest_tags).named(
            "suggest_tags",
            "Tags to offer while someone types one: matches (a merged tag's old name leads to what it became), and \
             tags in use a new one is probably meant to be.",
        ),
        Reader.get("/api/tags/info", assets::tag_info).named(
            "get_tag",
            "What a tag's page shows: how many have it, what it means, its picture and its old names.",
        ),
        Editor.put("/api/tags/info", assets::set_tag_info).named(
            "set_tag_info",
            "Say what a tag means, or pick the asset whose picture stands for it. What's left out stays; empty clears it.",
        ),
        Reader.get("/api/tags/tree", assets::tag_tree).named(
            "get_tag_tree",
            "Every tag that goes under another, as (tag, parent) pairs. Searching for a tag finds those under it too.",
        ),
        Editor.put("/api/tags/parents", assets::set_tag_parents).named(
            "set_tag_parents",
            "Put a tag under broader ones, in place of those it was under (none takes it out from under them all). \
             Searching for one of them then finds it too.",
        ),
        Editor.post("/api/tags/merge", assets::merge_tags).named(
            "merge_tags",
            "Merge tags (`from`) into one (`into`), or rename one. The old names are kept as aliases, so taggers give \
             the new one instead.",
        ),
        Editor
            .get("/api/tags/aliases", assets::tag_aliases)
            .named("list_tag_aliases", "The old names of merged tags, and what each became."),
        Editor.delete("/api/tags/aliases", assets::drop_tag_alias).named(
            "drop_tag_alias",
            "Stop mapping a merged tag's old name to what it became: the taggers may give it again.",
        ),
        Reader.get("/api/categories", assets::categories).named(
            "list_categories",
            "The categories in the library, the most used first.",
        ),
        // Collections.
        Reader
            .get("/api/collections", collections::list_collections)
            .named("list_collections", "Every collection, with how many assets each has."),
        Reader
            .get("/api/collections/{id}", collections::get_collection)
            .named("get_collection", "One collection and the assets in it."),
        Editor
            .post("/api/collections", collections::create_collection)
            .named("create_collection", "Make a collection: a named set of assets from any packs."),
        Editor
            .patch("/api/collections/{id}", collections::patch_collection)
            .named("change_collection", "Rename a collection, or change what it says about itself."),
        Editor.delete("/api/collections/{id}", collections::delete_collection).named(
            "delete_collection",
            "Delete a collection. The assets in it stay in the library.",
        ),
        Editor
            .post("/api/collections/{id}/assets", collections::collection_members)
            .named("change_collection_assets", "Put assets in a collection, or take them out."),
        Reader.get("/api/collections/{id}/credits", crate::credits::collection_credits).named(
            "get_collection_credits",
            "Credits for the packs a collection's assets come from, as JSON or with `format=md` as Markdown.",
        ),
        Reader
            .get("/api/collections/{id}/bundle.zip", bundles::collection_bundle)
            .named("download_collection", "A collection's assets and what they need, in one zip."),
        // The archive.
        Editor.post("/api/archive", assets::archive).named(
            "archive",
            "Move packs and assets to the archive: out of the library and its searches, kept until restored or \
             purged.",
        ),
        Editor
            .post("/api/archive/restore", assets::restore)
            .named("restore", "Bring packs and assets back from the archive into the library."),
        Admin.post("/api/archive/purge", assets::purge).named(
            "purge",
            "Delete for good what's in the archive: the packs and assets listed, or with `all`, everything. Bytes \
             nothing else uses are freed.",
        ),
        // Games and credits.
        Reader.post("/api/manifest", bundles::manifest).named(
            "get_manifest",
            "Turn a selection (ids and/or a search) into the list of files a game engine fetches.",
        ),
        Reader.post("/api/credits", crate::credits::credits).named(
            "get_credits",
            "Credits for packs, collections, assets and selections together, as JSON or with `format=md` as Markdown.",
        ),
        Reader.post("/api/check", crate::check::check_handler).named(
            "check_project",
            "Check a project's files by their hashes: which come from the library, and their credits. `format=md` \
             answers with a CREDITS.md that says what needs a look.",
        ),
        // Unreal.
        Reader.get("/api/packs/{id}/unreal", convert::unreal_status).named(
            "get_unreal_conversion",
            "How a pack's conversion through the Unreal editor stands.",
        ),
        Reader.get("/api/unreal/pending", convert::unreal_pending).named(
            "list_unreal_pending",
            "The packs waiting for the Unreal editor to convert them, which `hoard unreal-worker` asks for.",
        ),
        Editor.post("/api/packs/{id}/unreal/convert", convert::unreal_convert).named(
            "convert_unreal_pack",
            "Ask for a pack's Unreal files (.uasset) to be converted through the Unreal editor, by a computer running \
             `hoard unreal-worker`.",
        ),
        Editor.post("/api/packs/{id}/unreal/finish", convert::unreal_finish).named(
            "finish_unreal_pack",
            "After a batch of converted files: retag the pack, draw previews again and free the replaced originals.",
        ),
        // The library as a whole.
        Reader.get("/api/stats", stats::stats).named(
            "get_stats",
            "How big the library is: packs, assets and bytes, and the space conversions take.",
        ),
        Reader.get("/api/stats/library", stats::library_stats).named(
            "get_library_report",
            "What's in the library, by kind, format, license, source and tag: the Library page's charts.",
        ),
        Reader.get("/api/activity", stats::activity).named(
            "get_activity",
            "What the server is doing in the background: running work (with progress where it's known), then jobs \
             waiting their turn.",
        ),
        Reader.get("/api/openapi.json", openapi).named(
            "get_openapi",
            "This API as an OpenAPI 3.1 document: every operation, what it takes and who may call it.",
        ),
        // Administering.
        Admin.get("/api/admin/jobs", admin::admin_jobs).named(
            "list_jobs",
            "The last 50 background jobs: kind, pack, state, attempts and what went wrong.",
        ),
        Admin.get("/api/admin/maintenance", crate::maintenance::overview).named(
            "list_maintenance",
            "The Maintenance tasks, what's running, each one's last run, and the recent history.",
        ),
        Admin
            .post("/api/admin/maintenance/{task}", crate::maintenance::start_route)
            .named("start_maintenance", "Start a Maintenance task (one `list_maintenance` gives) in the background."),
        Admin.post("/api/admin/scrub", admin::admin_scrub).named(
            "scrub",
            "Check every stored file against its hash, in the background: what's damaged is reported as an issue.",
        ),
        Admin
            .post("/api/admin/gc", admin::admin_gc)
            .named("collect_garbage", "Free the stored files nothing uses any more, now."),
        Admin.post("/api/admin/backup", admin::admin_backup).named(
            "back_up",
            "Back up the database now, as a consistent snapshot. Stored files never change, so any file sync backs \
             those up.",
        ),
        Admin.get("/api/admin/issues", crate::issues::list).named(
            "list_issues",
            "What's gone wrong, kind by kind, the most serious first: files that couldn't be read, failed jobs, \
             unreachable locations.",
        ),
        Admin.post("/api/admin/issues/retry", crate::issues::retry).named(
            "retry_issues",
            "Try again what went wrong, by `ids` or every one of a `kind`: a pack is analyzed again, a folder scanned \
             or a job run once, however many of its issues are picked.",
        ),
        Admin.post("/api/admin/issues/dismiss", crate::issues::dismiss).named(
            "dismiss_issues",
            "Forget issues, by `ids` or every one of a `kind`: only what Hoard recorded or a failed job. The rest goes \
             away when what's wrong is put right.",
        ),
        Admin.post("/api/admin/conversions/preview", convert::conversions_preview).named(
            "preview_conversions",
            "What a change to the conversion rules would do to the files already in the library, before it's saved.",
        ),
        Admin.post("/api/admin/conversions/run", convert::conversions_run).named(
            "run_conversions",
            "Convert the files already in the library that the saved conversion rules match.",
        ),
        Admin.get("/api/admin/update", crate::updates::status_handler).named(
            "get_update",
            "This build, a newer release if there is one, what's keeping the server busy, and the last call to the \
             update address.",
        ),
        Admin.post("/api/admin/update/hook", crate::updates::call_handler).named(
            "call_update_hook",
            "Call the update address now, to see that it's wired up. Whatever it's wired to runs.",
        ),
        Admin.get("/api/admin/locations", locations::list_locations).named(
            "list_locations",
            "Where the library keeps its files: each location, whether it can be reached, and what the setup suggests.",
        ),
        Admin.post("/api/admin/locations", locations::create_location).named(
            "create_location",
            "Add a location: a folder Hoard keeps files in (managed), or one it watches and reads packs from.",
        ),
        Admin.patch("/api/admin/locations/{id}", locations::patch_location).named(
            "change_location",
            "Rename a location, change the kinds of files it takes, or make it the catch-all. Where it is can't change.",
        ),
        Admin.delete("/api/admin/locations/{id}", locations::delete_location).named(
            "delete_location",
            "Remove a location: a watched one with its packs (the folder is left alone), a managed one once its files \
             are moved out (409 while it holds any).",
        ),
        Admin.post("/api/admin/locations/{id}/scan", locations::scan_location).named(
            "scan_location",
            "Look through a watched folder now rather than at the next rescan.",
        ),
        Admin.post("/api/admin/locations/{id}/empty", locations::empty_location).named(
            "empty_location",
            "Move a managed location's files to where they'd go without it, then remove it. Runs as a job.",
        ),
        Admin.post("/api/admin/locations/rebalance", locations::rebalance).named(
            "rebalance_locations",
            "Move stored files to the locations their kinds go to now. Runs as a job.",
        ),
        Admin.get("/api/admin/fs", locations::browse).named(
            "browse_server_folders",
            "The folders in a folder on the server (`path`), for picking a location. None lists the drives, or `/`.",
        ),
        Admin.get("/api/admin/chatgpt", crate::chatgpt::get_status).named(
            "get_chatgpt",
            "Whether the server is signed in to ChatGPT for tagging, and as whom.",
        ),
        Admin
            .post("/api/admin/chatgpt/start", crate::chatgpt::start)
            .named("start_chatgpt_sign_in", "The address to open to sign the server in to ChatGPT.")
            .for_people("signing in to ChatGPT is for a person to do in a browser"),
        Admin
            .post("/api/admin/chatgpt/finish", crate::chatgpt::finish)
            .named("finish_chatgpt_sign_in", "Finish signing the server in to ChatGPT with what the browser came back with.")
            .for_people("signing in to ChatGPT is for a person to do in a browser"),
        Admin
            .post("/api/admin/chatgpt/sign-out", crate::chatgpt::sign_out_route)
            .named("sign_out_of_chatgpt", "Sign the server out of ChatGPT.")
            .for_people("signing in to ChatGPT is for a person to do in a browser"),
        Admin.get("/api/admin/chatgpt/models", crate::chatgpt::list_models).named(
            "list_chatgpt_models",
            "The models the server's ChatGPT sign-in may use for tagging.",
        ),
        Admin.get("/api/admin/profile", crate::profiler::handler).named(
            "get_profile",
            "Where the server has spent its time in the last ten minutes, or since `after`.",
        ),
        Admin.get("/api/admin/report", admin::report).named(
            "get_problem_report",
            "What a GitHub issue needs for a problem to be looked into (the build, where it runs, what's on, and the \
             server's last log lines), with nothing private in it.",
        ),
    ];
    ops.extend(settings::operations());
    ops.extend(github::operations());
    ops.extend(crate::jobs::operations());
    ops.extend(crate::format_wizard::operations());
    ops.extend(crate::uploads::operations());
    ops.extend(crate::projects::operations());
    ops.extend(crate::https::operations());
    ops.extend(crate::signin::operations());
    ops.extend(crate::auth::operations());
    ops.extend(crate::oauth::operations());
    ops.extend(crate::me::operations());
    ops
}

/// `GET /api/openapi.json`.
async fn openapi(State(state): State<AppState>, headers: HeaderMap) -> Json<Value> {
    Json(crate::catalog::openapi(&crate::mcp::base_url(&state, &headers)))
}

pub(crate) fn router(state: &AppState) -> Router<AppState> {
    // The upload limit is a setting that can change while the server runs, so it's checked per request.
    let upload_limit = || {
        tower::ServiceBuilder::new()
            .layer(DefaultBodyLimit::disable())
            .layer(middleware::from_fn_with_state(Arc::clone(state), limit_upload))
    };
    let (mut reads, mut own, mut edits, mut admin) = (Router::new(), Router::new(), Router::new(), Router::new());
    for op in operations() {
        let group = match op.access {
            Access::Reader => &mut reads,
            Access::SignedIn => &mut own,
            Access::Editor => &mut edits,
            Access::Admin => &mut admin,
        };
        let (path, upload) = (op.path, op.limit == Limit::Upload);
        let route = if upload { op.into_route().layer(upload_limit()) } else { op.into_route() };
        *group = std::mem::take(group).route(path, route);
    }

    // Open: health, signing in, ChatGPT's sign-in redirect, OAuth for MCP apps, MCP (which checks for itself), and
    // Hoard's certificate authority (to install on a phone before signing in).
    Router::new()
        .route("/api/health", get(|| async { Json(json!({ "ok": true })) }))
        // A page in a pack and its files: the address is signed, since a sandboxed page sends no cookies.
        .route("/api/site/{token}/", get(site::site_root))
        .route("/api/site/{token}/{*path}", get(site::site_file))
        .route("/auth/callback", get(crate::chatgpt::callback))
        .merge(crate::auth::router())
        .merge(crate::oauth::router())
        .merge(crate::signin::router())
        .merge(crate::https::router())
        .merge(crate::mcp::router(state))
        .merge(reads.route_layer(middleware::from_fn_with_state(Arc::clone(state), crate::auth::require_reader)))
        .merge(edits.route_layer(middleware::from_fn_with_state(Arc::clone(state), crate::auth::require_editor)))
        .merge(admin.route_layer(middleware::from_fn_with_state(Arc::clone(state), crate::auth::require_admin)))
        .merge(own.route_layer(middleware::from_fn_with_state(Arc::clone(state), crate::auth::require_user)))
        // The first time each tool is used earns its achievement.
        .route_layer(middleware::from_fn(crate::achievements::watch))
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
