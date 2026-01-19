use std::collections::HashMap;

// geo types available for future use if needed
use serde::Deserialize;

use crate::error::{AppError, Result};

const OVERPASS_URL: &str = "https://overpass-api.de/api/interpreter";
const USER_AGENT: &str = "MapToPoster-RS/2.0 (https://github.com/maptoposter)";

/// Highway types with their rendering priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HighwayType {
    Motorway,
    MotorwayLink,
    Trunk,
    Primary,
    PrimaryLink,
    Secondary,
    SecondaryLink,
    Tertiary,
    TertiaryLink,
    Residential,
    LivingStreet,
    Service,
    Unclassified,
    Default,
}

impl HighwayType {
    pub fn from_str(s: &str) -> Self {
        match s {
            "motorway" => Self::Motorway,
            "motorway_link" => Self::MotorwayLink,
            "trunk" => Self::Trunk,
            "primary" => Self::Primary,
            "primary_link" => Self::PrimaryLink,
            "secondary" => Self::Secondary,
            "secondary_link" => Self::SecondaryLink,
            "tertiary" => Self::Tertiary,
            "tertiary_link" => Self::TertiaryLink,
            "residential" => Self::Residential,
            "living_street" => Self::LivingStreet,
            "service" => Self::Service,
            "unclassified" => Self::Unclassified,
            _ => Self::Default,
        }
    }

    /// Get the line width for this highway type
    pub fn line_width(&self) -> f32 {
        match self {
            Self::Motorway | Self::MotorwayLink => 1.2,
            Self::Trunk | Self::Primary | Self::PrimaryLink => 1.0,
            Self::Secondary | Self::SecondaryLink => 0.8,
            Self::Tertiary | Self::TertiaryLink => 0.6,
            Self::Residential | Self::LivingStreet => 0.4,
            Self::Service | Self::Unclassified => 0.3,
            Self::Default => 0.4,
        }
    }

    /// Get the theme color key for this highway type
    pub fn theme_key(&self) -> &'static str {
        match self {
            Self::Motorway | Self::MotorwayLink => "road_motorway",
            Self::Trunk | Self::Primary | Self::PrimaryLink => "road_primary",
            Self::Secondary | Self::SecondaryLink => "road_secondary",
            Self::Tertiary | Self::TertiaryLink => "road_tertiary",
            Self::Residential | Self::LivingStreet | Self::Service | Self::Unclassified => {
                "road_residential"
            }
            Self::Default => "road_default",
        }
    }
}

/// A road segment with coordinates and type
#[derive(Debug, Clone)]
pub struct RoadSegment {
    pub points: Vec<(f64, f64)>,
    pub highway_type: HighwayType,
}

/// Water or park polygon feature
#[derive(Debug, Clone)]
pub struct AreaFeature {
    pub points: Vec<(f64, f64)>,
    pub feature_type: String,
}

/// Overpass API response structures
#[derive(Debug, Deserialize)]
struct OverpassResponse {
    elements: Vec<OverpassElement>,
}

#[derive(Debug, Deserialize)]
struct OverpassElement {
    #[serde(rename = "type")]
    element_type: String,
    id: i64,
    #[serde(default)]
    lat: Option<f64>,
    #[serde(default)]
    lon: Option<f64>,
    #[serde(default)]
    nodes: Option<Vec<i64>>,
    #[serde(default)]
    tags: Option<HashMap<String, String>>,
    #[serde(default)]
    members: Option<Vec<OverpassMember>>,
}

#[derive(Debug, Deserialize)]
struct OverpassMember {
    #[serde(rename = "type")]
    member_type: String,
    #[serde(rename = "ref")]
    reference: i64,
    role: String,
}

/// Fetch street network from Overpass API
pub async fn fetch_streets(
    center: (f64, f64),
    distance: u32,
    timeout_secs: f64,
) -> Result<Vec<RoadSegment>> {
    let query = format!(
        r#"[out:json][timeout:90];
(
  way["highway"~"^(motorway|motorway_link|trunk|primary|primary_link|secondary|secondary_link|tertiary|tertiary_link|residential|living_street|service|unclassified)$"](around:{},{},{});
);
out body;
>;
out skel qt;"#,
        distance, center.0, center.1
    );

    let response = execute_overpass_query(&query, timeout_secs).await?;
    parse_road_segments(&response)
}

