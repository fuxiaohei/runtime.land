use crate::server::PageVars;

use super::templates::{RenderHtmlMinified, TemplateEngine};
use anyhow::Result;
use axum::{
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use axum_template::engine::Engine;
use tower_http::services::ServeDir;

mod auth;

/// index is a handler for GET /
pub async fn index(engine: TemplateEngine) -> impl IntoResponse {
    #[derive(serde::Serialize)]
    struct IndexVars {
        page: PageVars,
    }
    // redirect to /overview
    RenderHtmlMinified(
        "index.hbs",
        engine,
        IndexVars {
            page: PageVars::new("Dashboard", "Overview"),
        },
    )
}

/// router returns the router for the dashboard
pub fn router(assets_dir: &str) -> Result<Router> {
    super::templates::extract(assets_dir)?;

    // init handlebars template engine
    let hbs = super::templates::init(assets_dir)?;

    // set static assets directory
    let static_assets_dir = format!("{}/static", assets_dir);

    let app = Router::new()
        .route("/", any(index))
        .route("/sign-in", get(auth::sign_in))
        .nest_service("/static", ServeDir::new(static_assets_dir))
        .with_state(Engine::from(hbs));
    Ok(app)
}
