//! Every operation the HTTP API offers, each declared once: the route the server answers, who may call it, what it
//! does and what it takes. The router is built from it (`api::router`), MCP offers it to assistants
//! (`find_operations`, `describe_operation`, `read_operation`, `run_operation`) and `/api/openapi.json` describes it,
//! so a route can't be added without assistants finding and calling it with the same sign-in, and what it takes is
//! read off its handler's own types, so the description can't drift from what the handler reads.
//!
//! A module offers its operations from a function beside its handlers, like
//! `Editor.put("/api/tags/parents", set_tag_parents).named("set_tag_parents", "…")`.

use std::sync::LazyLock;

use axum::extract::{Extension, Json, Multipart, Path, Query, Request, State};
use axum::handler::Handler;
use axum::routing::{MethodFilter, MethodRouter};
use http::{HeaderMap, Method};
use schemars::generate::SchemaSettings;
use schemars::{JsonSchema, SchemaGenerator};
use serde::Serialize;
use serde_json::{Value, json};

use crate::AppState;
use crate::auth::Role;

/// Who may call an operation, checked before it runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum Access {
    /// Anyone signed in, or anyone at all when Settings lets guests browse.
    Reader,
    /// Anyone signed in, for what's their own: favorites, saved searches, password, tokens and apps.
    SignedIn,
    /// Accounts and tokens that may change the library.
    Editor,
    /// Admins: Settings, accounts, maintenance and deleting for good.
    Admin,
}

impl Access {
    /// The least an account or token must be able to do to call it.
    pub(crate) const fn role(self) -> Role {
        match self {
            Self::Reader | Self::SignedIn => Role::View,
            Self::Editor => Role::Edit,
            Self::Admin => Role::Admin,
        }
    }

    pub(crate) fn get<H: Handler<T, AppState> + Inputs<A>, T: 'static, A>(self, path: &'static str, h: H) -> Unnamed {
        self.on(Method::GET, MethodFilter::GET, path, h)
    }

    pub(crate) fn post<H: Handler<T, AppState> + Inputs<A>, T: 'static, A>(self, path: &'static str, h: H) -> Unnamed {
        self.on(Method::POST, MethodFilter::POST, path, h)
    }

    pub(crate) fn put<H: Handler<T, AppState> + Inputs<A>, T: 'static, A>(self, path: &'static str, h: H) -> Unnamed {
        self.on(Method::PUT, MethodFilter::PUT, path, h)
    }

    pub(crate) fn patch<H: Handler<T, AppState> + Inputs<A>, T: 'static, A>(self, path: &'static str, h: H) -> Unnamed {
        self.on(Method::PATCH, MethodFilter::PATCH, path, h)
    }

    pub(crate) fn delete<H: Handler<T, AppState> + Inputs<A>, T: 'static, A>(
        self,
        path: &'static str,
        h: H,
    ) -> Unnamed {
        self.on(Method::DELETE, MethodFilter::DELETE, path, h)
    }

    fn on<H: Handler<T, AppState> + Inputs<A>, T: 'static, A>(
        self,
        method: Method,
        filter: MethodFilter,
        path: &'static str,
        h: H,
    ) -> Unnamed {
        Unnamed { access: self, method, path, input: H::input, route: axum::routing::on(filter, h) }
    }
}

/// An operation before it's said what it is: see [`Unnamed::named`].
#[derive(Debug)]
pub(crate) struct Unnamed {
    access: Access,
    method: Method,
    path: &'static str,
    input: fn(&mut SchemaGenerator) -> Input,
    route: MethodRouter<AppState>,
}

impl Unnamed {
    /// What it's called (snake case, unique) and what it does, in a sentence or a few: for an assistant deciding
    /// whether it's the one, and how to call it beyond what its inputs say.
    pub(crate) fn named(self, name: &'static str, summary: &'static str) -> Operation {
        Operation {
            name,
            summary,
            access: self.access,
            method: self.method,
            path: self.path,
            input: self.input,
            route: self.route,
            limit: Limit::Usual,
            for_people: None,
        }
    }
}

/// How big a request an operation takes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Limit {
    /// axum's own (2 MB).
    Usual,
    /// Up to this many bytes.
    Bytes(usize),
    /// Up to Settings' "Largest upload", as it is when the request comes.
    Upload,
}

