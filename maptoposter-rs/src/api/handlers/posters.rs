use std::panic::AssertUnwindSafe;
use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Path, State},
    http::{header, StatusCode},
    response::Response,
    Json,
};
use futures::FutureExt;
use tokio::fs::File;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::api::models::{JobStatus, JobStatusResponse, PosterCreateRequest, PosterCreateResponse};
use crate::api::state::{AppState, JobRequest};
use crate::config::Settings;
use crate::core::poster_generator::{PosterGenerator, PosterRequest};
use crate::core::progress::GenerationProgress;
use crate::error::{AppError, Result};
use crate::themes::loader::load_theme;

/// Create a new poster generation job
pub async fn create_poster(
    State(state): State<Arc<AppState>>,
    Json(request): Json<PosterCreateRequest>,
) -> Result<Json<PosterCreateResponse>> {
    // Validate distance
    state.config.validate_distance(request.distance).map_err(AppError::InvalidDistance)?;

    // Validate theme exists
    if load_theme(&state.config.themes_dir, &request.theme).is_none() {
        return Err(AppError::ThemeNotFound(request.theme.clone()));
    }

    // Create job
    let job_request = JobRequest {
        city: request.city.clone(),
        country: request.country.clone(),
        theme: request.theme.clone(),
        distance: request.distance,
    };

    let job = state.create_job(job_request.clone());
    let job_id = job.id;

    // Spawn background task for poster generation with panic handling
    let state_clone = state.clone();
    tokio::spawn(async move {
        // Catch panics and mark job as failed
        let result = AssertUnwindSafe(process_poster_job(
            state_clone.clone(),
            job_id,
            job_request,
        ))
        .catch_unwind()
        .await;

        if result.is_err() {
            tracing::error!("Job {} panicked during processing", job_id);
            state_clone.fail_job(job_id, "Internal error: job processing crashed".to_string());
        }
    });

    Ok(Json(PosterCreateResponse {
        job_id: job_id.to_string(),
        status: "queued".to_string(),
        estimated_time: estimate_generation_time(request.distance),
    }))
}

/// Process a poster generation job
async fn process_poster_job(state: Arc<AppState>, job_id: Uuid, request: JobRequest) {
    // Update status to processing
    state.update_job_status(job_id, JobStatus::Processing);

    // Load theme
    let theme = match load_theme(&state.config.themes_dir, &request.theme) {
        Some(t) => t,
        None => {
            state.fail_job(job_id, format!("Theme '{}' not found", request.theme));
            return;
        }
    };

    // Create generator
    let generator = match PosterGenerator::new(
        theme,
        &state.config.fonts_dir,
        state.config.nominatim_timeout,
        state.config.osm_timeout,
    ) {
        Ok(g) => g,
        Err(e) => {
            state.fail_job(job_id, format!("Failed to create generator: {}", e));
            return;
        }
    };

    // Create poster request
    let poster_request = PosterRequest {
        city: request.city.clone(),
        country: request.country.clone(),
        theme_name: request.theme.clone(),
        distance: request.distance,
        dpi: state.config.output_dpi,
    };

    // Output path
    let output_path = state.config.static_dir.join(format!("{}.png", job_id));

    // Create progress callback
    let state_clone = state.clone();
    let progress_callback = Box::new(move |progress: GenerationProgress| {
        state_clone.update_job_progress(
            job_id,
            progress.progress,
            Some(progress.step),
            Some(progress.message),
        );
    });

    // Generate poster
    match generator
        .generate(&poster_request, &output_path, Some(progress_callback))
        .await
    {
        Ok(()) => {
            state.complete_job(job_id, output_path.to_string_lossy().to_string());
        }
        Err(e) => {
            state.fail_job(job_id, e.to_string());
        }
    }
}

/// Get the status of a poster job
pub async fn get_poster_status(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<Json<JobStatusResponse>> {
    let uuid = Uuid::parse_str(&job_id).map_err(|_| AppError::JobNotFound(job_id.clone()))?;

    let job = state
        .get_job(uuid)
        .ok_or_else(|| AppError::JobNotFound(job_id))?;

    Ok(Json(job.to_response()))
}

/// Download a completed poster
pub async fn download_poster(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<Response> {
    let uuid = Uuid::parse_str(&job_id).map_err(|_| AppError::JobNotFound(job_id.clone()))?;

    let job = state
        .get_job(uuid)
        .ok_or_else(|| AppError::JobNotFound(job_id.clone()))?;

    if job.status != JobStatus::Completed {
        return Err(AppError::Internal(format!(
            "Job {} is not completed (status: {})",
            job_id, job.status
        )));
    }

    let output_path = job
        .output_path
        .ok_or_else(|| AppError::Internal("No output path for completed job".to_string()))?;

    let file = File::open(&output_path)
        .await
        .map_err(|e| AppError::Io(e))?;

    let stream = ReaderStream::new(file);
    let body = Body::from_stream(stream);

    // Generate filename
    let filename = format!(
        "{}_{}.png",
        Settings::sanitize_filename(&job.request.city),
        Settings::sanitize_filename(&job.request.theme)
    );

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "image/png")
        .header(
            header::CONTENT_DISPOSITION,
            format!("attachment; filename=\"{}\"", filename),
        )
        .body(body)
        .unwrap())
}

/// Estimate generation time in seconds based on distance
fn estimate_generation_time(distance: u32) -> u32 {
    // Rough estimate: 30 seconds base + 1 second per 1000m
    30 + distance / 1000
}
