use crate::templates::{new_handlebar, Engine};
use anyhow::{anyhow, Result};
use axum::{
    body::Body,
    http::StatusCode,
    middleware,
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};
use axum_template::RenderHtml;
use land_vars::{AuthUser, BreadCrumbKey, Page};
use serde::Serialize;
use tower_http::services::ServeDir;

mod auth;
mod middle;

/// redirect returns a redirect response
fn redirect(url: &str) -> impl IntoResponse {
    Response::builder()
        .status(StatusCode::FOUND)
        .header("Location", url)
        .body(Body::empty())
        .unwrap()
}

async fn handler(Extension(user): Extension<AuthUser>, engine: Engine) -> impl IntoResponse {
    #[derive(Serialize)]
    struct Vars {
        pub page: Page,
    }
    RenderHtml(
        "index.hbs",
        engine,
        Vars {
            page: Page::new("Dashboard", BreadCrumbKey::Home, Some(user)),
        },
    )
}

pub async fn route(assets_dir: &str, tpl_dir: Option<String>) -> Result<Router> {
    // Extract templates
    let hbs = new_handlebar(assets_dir, tpl_dir.clone())?;
    // set static assets directory
    let static_assets_dir = format!("{}/static", tpl_dir.unwrap_or(assets_dir.to_string()));

    let app = Router::new()
        .route("/", get(handler))
        .route("/sign-in", get(auth::sign_in))
        .route("/sign-callback", get(auth::callback))
        .route("/sign-out", get(auth::sign_out))
        .nest_service("/static", ServeDir::new(static_assets_dir))
        .route_layer(middleware::from_fn(middle::auth))
        .route_layer(middleware::from_fn(middle::logger))
        .with_state(Engine::from(hbs));
    Ok(app)
}

// Make our own error that wraps `anyhow::Error`.
pub struct ServerError(pub StatusCode, pub anyhow::Error);

impl Clone for ServerError {
    fn clone(&self) -> Self {
        Self(self.0, anyhow::anyhow!(self.1.to_string()))
    }
}

impl ServerError {
    /// status_code creates a new `ServerError` with the given status code and message.
    pub fn _status_code(code: StatusCode, msg: &str) -> Self {
        Self(code, anyhow!(msg.to_string()))
    }
}

// This enables using `?` on functions that return `Result<_, anyhow::Error>` to turn them into
// `Result<_, RespError>`. That way you don't need to do that manually.
impl<E> From<E> for ServerError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(StatusCode::INTERNAL_SERVER_ERROR, err.into())
    }
}

// Tell axum how to convert `RespError` into a response.
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        let mut resp = (self.0, self.1.to_string()).into_response();
        let exts = resp.extensions_mut();
        exts.insert(self);
        resp
    }
}