/// One thing the API does. See the module docs.
#[derive(Debug)]
pub(crate) struct Operation {
    pub(crate) name: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) access: Access,
    pub(crate) method: Method,
    pub(crate) path: &'static str,
    pub(crate) limit: Limit,
    /// Why assistants aren't offered it, when it's for a person to do themselves.
    pub(crate) for_people: Option<&'static str>,
    input: fn(&mut SchemaGenerator) -> Input,
    route: MethodRouter<AppState>,
}

impl Operation {
    /// Takes up to `bytes` (instead of axum's 2 MB).
    pub(crate) const fn limit(mut self, bytes: usize) -> Self {
        self.limit = Limit::Bytes(bytes);
        self
    }

    /// Takes up to Settings' "Largest upload".
    pub(crate) const fn upload(mut self) -> Self {
        self.limit = Limit::Upload;
        self
    }

    /// Not offered to assistants, for `why`: it's for a person to do themselves (a password, a token, letting an app
    /// in). It's still in `/api/openapi.json`, and still works for an app that calls the HTTP API itself.
    pub(crate) const fn for_people(mut self, why: &'static str) -> Self {
        self.for_people = Some(why);
        self
    }

    /// Its route, for the router.
    pub(crate) fn into_route(self) -> MethodRouter<AppState> {
        match self.limit {
            Limit::Usual | Limit::Upload => self.route,
            Limit::Bytes(n) => self.route.layer(axum::extract::DefaultBodyLimit::max(n)),
        }
    }

    fn describe(&self, generator: &mut SchemaGenerator) -> Described {
        let input = (self.input)(generator);
        Described {
            name: self.name,
            summary: self.summary,
            access: self.access,
            method: match self.method {
                Method::POST => "POST",
                Method::PUT => "PUT",
                Method::PATCH => "PATCH",
                Method::DELETE => "DELETE",
                _ => "GET",
            },
            path: self.path,
            for_people: self.for_people,
            path_params: path_params(self.path),
            query: input.query,
            body: input.body,
        }
    }
}

/// An operation as assistants and `/api/openapi.json` see it.
#[derive(Clone, Debug, Serialize)]
pub(crate) struct Described {
    pub(crate) name: &'static str,
    pub(crate) summary: &'static str,
    pub(crate) access: Access,
    pub(crate) method: &'static str,
    pub(crate) path: &'static str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) for_people: Option<&'static str>,
    /// The `{names}` in its path, in order. A `{*name}` one takes the rest of the path, slashes and all.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) path_params: Vec<&'static str>,
    /// What its query string takes: a JSON Schema object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) query: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) body: Option<Body>,
}

impl Described {
    /// Whether calling it changes anything.
    pub(crate) fn changes(&self) -> bool {
        self.method != "GET"
    }

    /// What `who` would be told if it called this, or `None` if it may.
    pub(crate) fn refused(&self, role: Role) -> Option<String> {
        if let Some(why) = self.for_people {
            return Some(format!("`{}` is for a person to do in Hoard themselves: {why}", self.name));
        }
        (role < self.access.role())
            .then(|| format!("`{}` needs an account or token that can {}", self.name, self.access.role().as_str()))
    }
}

/// What an operation's request body is.
#[derive(Clone, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub(crate) enum Body {
    /// JSON, as `schema` says. Sometimes it may be left out.
    Json { schema: Value, required: bool },
    /// A file's bytes, as they are.
    File,
    /// A multipart form of files.
    Form,
}

/// What an operation takes besides its path: read off its handler's arguments (see [`Part`]).
#[derive(Clone, Debug, Default)]
pub(crate) struct Input {
    query: Option<Value>,
    body: Option<Body>,
}

/// One of a handler's arguments, as far as what a request must carry goes.
pub(crate) trait Part {
    fn add(_input: &mut Input, _generator: &mut SchemaGenerator) {}
}

impl<S> Part for State<S> {}
impl<T> Part for Path<T> {}
impl<T> Part for Extension<T> {}
impl Part for HeaderMap {}
impl Part for Method {}

impl<T: JsonSchema> Part for Query<T> {
    fn add(input: &mut Input, generator: &mut SchemaGenerator) {
        input.query = Some(generator.subschema_for::<T>().to_value());
    }
}

impl<T: JsonSchema> Part for Json<T> {
    fn add(input: &mut Input, generator: &mut SchemaGenerator) {
        input.body = Some(Body::Json { schema: generator.subschema_for::<T>().to_value(), required: true });
    }
}

impl<T: JsonSchema> Part for Option<Json<T>> {
    fn add(input: &mut Input, generator: &mut SchemaGenerator) {
        input.body = Some(Body::Json { schema: generator.subschema_for::<T>().to_value(), required: false });
    }
}

