use std::path::Path;

use serde_json::Value;

use crate::core::geocoding::{format_coordinates, geocode};
use crate::core::osm_client::{calculate_bounds, fetch_parks, fetch_streets, fetch_water};
use crate::core::progress::{GenerationProgress, ProgressCallback};
use crate::error::{AppError, Result};
use crate::rendering::canvas::Canvas;
use crate::rendering::gradients::apply_gradient_fades;
use crate::rendering::typography::{render_poster_typography, FontSet};
use crate::themes::loader::get_theme_color;

/// Request for poster generation
#[derive(Debug, Clone)]
pub struct PosterRequest {
    pub city: String,
    pub country: String,
    pub theme_name: String,
    pub distance: u32,
    pub dpi: u32,
}

impl Default for PosterRequest {
    fn default() -> Self {
        Self {
            city: String::new(),
            country: String::new(),
            theme_name: "feature_based".to_string(),
            distance: 15000,
            dpi: 300,
        }
    }
}

/// Poster generator with theme and configuration
pub struct PosterGenerator {
    theme: Value,
    fonts: FontSet,
    nominatim_timeout: f64,
    osm_timeout: f64,
}

impl PosterGenerator {
    /// Create a new poster generator
    pub fn new(
        theme: Value,
        fonts_dir: &Path,
        nominatim_timeout: f64,
        osm_timeout: f64,
    ) -> Result<Self> {
        let fonts = FontSet::load(fonts_dir)?;

        Ok(Self {
            theme,
            fonts,
            nominatim_timeout,
            osm_timeout,
        })
    }

    /// Generate a poster and save it to the specified path
    pub async fn generate(
        &self,
        request: &PosterRequest,
        output_path: &Path,
        progress_callback: Option<ProgressCallback>,
    ) -> Result<()> {
        let report = |progress: GenerationProgress| {
            if let Some(ref cb) = progress_callback {
                cb(progress);
            }
        };

        // Step 1: Geocode the location
        report(GenerationProgress::geocoding());
        let (lat, lon) = geocode(&request.city, &request.country, self.nominatim_timeout).await?;
        let coordinates = format_coordinates(lat, lon);
        tracing::info!("Geocoded {}, {} to ({}, {})", request.city, request.country, lat, lon);

        // Step 2: Fetch street network
        report(GenerationProgress::fetching_streets());
        let streets = fetch_streets((lat, lon), request.distance, self.osm_timeout).await?;
        tracing::info!("Fetched {} road segments", streets.len());

        if streets.is_empty() {
            return Err(AppError::DataFetch(
                "No street data found for this location".to_string(),
            ));
        }

        // Step 3: Fetch water features (non-fatal if missing)
        report(GenerationProgress::fetching_water());
        let water = match fetch_water((lat, lon), request.distance, self.osm_timeout).await {
            Ok(w) => {
                tracing::info!("Fetched {} water features", w.len());
                w
            }
            Err(e) => {
                tracing::warn!("Could not fetch water features: {}", e);
                Vec::new()
            }
        };

        // Step 4: Fetch park features (non-fatal if missing)
        report(GenerationProgress::fetching_parks());
        let parks = match fetch_parks((lat, lon), request.distance, self.osm_timeout).await {
            Ok(p) => {
                tracing::info!("Fetched {} park features", p.len());
                p
            }
            Err(e) => {
                tracing::warn!("Could not fetch park features: {}", e);
                Vec::new()
            }
        };

        // Step 5: Create canvas and set up coordinate transform
        report(GenerationProgress::rendering_background());
        let mut canvas = Canvas::poster()?;

        // Fill background
        let bg_color = get_theme_color(&self.theme, "bg", "#FFFFFF");
        canvas.fill_background(&bg_color);

        // Calculate bounds and set transform
        let bounds = calculate_bounds(&streets)
            .ok_or_else(|| AppError::Rendering("Could not calculate map bounds".to_string()))?;
        canvas.set_geo_transform(bounds);

        // Step 6: Render water features
        report(GenerationProgress::rendering_water());
        if !water.is_empty() {
            let water_color = get_theme_color(&self.theme, "water", "#C0C0C0");
            canvas.draw_polygons(&water, &water_color);
        }

        // Step 7: Render park features
        report(GenerationProgress::rendering_parks());
        if !parks.is_empty() {
            let parks_color = get_theme_color(&self.theme, "parks", "#F0F0F0");
            canvas.draw_polygons(&parks, &parks_color);
        }

        // Step 8: Render roads
        report(GenerationProgress::rendering_roads());
        // Calculate base width multiplier based on distance (larger distance = thinner lines)
        let base_width = 2.0 * (15000.0 / request.distance as f32).sqrt();
        canvas.draw_roads(&streets, &self.theme, base_width);

        // Step 9: Apply gradient fades
        report(GenerationProgress::rendering_gradients());
        let gradient_color = get_theme_color(&self.theme, "gradient_color", &bg_color);
        apply_gradient_fades(&mut canvas.pixmap, &gradient_color);

        // Step 10: Render typography
        report(GenerationProgress::rendering_text());
        let text_color = get_theme_color(&self.theme, "text", "#000000");
        render_poster_typography(
            &mut canvas.pixmap,
            &self.fonts,
            &request.city,
            &request.country,
            &coordinates,
            &text_color,
        );

        // Step 11: Save the poster
        report(GenerationProgress::saving());
        canvas.save_png(output_path)?;
        tracing::info!("Saved poster to {:?}", output_path);

        report(GenerationProgress::completed());
        Ok(())
    }
}
