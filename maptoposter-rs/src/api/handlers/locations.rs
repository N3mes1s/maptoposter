use std::sync::Arc;

use axum::{
    extract::{Query, State},
    Json,
};
use serde::Deserialize;

use crate::api::models::{LocationResult, LocationSearchResponse};
use crate::api::state::AppState;
use crate::core::geocoding::search_nominatim;
use crate::error::Result;

/// Query parameters for location search
#[derive(Debug, Deserialize)]
pub struct LocationSearchQuery {
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: u32,
}

fn default_limit() -> u32 {
    5
}

/// Search for locations using Nominatim
pub async fn search_locations(
    State(state): State<Arc<AppState>>,
    Query(query): Query<LocationSearchQuery>,
) -> Result<Json<LocationSearchResponse>> {
    let results = search_nominatim(&query.q, query.limit, state.config.nominatim_timeout).await?;

    let locations: Vec<LocationResult> = results
        .into_iter()
        .map(|r| LocationResult {
            display_name: r.display_name,
            lat: r.lat,
            lon: r.lon,
            city: r.city,
            country: r.country,
        })
        .collect();

    let count = locations.len();
    Ok(Json(LocationSearchResponse {
        results: locations,
        count,
    }))
}
