use serde::Deserialize;

use crate::error::{AppError, Result};

const NOMINATIM_URL: &str = "https://nominatim.openstreetmap.org";
const USER_AGENT: &str = "MapToPoster-RS/2.0 (https://github.com/maptoposter)";

/// Raw Nominatim search response
#[derive(Debug, Deserialize)]
struct NominatimResult {
    #[serde(default)]
    lat: String,
    #[serde(default)]
    lon: String,
    display_name: String,
    #[serde(default)]
    address: Option<NominatimAddress>,
}

#[derive(Debug, Deserialize)]
struct NominatimAddress {
    city: Option<String>,
    town: Option<String>,
    village: Option<String>,
    municipality: Option<String>,
    country: Option<String>,
}

/// Parsed location result
#[derive(Debug, Clone)]
pub struct LocationData {
    pub lat: f64,
    pub lon: f64,
    pub display_name: String,
    pub city: Option<String>,
    pub country: Option<String>,
}

/// Geocode a city and country to coordinates
pub async fn geocode(city: &str, country: &str, timeout_secs: f64) -> Result<(f64, f64)> {
    let query = format!("{}, {}", city, country);
    let results = search_nominatim(&query, 1, timeout_secs).await?;

    results
        .into_iter()
        .next()
        .map(|r| (r.lat, r.lon))
        .ok_or_else(|| AppError::Geocoding(format!("Location not found: {}, {}", city, country)))
}

/// Search Nominatim for locations matching a query
pub async fn search_nominatim(query: &str, limit: u32, timeout_secs: f64) -> Result<Vec<LocationData>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs_f64(timeout_secs))
        .user_agent(USER_AGENT)
        .build()?;

    let url = format!(
        "{}/search?q={}&format=json&limit={}&addressdetails=1",
        NOMINATIM_URL,
        urlencoding::encode(query),
        limit
    );

    let response = client.get(&url).send().await?;

    if !response.status().is_success() {
        return Err(AppError::Geocoding(format!(
            "Nominatim API error: {}",
            response.status()
        )));
    }

    let results: Vec<NominatimResult> = response.json().await?;

    let locations = results
        .into_iter()
        .filter_map(|r| {
            let lat = r.lat.parse::<f64>().ok()?;
            let lon = r.lon.parse::<f64>().ok()?;

            let city = r.address.as_ref().and_then(|a| {
                a.city
                    .clone()
                    .or_else(|| a.town.clone())
                    .or_else(|| a.village.clone())
                    .or_else(|| a.municipality.clone())
            });

            let country = r.address.as_ref().and_then(|a| a.country.clone());

            Some(LocationData {
                lat,
                lon,
                display_name: r.display_name,
                city,
                country,
            })
        })
        .collect();

    Ok(locations)
}

/// Format coordinates for display (e.g., "40.7128° N, 74.0060° W")
pub fn format_coordinates(lat: f64, lon: f64) -> String {
    let lat_dir = if lat >= 0.0 { "N" } else { "S" };
    let lon_dir = if lon >= 0.0 { "E" } else { "W" };

    format!(
        "{:.4}° {}, {:.4}° {}",
        lat.abs(),
        lat_dir,
        lon.abs(),
        lon_dir
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_coordinates() {
        assert_eq!(
            format_coordinates(40.7128, -74.0060),
            "40.7128° N, 74.0060° W"
        );
        assert_eq!(
            format_coordinates(-33.8688, 151.2093),
            "33.8688° S, 151.2093° E"
        );
    }
}
