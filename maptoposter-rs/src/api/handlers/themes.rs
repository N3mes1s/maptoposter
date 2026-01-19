use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Json,
};

use crate::api::models::{ThemeInfo, ThemeListResponse};
use crate::api::state::AppState;
use crate::error::{AppError, Result};
use crate::themes::loader::{load_theme, load_themes};

/// List all available themes
pub async fn list_themes(State(state): State<Arc<AppState>>) -> Json<ThemeListResponse> {
    let themes_map = load_themes(&state.config.themes_dir);

    let themes: Vec<ThemeInfo> = themes_map
        .into_iter()
        .map(|(name, theme)| ThemeInfo {
            name,
            description: theme.get("description").and_then(|v| v.as_str()).map(String::from),
            bg: theme.get("bg").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
            text: theme.get("text").and_then(|v| v.as_str()).unwrap_or("#000000").to_string(),
            gradient_color: theme.get("gradient_color").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
            water: theme.get("water").and_then(|v| v.as_str()).unwrap_or("#C0C0C0").to_string(),
            parks: theme.get("parks").and_then(|v| v.as_str()).unwrap_or("#F0F0F0").to_string(),
        })
        .collect();

    let count = themes.len();
    Json(ThemeListResponse { themes, count })
}

/// Get a specific theme by name
pub async fn get_theme(
    State(state): State<Arc<AppState>>,
    Path(name): Path<String>,
) -> Result<Json<ThemeInfo>> {
    let theme = load_theme(&state.config.themes_dir, &name)
        .ok_or_else(|| AppError::ThemeNotFound(name.clone()))?;

    Ok(Json(ThemeInfo {
        name,
        description: theme.get("description").and_then(|v| v.as_str()).map(String::from),
        bg: theme.get("bg").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
        text: theme.get("text").and_then(|v| v.as_str()).unwrap_or("#000000").to_string(),
        gradient_color: theme.get("gradient_color").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
        water: theme.get("water").and_then(|v| v.as_str()).unwrap_or("#C0C0C0").to_string(),
        parks: theme.get("parks").and_then(|v| v.as_str()).unwrap_or("#F0F0F0").to_string(),
    }))
}
