use crate::core::osm_client::HighwayType;

/// Road style configuration
#[derive(Debug, Clone)]
pub struct RoadStyle {
    pub width: f32,
    pub color_key: &'static str,
    pub default_color: &'static str,
}

impl RoadStyle {
    pub fn for_highway(highway_type: HighwayType) -> Self {
        match highway_type {
            HighwayType::Motorway | HighwayType::MotorwayLink => Self {
                width: 1.2,
                color_key: "road_motorway",
                default_color: "#0A0A0A",
            },
            HighwayType::Trunk | HighwayType::Primary | HighwayType::PrimaryLink => Self {
                width: 1.0,
                color_key: "road_primary",
                default_color: "#1A1A1A",
            },
            HighwayType::Secondary | HighwayType::SecondaryLink => Self {
                width: 0.8,
                color_key: "road_secondary",
                default_color: "#2A2A2A",
            },
            HighwayType::Tertiary | HighwayType::TertiaryLink => Self {
                width: 0.6,
                color_key: "road_tertiary",
                default_color: "#3A3A3A",
            },
            HighwayType::Residential | HighwayType::LivingStreet => Self {
                width: 0.4,
                color_key: "road_residential",
                default_color: "#4A4A4A",
            },
            HighwayType::Service | HighwayType::Unclassified => Self {
                width: 0.3,
                color_key: "road_residential",
                default_color: "#4A4A4A",
            },
            HighwayType::Default => Self {
                width: 0.4,
                color_key: "road_default",
                default_color: "#3A3A3A",
            },
        }
    }
}

/// Get the drawing priority for a highway type (higher = drawn later = on top)
pub fn highway_priority(highway_type: HighwayType) -> u8 {
    match highway_type {
        HighwayType::Motorway | HighwayType::MotorwayLink => 10,
        HighwayType::Trunk => 9,
        HighwayType::Primary | HighwayType::PrimaryLink => 8,
        HighwayType::Secondary | HighwayType::SecondaryLink => 6,
        HighwayType::Tertiary | HighwayType::TertiaryLink => 4,
        HighwayType::Residential | HighwayType::LivingStreet => 2,
        HighwayType::Service | HighwayType::Unclassified | HighwayType::Default => 1,
    }
}