impl<T: JsonSchema, E> Part for Result<Json<T>, E> {
    fn add(input: &mut Input, generator: &mut SchemaGenerator) {
        input.body = Some(Body::Json { schema: generator.subschema_for::<T>().to_value(), required: true });
    }
}

impl Part for Request {
    fn add(input: &mut Input, _: &mut SchemaGenerator) {
        input.body = Some(Body::File);
    }
}

impl Part for axum::body::Body {
    fn add(input: &mut Input, _: &mut SchemaGenerator) {
        input.body = Some(Body::File);
    }
}

impl Part for axum::body::Bytes {
    fn add(input: &mut Input, _: &mut SchemaGenerator) {
        input.body = Some(Body::File);
    }
}

impl Part for Multipart {
    fn add(input: &mut Input, _: &mut SchemaGenerator) {
        input.body = Some(Body::Form);
    }
}

/// A handler whose arguments are all [`Part`]s: what a request to it must carry.
pub(crate) trait Inputs<A> {
    fn input(generator: &mut SchemaGenerator) -> Input;
}

macro_rules! inputs {
    ($($t:ident),*) => {
        impl<F, Fut, $($t: Part,)*> Inputs<($($t,)*)> for F
        where
            F: FnOnce($($t),*) -> Fut,
        {
            fn input(generator: &mut SchemaGenerator) -> Input {
                let parts: &[fn(&mut Input, &mut SchemaGenerator)] = &[$($t::add),*];
                let mut input = Input::default();
                for add in parts {
                    add(&mut input, generator);
                }
                input
            }
        }
    };
}

inputs!();
inputs!(T1);
inputs!(T1, T2);
inputs!(T1, T2, T3);
inputs!(T1, T2, T3, T4);
inputs!(T1, T2, T3, T4, T5);
inputs!(T1, T2, T3, T4, T5, T6);
inputs!(T1, T2, T3, T4, T5, T6, T7);

/// The `{names}` in a route's path.
fn path_params(path: &'static str) -> Vec<&'static str> {
    path.split('/').filter_map(|s| s.strip_prefix('{')?.strip_suffix('}')).map(|s| s.trim_start_matches('*')).collect()
}

/// Every operation, described, in the order the API lists them.
pub(crate) static CATALOG: LazyLock<Vec<Described>> = LazyLock::new(|| {
    let mut generator =
        SchemaSettings::draft2020_12().with(|s| s.inline_subschemas = true).for_deserialize().into_generator();
    crate::api::operations().iter().map(|op| op.describe(&mut generator)).collect()
});

/// The operation called `name`.
pub(crate) fn find(name: &str) -> Option<&'static Described> {
    CATALOG.iter().find(|d| d.name == name)
}

/// The catalog as an `OpenAPI` 3.1 document, for apps and agents that call the HTTP API themselves.
pub(crate) fn openapi(base: &str) -> Value {
    let mut paths = serde_json::Map::new();
    for d in CATALOG.iter() {
        let route = d.path.replace("{*", "{");
        let mut parameters: Vec<Value> = d
            .path_params
            .iter()
            .map(|p| json!({ "name": p, "in": "path", "required": true, "schema": { "type": "string" } }))
            .collect();
        if let Some(query) = &d.query {
            let required: Vec<&str> =
                query["required"].as_array().into_iter().flatten().filter_map(Value::as_str).collect();
            for (name, schema) in query["properties"].as_object().into_iter().flatten() {
                let mut p = json!({ "name": name, "in": "query", "required": required.contains(&name.as_str()), "schema": schema });
                if let Some(text) = schema.get("description") {
                    p["description"] = text.clone();
                }
                parameters.push(p);
            }
        }
        let mut op = json!({
            "operationId": d.name,
            "summary": first_sentence(d.summary),
            "description": d.summary,
            "x-hoard-access": d.access,
            "responses": { "200": { "description": "Done" }, "default": { "description": "What went wrong, as `{\"error\": \"…\"}`" } },
        });
        if !parameters.is_empty() {
            op["parameters"] = json!(parameters);
        }
        if let Some(why) = d.for_people {
            op["x-hoard-for-people"] = json!(why);
        }
        match &d.body {
            Some(Body::Json { schema, required }) => {
                op["requestBody"] =
                    json!({ "required": required, "content": { "application/json": { "schema": schema } } });
            }
            Some(Body::File) => {
                op["requestBody"] = json!({ "required": true, "content": { "application/octet-stream": {} } });
            }
            Some(Body::Form) => {
                op["requestBody"] = json!({ "required": true, "content": { "multipart/form-data": {} } });
            }
            None => {}
        }
        let entry = paths.entry(route).or_insert_with(|| json!({}));
        entry[d.method.to_lowercase()] = op;
    }
    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "Hoard",
            "version": hoard_core::VERSION,
            "description": "Hoard's HTTP API: everything the portal does. Send an access token from Settings → Account \
                            as `Authorization: Bearer <token>`; what it may call is what its account may do.",
        },
        "servers": [{ "url": base }],
        "components": { "securitySchemes": { "token": { "type": "http", "scheme": "bearer" } } },
        "security": [{ "token": [] }],
        "paths": paths,
    })
}

