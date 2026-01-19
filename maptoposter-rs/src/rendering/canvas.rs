use std::path::Path;

use tiny_skia::{Color, FillRule, LineCap, LineJoin, Paint, PathBuilder, Pixmap, Stroke, Transform};

use crate::core::osm_client::{AreaFeature, HighwayType, RoadSegment};
use crate::error::{AppError, Result};
use crate::themes::loader::{get_theme_color, parse_hex_color};

/// Canvas dimensions for poster (12x16 inches at 300 DPI)
pub const POSTER_WIDTH: u32 = 3600;
pub const POSTER_HEIGHT: u32 = 4800;

/// Canvas for rendering the poster
pub struct Canvas {
    pub pixmap: Pixmap,
    pub width: u32,
    pub height: u32,
    /// Coordinate transform parameters
    geo_center: (f64, f64),  // (lat, lon)
    geo_scale: f64,
}

impl Canvas {
    /// Create a new canvas with the specified dimensions
    pub fn new(width: u32, height: u32) -> Result<Self> {
        let pixmap = Pixmap::new(width, height)
            .ok_or_else(|| AppError::Rendering("Failed to create pixmap".to_string()))?;

        Ok(Self {
            pixmap,
            width,
            height,
            geo_center: (0.0, 0.0),
            geo_scale: 1.0,
        })
    }

    /// Create a poster-sized canvas
    pub fn poster() -> Result<Self> {
        Self::new(POSTER_WIDTH, POSTER_HEIGHT)
    }

    /// Fill the entire canvas with a color
    pub fn fill_background(&mut self, hex_color: &str) {
        if let Some((r, g, b)) = parse_hex_color(hex_color) {
            let color = Color::from_rgba8(r, g, b, 255);
            self.pixmap.fill(color);
        }
    }

    /// Set the coordinate transform based on geographic bounds
    pub fn set_geo_transform(&mut self, bounds: ((f64, f64), (f64, f64))) {
        let ((min_lat, min_lon), (max_lat, max_lon)) = bounds;

        // Add some padding
        let lat_range = max_lat - min_lat;
        let lon_range = max_lon - min_lon;
        let padding = 0.05; // 5% padding

        let min_lat = min_lat - lat_range * padding;
        let max_lat = max_lat + lat_range * padding;
        let min_lon = min_lon - lon_range * padding;
        let max_lon = max_lon + lon_range * padding;

        let lat_range = max_lat - min_lat;
        let lon_range = max_lon - min_lon;

        // Calculate center of bounds
        let center_lat = (min_lat + max_lat) / 2.0;
        let center_lon = (min_lon + max_lon) / 2.0;

        // Calculate scale to fit the poster while maintaining aspect ratio
        let scale_x = self.width as f64 / lon_range;
        let scale_y = self.height as f64 / lat_range;
        let scale = scale_x.min(scale_y);

        // Store transform parameters
        self.geo_center = (center_lat, center_lon);
        self.geo_scale = scale;
    }

    /// Convert geographic coordinates to screen coordinates
    pub fn geo_to_screen(&self, lat: f64, lon: f64) -> (f32, f32) {
        let (center_lat, center_lon) = self.geo_center;

        // Convert lon to x (lon increases = x increases)
        let x = (lon - center_lon) * self.geo_scale + (self.width as f64 / 2.0);

        // Convert lat to y (lat increases = y decreases, since screen y goes down)
        let y = (center_lat - lat) * self.geo_scale + (self.height as f64 / 2.0);

        (x as f32, y as f32)
    }

    /// Draw filled polygons (for water, parks)
    pub fn draw_polygons(&mut self, features: &[AreaFeature], hex_color: &str) {
        let (r, g, b) = match parse_hex_color(hex_color) {
            Some(c) => c,
            None => return,
        };

        let mut paint = Paint::default();
        paint.set_color_rgba8(r, g, b, 255);
        paint.anti_alias = true;

        for feature in features {
            if feature.points.len() < 3 {
                continue;
            }

            let mut pb = PathBuilder::new();
            let (x, y) = self.geo_to_screen(feature.points[0].0, feature.points[0].1);
            pb.move_to(x, y);

            for (lat, lon) in &feature.points[1..] {
                let (x, y) = self.geo_to_screen(*lat, *lon);
                pb.line_to(x, y);
            }
            pb.close();

            if let Some(path) = pb.finish() {
                self.pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
            }
        }
    }

    /// Draw road segments with appropriate styling
    pub fn draw_roads(
        &mut self,
        segments: &[RoadSegment],
        theme: &serde_json::Value,
        base_width_multiplier: f32,
    ) {
        // Sort segments by highway type priority (draw minor roads first)
        let mut sorted_segments: Vec<&RoadSegment> = segments.iter().collect();
        sorted_segments.sort_by_key(|s| match s.highway_type {
            HighwayType::Motorway | HighwayType::MotorwayLink => 10,
            HighwayType::Trunk | HighwayType::Primary | HighwayType::PrimaryLink => 8,
            HighwayType::Secondary | HighwayType::SecondaryLink => 6,
            HighwayType::Tertiary | HighwayType::TertiaryLink => 4,
            _ => 2,
        });

        for segment in sorted_segments {
            if segment.points.len() < 2 {
                continue;
            }

            let color_key = segment.highway_type.theme_key();
            let hex_color = get_theme_color(theme, color_key, "#3A3A3A");
            let (r, g, b) = match parse_hex_color(&hex_color) {
                Some(c) => c,
                None => continue,
            };

            let mut paint = Paint::default();
            paint.set_color_rgba8(r, g, b, 255);
            paint.anti_alias = true;

            let line_width = segment.highway_type.line_width() * base_width_multiplier;

            let stroke = Stroke {
                width: line_width,
                line_cap: LineCap::Round,
                line_join: LineJoin::Round,
                ..Default::default()
            };

            let mut pb = PathBuilder::new();
            let (x, y) = self.geo_to_screen(segment.points[0].0, segment.points[0].1);
            pb.move_to(x, y);

            for (lat, lon) in &segment.points[1..] {
                let (x, y) = self.geo_to_screen(*lat, *lon);
                pb.line_to(x, y);
            }

            if let Some(path) = pb.finish() {
                self.pixmap.stroke_path(
                    &path,
                    &paint,
                    &stroke,
                    Transform::identity(),
                    None,
                );
            }
        }
    }

    /// Save the canvas to a PNG file
    pub fn save_png(&self, path: &Path) -> Result<()> {
        self.pixmap
            .save_png(path)
            .map_err(|e| AppError::Rendering(format!("Failed to save PNG: {}", e)))
    }

    /// Get PNG data as bytes
    pub fn to_png_bytes(&self) -> Result<Vec<u8>> {
        self.pixmap
            .encode_png()
            .map_err(|e| AppError::Rendering(format!("Failed to encode PNG: {}", e)))
    }
}
