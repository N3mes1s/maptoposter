use std::env;
use std::path::PathBuf;

/// Application configuration loaded from environment variables
#[derive(Clone, Debug)]
pub struct Settings {
    /// Server port
    pub port: u16,
    /// Path to themes directory
    pub themes_dir: PathBuf,
    /// Path to fonts directory
    pub fonts_dir: PathBuf,
    /// Path to static output directory
    pub static_dir: PathBuf,
    /// Path to frontend directory
    pub frontend_dir: PathBuf,
    /// Default theme name
    pub default_theme: String,
    /// Default map distance in meters
    pub default_distance: u32,
    /// Minimum allowed distance
    pub min_distance: u32,
    /// Maximum allowed distance
    pub max_distance: u32,
    /// Output DPI
    pub output_dpi: u32,
    /// Preview DPI (lower quality for speed)
    pub preview_dpi: u32,
    /// Nominatim API delay in seconds
    pub nominatim_delay: f64,
    /// Nominatim API timeout in seconds
    pub nominatim_timeout: f64,
    /// OSM API delay in seconds
    pub osm_delay: f64,
    /// OSM API timeout in seconds
    pub osm_timeout: f64,
    /// Maximum concurrent jobs
    pub max_concurrent_jobs: usize,
    /// Job time-to-live in hours
    pub job_ttl_hours: u32,
}

impl Settings {
    /// Load settings from environment variables with sensible defaults
    pub fn from_env() -> Self {
        Self {
            port: env::var("PORT")
                .or_else(|_| env::var("API_PORT"))
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(8000),
            themes_dir: env::var("THEMES_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("themes")),
            fonts_dir: env::var("FONTS_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("fonts")),
            static_dir: env::var("STATIC_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("static")),
            frontend_dir: env::var("FRONTEND_DIR")
                .map(PathBuf::from)
                .unwrap_or_else(|_| PathBuf::from("frontend")),
            default_theme: env::var("DEFAULT_THEME")
                .unwrap_or_else(|_| "feature_based".to_string()),
            default_distance: env::var("DEFAULT_DISTANCE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(15000),
            min_distance: env::var("MIN_DISTANCE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(2000),
            max_distance: env::var("MAX_DISTANCE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(50000),
            output_dpi: env::var("OUTPUT_DPI")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(300),
            preview_dpi: env::var("PREVIEW_DPI")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(72),
            nominatim_delay: env::var("NOMINATIM_DELAY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(1.0),
            nominatim_timeout: env::var("NOMINATIM_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(10.0),
            osm_delay: env::var("OSM_DELAY")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(0.5),
            osm_timeout: env::var("OSM_TIMEOUT")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(60.0),
            max_concurrent_jobs: env::var("MAX_CONCURRENT_JOBS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(5),
            job_ttl_hours: env::var("JOB_TTL_HOURS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(24),
        }
    }

    /// Validate that a distance is within allowed bounds
    pub fn validate_distance(&self, distance: u32) -> Result<u32, String> {
        if distance < self.min_distance {
            Err(format!(
                "Distance {} is below minimum of {}",
                distance, self.min_distance
            ))
        } else if distance > self.max_distance {
            Err(format!(
                "Distance {} is above maximum of {}",
                distance, self.max_distance
            ))
        } else {
            Ok(distance)
        }
    }

    /// Sanitize a filename for safe storage
    pub fn sanitize_filename(name: &str) -> String {
        name.chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '-' || c == '_' {
                    c
                } else {
                    '_'
                }
            })
            .collect()
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self::from_env()
    }
}
