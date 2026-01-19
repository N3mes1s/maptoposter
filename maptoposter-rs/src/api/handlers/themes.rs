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

    // Collect and sort themes by name for consistent ordering
    let mut themes: Vec<ThemeInfo> = themes_map
        .into_iter()
        .map(|(name, theme)| ThemeInfo {
            id: name.clone(),
            name,
            description: theme.get("description").and_then(|v| v.as_str()).map(String::from),
            bg: theme.get("bg").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
            text: theme.get("text").and_then(|v| v.as_str()).unwrap_or("#000000").to_string(),
            gradient_color: theme.get("gradient_color").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
            water: theme.get("water").and_then(|v| v.as_str()).unwrap_or("#C0C0C0").to_string(),
            parks: theme.get("parks").and_then(|v| v.as_str()).unwrap_or("#F0F0F0").to_string(),
            road_motorway: theme.get("road_motorway").and_then(|v| v.as_str()).map(String::from),
            road_primary: theme.get("road_primary").and_then(|v| v.as_str()).map(String::from),
            road_default: theme.get("road_default").and_then(|v| v.as_str()).map(String::from),
        })
        .collect();

    // Sort alphabetically by id for consistent UI ordering
    themes.sort_by(|a, b| a.id.cmp(&b.id));

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
        id: name.clone(),
        name,
        description: theme.get("description").and_then(|v| v.as_str()).map(String::from),
        bg: theme.get("bg").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
        text: theme.get("text").and_then(|v| v.as_str()).unwrap_or("#000000").to_string(),
        gradient_color: theme.get("gradient_color").and_then(|v| v.as_str()).unwrap_or("#FFFFFF").to_string(),
        water: theme.get("water").and_then(|v| v.as_str()).unwrap_or("#C0C0C0").to_string(),
        parks: theme.get("parks").and_then(|v| v.as_str()).unwrap_or("#F0F0F0").to_string(),
        road_motorway: theme.get("road_motorway").and_then(|v| v.as_str()).map(String::from),
        road_primary: theme.get("road_primary").and_then(|v| v.as_str()).map(String::from),
        road_default: theme.get("road_default").and_then(|v| v.as_str()).map(String::from),
    }))
}
