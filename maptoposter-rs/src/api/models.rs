use serde::{Deserialize, Serialize};

/// Request to create a new poster
#[derive(Debug, Deserialize)]
pub struct PosterCreateRequest {
    pub city: String,
    pub country: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_distance")]
    pub distance: u32,
}

fn default_theme() -> String {
    "feature_based".to_string()
}

fn default_distance() -> u32 {
    15000
}

/// Response when a poster job is created
#[derive(Debug, Serialize)]
pub struct PosterCreateResponse {
    pub job_id: String,
    pub status: String,
    pub estimated_time: u32,
}

/// Job status response
#[derive(Debug, Clone, Serialize)]
pub struct JobStatusResponse {
    pub job_id: String,
    pub status: JobStatus,
    pub progress: f32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_step: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// Job status enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JobStatus {
    Queued,
    Processing,
    Completed,
    Failed,
}

impl std::fmt::Display for JobStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobStatus::Queued => write!(f, "queued"),
            JobStatus::Processing => write!(f, "processing"),
            JobStatus::Completed => write!(f, "completed"),
            JobStatus::Failed => write!(f, "failed"),
        }
    }
}

/// Theme information response
#[derive(Debug, Clone, Serialize)]
pub struct ThemeInfo {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub bg: String,
    pub text: String,
    pub gradient_color: String,
    pub water: String,
    pub parks: String,
}

/// Theme list response
#[derive(Debug, Serialize)]
pub struct ThemeListResponse {
    pub themes: Vec<ThemeInfo>,
    pub count: usize,
}

/// Location search result
#[derive(Debug, Clone, Serialize)]
pub struct LocationResult {
    pub display_name: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

/// Location search response
#[derive(Debug, Serialize)]
pub struct LocationSearchResponse {
    pub results: Vec<LocationResult>,
    pub count: usize,
}

/// Health check response
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

/// Readiness check response
#[derive(Debug, Serialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub checks: ReadinessChecks,
}

/// Individual readiness checks
#[derive(Debug, Serialize)]
pub struct ReadinessChecks {
    pub themes: bool,
    pub fonts: bool,
    pub static_dir: bool,
}

/// Progress update for SSE streaming
#[derive(Debug, Clone, Serialize)]
pub struct ProgressUpdate {
    pub job_id: String,
    pub status: JobStatus,
    pub progress: f32,
    pub step: String,
    pub message: String,
}
