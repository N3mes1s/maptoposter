use std::collections::HashMap;
use std::sync::Arc;

use chrono::{DateTime, Utc};
use parking_lot::RwLock;
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::api::models::{JobStatus, JobStatusResponse};
use crate::config::Settings;

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

/// Application state shared across handlers
pub struct AppState {
    pub config: Settings,
    pub jobs: RwLock<HashMap<Uuid, JobState>>,
    pub job_sender: mpsc::Sender<JobRequest>,
    job_receiver: RwLock<Option<mpsc::Receiver<JobRequest>>>,
}

impl AppState {
    pub fn new(config: Settings) -> Self {
        let (tx, rx) = mpsc::channel(100);
        Self {
            config,
            jobs: RwLock::new(HashMap::new()),
            job_sender: tx,
            job_receiver: RwLock::new(Some(rx)),
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

        self.jobs.write().retain(|_, job| {
            job.created_at > cutoff
        });
    }
}
