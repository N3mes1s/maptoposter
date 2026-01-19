/// Progress step names
pub const STEP_GEOCODING: &str = "geocoding";
pub const STEP_FETCHING_STREETS: &str = "fetching_streets";
pub const STEP_FETCHING_WATER: &str = "fetching_water";
pub const STEP_FETCHING_PARKS: &str = "fetching_parks";
pub const STEP_RENDERING_BACKGROUND: &str = "rendering_background";
pub const STEP_RENDERING_WATER: &str = "rendering_water";
pub const STEP_RENDERING_PARKS: &str = "rendering_parks";
pub const STEP_RENDERING_ROADS: &str = "rendering_roads";
pub const STEP_RENDERING_GRADIENTS: &str = "rendering_gradients";
pub const STEP_RENDERING_TEXT: &str = "rendering_text";
pub const STEP_SAVING: &str = "saving";
pub const STEP_COMPLETED: &str = "completed";

/// Progress update for poster generation
#[derive(Debug, Clone)]
pub struct GenerationProgress {
    pub step: String,
    pub progress: f32,
    pub message: String,
}

impl GenerationProgress {
    pub fn new(step: &str, progress: f32, message: &str) -> Self {
        Self {
            step: step.to_string(),
            progress,
            message: message.to_string(),
        }
    }

    pub fn geocoding() -> Self {
        Self::new(STEP_GEOCODING, 0.05, "Geocoding location...")
    }

    pub fn fetching_streets() -> Self {
        Self::new(STEP_FETCHING_STREETS, 0.15, "Fetching street network...")
    }

    pub fn fetching_water() -> Self {
        Self::new(STEP_FETCHING_WATER, 0.30, "Fetching water features...")
    }

    pub fn fetching_parks() -> Self {
        Self::new(STEP_FETCHING_PARKS, 0.40, "Fetching park features...")
    }

    pub fn rendering_background() -> Self {
        Self::new(STEP_RENDERING_BACKGROUND, 0.50, "Rendering background...")
    }

    pub fn rendering_water() -> Self {
        Self::new(STEP_RENDERING_WATER, 0.55, "Rendering water features...")
    }

    pub fn rendering_parks() -> Self {
        Self::new(STEP_RENDERING_PARKS, 0.60, "Rendering park features...")
    }

    pub fn rendering_roads() -> Self {
        Self::new(STEP_RENDERING_ROADS, 0.70, "Rendering road network...")
    }

    pub fn rendering_gradients() -> Self {
        Self::new(STEP_RENDERING_GRADIENTS, 0.85, "Applying gradient fades...")
    }

    pub fn rendering_text() -> Self {
        Self::new(STEP_RENDERING_TEXT, 0.90, "Rendering typography...")
    }

    pub fn saving() -> Self {
        Self::new(STEP_SAVING, 0.95, "Saving poster...")
    }

    pub fn completed() -> Self {
        Self::new(STEP_COMPLETED, 1.0, "Poster generated successfully!")
    }
}

/// Progress callback type
pub type ProgressCallback = Box<dyn Fn(GenerationProgress) + Send + Sync>;