/// A summary's first sentence, for a one-line title.
pub(crate) fn first_sentence(summary: &str) -> &str {
    summary.find(". ").map_or(summary, |i| &summary[..=i])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_operation_has_a_name_of_its_own_and_says_what_it_does() {
        let mut seen = std::collections::HashSet::new();
        for d in CATALOG.iter() {
            assert!(seen.insert(d.name), "two operations are called {}", d.name);
            assert!(
                !d.name.is_empty() && d.name.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'),
                "{} isn't snake case",
                d.name
            );
            assert!(d.summary.ends_with('.') || d.summary.ends_with(')'), "{} doesn't end its sentence", d.name);
            assert!(d.summary.len() >= 20, "{} says too little", d.name);
            assert!(d.path.starts_with("/api/"), "{} isn't in the API", d.name);
        }
        let mut routes = std::collections::HashSet::new();
        for d in CATALOG.iter() {
            assert!(routes.insert((d.method, d.path)), "{} {} is there twice", d.method, d.path);
        }
    }

    #[test]
    fn what_an_operation_takes_is_read_off_its_handler() {
        let parents = find("set_tag_parents").unwrap();
        let Some(Body::Json { schema, required: true }) = &parents.body else { panic!("{:?}", parents.body) };
        assert!(schema["properties"].get("tag").is_some(), "{schema}");
        let asset = find("get_asset").unwrap();
        assert_eq!(asset.path_params, ["id"]);
        assert!(asset.body.is_none() && !asset.changes());
        let search = find("search_assets").unwrap();
        assert!(search.query.as_ref().unwrap()["properties"].get("q").is_some());
    }

    /// Routes are only added through the catalog, apart from signing in and what's open to everyone before then:
    /// any other API route added with `.route` is one assistants couldn't find.
    #[test]
    fn every_route_is_in_the_catalog() {
        const OPEN: &[&str] = &[
            "/api/health",
            "/api/site/{token}/",
            "/api/site/{token}/{*path}",
            "/api/auth/setup",
            "/api/auth/login",
            "/api/auth/logout",
            "/api/auth/status",
            "/api/auth/oauth/start",
            "/api/https/ca.crt",
            "/api/desktop/launch",
        ];
        let src = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src");
        let mut stack = vec![src];
        let mut stray = Vec::new();
        while let Some(dir) = stack.pop() {
            for entry in std::fs::read_dir(&dir).unwrap() {
                let path = entry.unwrap().path();
                if path.is_dir() {
                    stack.push(path);
                    continue;
                }
                if path.extension().is_none_or(|e| e != "rs") {
                    continue;
                }
                let text = std::fs::read_to_string(&path).unwrap();
                for (i, _) in text.match_indices(".route(") {
                    let rest = text[i + ".route(".len()..].trim_start();
                    let Some(route) = rest.strip_prefix('"').and_then(|r| r.split('"').next()) else { continue };
                    if route.starts_with("/api/") && !OPEN.contains(&route) {
                        stray.push(format!("{}: {route}", path.display()));
                    }
                }
            }
        }
        assert!(stray.is_empty(), "routes outside the catalog (add them to an `operations()` instead): {stray:#?}");
    }

    #[test]
    fn the_openapi_document_has_every_operation() {
        let doc = openapi("https://hoard.example");
        let n: usize = doc["paths"].as_object().unwrap().values().map(|p| p.as_object().unwrap().len()).sum();
        assert_eq!(n, CATALOG.len());
        let put = &doc["paths"]["/api/tags/parents"]["put"];
        assert_eq!(put["operationId"], "set_tag_parents");
        assert!(put["requestBody"]["content"]["application/json"]["schema"]["properties"]["tag"].is_object());
        let pack_file = &doc["paths"]["/api/packs/{id}/files/{path}"]["get"];
        assert_eq!(pack_file["parameters"][1]["name"], "path");
    }
}