/// Fetch water features from Overpass API
pub async fn fetch_water(
    center: (f64, f64),
    distance: u32,
    timeout_secs: f64,
) -> Result<Vec<AreaFeature>> {
    let query = format!(
        r#"[out:json][timeout:60];
(
  way["natural"="water"](around:{},{},{});
  way["waterway"="riverbank"](around:{},{},{});
  relation["natural"="water"](around:{},{},{});
);
out body;
>;
out skel qt;"#,
        distance, center.0, center.1, distance, center.0, center.1, distance, center.0, center.1
    );

    let response = execute_overpass_query(&query, timeout_secs).await?;
    parse_area_features(&response, "water")
}

/// Fetch park features from Overpass API
pub async fn fetch_parks(
    center: (f64, f64),
    distance: u32,
    timeout_secs: f64,
) -> Result<Vec<AreaFeature>> {
    let query = format!(
        r#"[out:json][timeout:60];
(
  way["leisure"="park"](around:{},{},{});
  way["landuse"="grass"](around:{},{},{});
  way["landuse"="forest"](around:{},{},{});
  relation["leisure"="park"](around:{},{},{});
);
out body;
>;
out skel qt;"#,
        distance, center.0, center.1, distance, center.0, center.1, distance, center.0, center.1,
        distance, center.0, center.1
    );

    let response = execute_overpass_query(&query, timeout_secs).await?;
    parse_area_features(&response, "park")
}

/// Execute an Overpass API query
async fn execute_overpass_query(query: &str, timeout_secs: f64) -> Result<OverpassResponse> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs_f64(timeout_secs))
        .user_agent(USER_AGENT)
        .build()?;

    let response = client
        .post(OVERPASS_URL)
        .body(query.to_string())
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(AppError::DataFetch(format!(
            "Overpass API error: {}",
            response.status()
        )));
    }

    let data: OverpassResponse = response.json().await?;
    Ok(data)
}

/// Parse road segments from Overpass response
fn parse_road_segments(response: &OverpassResponse) -> Result<Vec<RoadSegment>> {
    // Build node lookup table
    let mut nodes: HashMap<i64, (f64, f64)> = HashMap::new();
    for element in &response.elements {
        if element.element_type == "node" {
            if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                nodes.insert(element.id, (lat, lon));
            }
        }
    }

    // Parse ways into road segments
    let mut segments = Vec::new();
    for element in &response.elements {
        if element.element_type == "way" {
            if let Some(node_ids) = &element.nodes {
                let points: Vec<(f64, f64)> = node_ids
                    .iter()
                    .filter_map(|id| nodes.get(id).copied())
                    .collect();

                if points.len() >= 2 {
                    let highway_type = element
                        .tags
                        .as_ref()
                        .and_then(|t| t.get("highway"))
                        .map(|s| HighwayType::from_str(s))
                        .unwrap_or(HighwayType::Default);

                    segments.push(RoadSegment {
                        points,
                        highway_type,
                    });
                }
            }
        }
    }

    Ok(segments)
}

/// Parse area features from Overpass response
fn parse_area_features(response: &OverpassResponse, feature_type: &str) -> Result<Vec<AreaFeature>> {
    // Build node lookup table
    let mut nodes: HashMap<i64, (f64, f64)> = HashMap::new();
    for element in &response.elements {
        if element.element_type == "node" {
            if let (Some(lat), Some(lon)) = (element.lat, element.lon) {
                nodes.insert(element.id, (lat, lon));
            }
        }
    }

    // Parse ways into area features
    let mut features = Vec::new();
    for element in &response.elements {
        if element.element_type == "way" {
            if let Some(node_ids) = &element.nodes {
                let points: Vec<(f64, f64)> = node_ids
                    .iter()
                    .filter_map(|id| nodes.get(id).copied())
                    .collect();

                if points.len() >= 3 {
                    features.push(AreaFeature {
                        points,
                        feature_type: feature_type.to_string(),
                    });
                }
            }
        }
    }

    Ok(features)
}

/// Calculate bounding box from road segments
pub fn calculate_bounds(segments: &[RoadSegment]) -> Option<((f64, f64), (f64, f64))> {
    if segments.is_empty() {
        return None;
    }

    let mut min_lat = f64::MAX;
    let mut max_lat = f64::MIN;
    let mut min_lon = f64::MAX;
    let mut max_lon = f64::MIN;

    for segment in segments {
        for (lat, lon) in &segment.points {
            min_lat = min_lat.min(*lat);
            max_lat = max_lat.max(*lat);
            min_lon = min_lon.min(*lon);
            max_lon = max_lon.max(*lon);
        }
    }

    Some(((min_lat, min_lon), (max_lat, max_lon)))
}
