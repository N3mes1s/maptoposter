use std::collections::HashMap;

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::api::models::{JobStatus, JobStatusResponse};
use crate::config::Settings;
use crate::core::osm_client::{AreaFeature, RoadSegment};
use crate::core::rate_limiter::{ApiRateLimiters, Cache};

/// Cached map data for re-rendering with different themes
#[derive(Debug, Clone)]
pub struct CachedMapData {
    pub city: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    pub distance: u32,
    pub streets: Vec<RoadSegment>,
    pub water: Vec<AreaFeature>,
    pub parks: Vec<AreaFeature>,
}

/// Internal job state
#[derive(Debug, Clone)]
pub struct JobState {
    pub id: Uuid,
    pub status: JobStatus,
    pub progress: f32,
    pub current_step: Option<String>,
    pub message: Option<String>,
    pub output_path: Option<String>,
    pub error: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub request: JobRequest,
}

/// Job request data
#[derive(Debug, Clone)]
pub struct JobRequest {
    pub city: String,
    pub country: String,
    pub theme: String,
    pub distance: u32,
}

impl JobState {
    pub fn new(request: JobRequest) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            status: JobStatus::Queued,
            progress: 0.0,
            current_step: None,
            message: Some("Job queued".to_string()),
            output_path: None,
            error: None,
            created_at: now,
            updated_at: now,
            request,
        }
    }

    pub fn to_response(&self) -> JobStatusResponse {
        JobStatusResponse {
            job_id: self.id.to_string(),
            status: self.status,
            progress: self.progress,
            current_step: self.current_step.clone(),
            message: self.message.clone(),
            download_url: self.output_path.as_ref().map(|_| {
                format!("/api/posters/{}/download", self.id)
            }),
            error: self.error.clone(),
        }
    }
}

/// Progress update message for job processing
#[derive(Debug, Clone)]
pub struct ProgressMessage {
    pub job_id: Uuid,
    pub status: JobStatus,
    pub progress: f32,
    pub step: String,
    pub message: String,
}

/// Cached geocoding result
#[derive(Debug, Clone)]
pub struct GeocodingResult {
    pub lat: f64,
    pub lon: f64,
}

/// Application state shared across handlers
pub struct AppState {
    pub config: Settings,
    pub jobs: RwLock<HashMap<Uuid, JobState>>,
    pub job_sender: mpsc::Sender<JobRequest>,
    job_receiver: RwLock<Option<mpsc::Receiver<JobRequest>>>,
    /// Rate limiters for external APIs
    pub rate_limiters: ApiRateLimiters,
    /// Cache for geocoding results (city,country -> coordinates)
    pub geocoding_cache: Cache<GeocodingResult>,
    /// Cache for map data (job_id -> map data) for re-rendering
    pub map_data_cache: RwLock<HashMap<Uuid, CachedMapData>>,
}

impl AppState {
    pub fn new(config: Settings) -> Self {
        let (tx, rx) = mpsc::channel(100);

        // Create rate limiters with configured delays
        let rate_limiters = ApiRateLimiters::new(
            config.nominatim_delay,
            config.osm_delay,
        );

        // Cache geocoding results for 24 hours, max 1000 entries
        let geocoding_cache = Cache::new(24 * 60 * 60, 1000);

        Self {
            config,
            jobs: RwLock::new(HashMap::new()),
            job_sender: tx,
            job_receiver: RwLock::new(Some(rx)),
            rate_limiters,
            geocoding_cache,
            map_data_cache: RwLock::new(HashMap::new()),
        }
    }

    /// Take the job receiver (can only be called once)
    pub fn take_job_receiver(&self) -> Option<mpsc::Receiver<JobRequest>> {
        self.job_receiver.write().take()
    }

    /// Create a new job and return its state
    pub fn create_job(&self, request: JobRequest) -> JobState {
        let job = JobState::new(request);
        let id = job.id;
        self.jobs.write().insert(id, job.clone());
        job
    }

    /// Get a job by ID
    pub fn get_job(&self, id: Uuid) -> Option<JobState> {
        self.jobs.read().get(&id).cloned()
    }

    /// Update job status
    pub fn update_job_status(&self, id: Uuid, status: JobStatus) {
        if let Some(job) = self.jobs.write().get_mut(&id) {
            job.status = status;
            job.updated_at = Utc::now();
        }
    }

    /// Update job progress
    pub fn update_job_progress(
        &self,
        id: Uuid,
        progress: f32,
        step: Option<String>,
        message: Option<String>,
    ) {
        if let Some(job) = self.jobs.write().get_mut(&id) {
            job.progress = progress;
            job.current_step = step;
            job.message = message;
            job.updated_at = Utc::now();
        }
    }

    /// Mark job as completed
    pub fn complete_job(&self, id: Uuid, output_path: String) {
        if let Some(job) = self.jobs.write().get_mut(&id) {
            job.status = JobStatus::Completed;
            job.progress = 1.0;
            job.output_path = Some(output_path);
            job.current_step = Some("completed".to_string());
            job.message = Some("Poster generated successfully".to_string());
            job.updated_at = Utc::now();
        }
    }

    /// Mark job as failed
    pub fn fail_job(&self, id: Uuid, error: String) {
        if let Some(job) = self.jobs.write().get_mut(&id) {
            job.status = JobStatus::Failed;
            job.error = Some(error);
            job.current_step = Some("failed".to_string());
            job.updated_at = Utc::now();
        }
    }

    /// Clean up old jobs based on TTL
    pub fn cleanup_old_jobs(&self) {
        let ttl_hours = self.config.job_ttl_hours as i64;
        let cutoff = Utc::now() - chrono::Duration::hours(ttl_hours);

        // Collect IDs to remove
        let removed_ids: Vec<Uuid> = {
            let jobs = self.jobs.read();
            jobs.iter()
                .filter(|(_, job)| job.created_at <= cutoff)
                .map(|(id, _)| *id)
                .collect()
        };

        // Remove jobs
        {
            let mut jobs = self.jobs.write();
            for id in &removed_ids {
                jobs.remove(id);
            }
        }

        // Also clean up cached map data for removed jobs
        {
            let mut cache = self.map_data_cache.write();
            for id in removed_ids {
                cache.remove(&id);
            }
        }
    }

    /// Store cached map data for a job
    pub fn cache_map_data(&self, job_id: Uuid, data: CachedMapData) {
        self.map_data_cache.write().insert(job_id, data);
    }

    /// Get cached map data for a job
    pub fn get_cached_map_data(&self, job_id: Uuid) -> Option<CachedMapData> {
        self.map_data_cache.read().get(&job_id).cloned()
    }
}
