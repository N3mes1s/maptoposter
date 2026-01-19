use std::sync::Arc;

use axum::{extract::State, Json};

use crate::api::models::{HealthResponse, ReadinessChecks, ReadinessResponse};
use crate::api::state::AppState;
use crate::themes::loader::load_themes;

/// Basic health check endpoint
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// Readiness check that verifies themes, fonts, and directories exist
pub async fn readiness_check(State(state): State<Arc<AppState>>) -> Json<ReadinessResponse> {
    let themes_ok = !load_themes(&state.config.themes_dir).is_empty();
    let fonts_ok = state.config.fonts_dir.join("Roboto-Bold.ttf").exists();
    let static_ok = state.config.static_dir.exists() || std::fs::create_dir_all(&state.config.static_dir).is_ok();

    let all_ok = themes_ok && fonts_ok && static_ok;

    Json(ReadinessResponse {
        status: if all_ok { "ready" } else { "not_ready" }.to_string(),
        checks: ReadinessChecks {
            themes: themes_ok,
            fonts: fonts_ok,
            static_dir: static_ok,
        },
    })
}
