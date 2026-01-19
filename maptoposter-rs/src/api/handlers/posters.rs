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

use crate::api::models::{JobStatus, JobStatusResponse, PosterCreateRequest, PosterCreateResponse, ReRenderRequest};
use crate::api::state::{AppState, CachedMapData, JobRequest};
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

    // Spawn background task for poster generation with timeout and panic handling
    let state_clone = state.clone();
    let job_timeout = std::time::Duration::from_secs(180); // 3 minute timeout for entire job

    tokio::spawn(async move {
        // Wrap job processing with timeout
        let job_result = tokio::time::timeout(
            job_timeout,
            AssertUnwindSafe(process_poster_job(
                state_clone.clone(),
                job_id,
                job_request,
            ))
            .catch_unwind()
        ).await;

        match job_result {
            Ok(Ok(())) => {
                // Job completed normally
            }
            Ok(Err(_panic)) => {
                tracing::error!("Job {} panicked during processing", job_id);
                state_clone.fail_job(job_id, "Internal error: job processing crashed".to_string());
            }
            Err(_timeout) => {
                tracing::error!("Job {} timed out after {:?}", job_id, job_timeout);
                state_clone.fail_job(job_id, "Generation timed out - try a smaller area".to_string());
            }
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

    // Generate poster and cache map data
    match generator
        .generate_with_cache(&poster_request, &output_path, Some(progress_callback))
        .await
    {
        Ok(map_data) => {
            // Cache map data for re-rendering
            let cached_data = CachedMapData {
                city: map_data.city,
                country: map_data.country,
                lat: map_data.lat,
                lon: map_data.lon,
                distance: map_data.distance,
                streets: map_data.streets,
                water: map_data.water,
                parks: map_data.parks,
            };
            state.cache_map_data(job_id, cached_data);
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

/// Re-render a poster with a different theme using cached map data
pub async fn rerender_poster(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
    Json(request): Json<ReRenderRequest>,
) -> Result<Json<PosterCreateResponse>> {
    let uuid = Uuid::parse_str(&job_id).map_err(|_| AppError::JobNotFound(job_id.clone()))?;

    // Get cached map data
    let cached_data = state
        .get_cached_map_data(uuid)
        .ok_or_else(|| AppError::Internal("No cached data available for this job".to_string()))?;

    // Validate theme exists
    if load_theme(&state.config.themes_dir, &request.theme).is_none() {
        return Err(AppError::ThemeNotFound(request.theme.clone()));
    }

    // Create new job for re-render
    let job_request = JobRequest {
        city: cached_data.city.clone(),
        country: cached_data.country.clone(),
        theme: request.theme.clone(),
        distance: cached_data.distance,
    };

    let new_job = state.create_job(job_request);
    let new_job_id = new_job.id;

    // Copy cached data to new job
    state.cache_map_data(new_job_id, cached_data.clone());

    // Spawn background task for re-rendering with timeout
    let state_clone = state.clone();
    let theme_name = request.theme.clone();
    let rerender_timeout = std::time::Duration::from_secs(30); // 30 second timeout for re-render

    tokio::spawn(async move {
        let job_result = tokio::time::timeout(
            rerender_timeout,
            AssertUnwindSafe(process_rerender_job(
                state_clone.clone(),
                new_job_id,
                theme_name,
                cached_data,
            ))
            .catch_unwind()
        ).await;

        match job_result {
            Ok(Ok(())) => {}
            Ok(Err(_panic)) => {
                tracing::error!("Re-render job {} panicked", new_job_id);
                state_clone.fail_job(new_job_id, "Internal error: re-render crashed".to_string());
            }
            Err(_timeout) => {
                tracing::error!("Re-render job {} timed out", new_job_id);
                state_clone.fail_job(new_job_id, "Re-render timed out".to_string());
            }
        }
    });

    Ok(Json(PosterCreateResponse {
        job_id: new_job_id.to_string(),
        status: "queued".to_string(),
        estimated_time: 5, // Re-render is much faster
    }))
}

/// Process a re-render job using cached data
async fn process_rerender_job(
    state: Arc<AppState>,
    job_id: Uuid,
    theme_name: String,
    cached_data: CachedMapData,
) {
    use crate::core::geocoding::format_coordinates;

    state.update_job_status(job_id, JobStatus::Processing);

    // Load theme
    let theme = match load_theme(&state.config.themes_dir, &theme_name) {
        Some(t) => t,
        None => {
            state.fail_job(job_id, format!("Theme '{}' not found", theme_name));
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

    // Output path
    let output_path = state.config.static_dir.join(format!("{}.png", job_id));

    // Create progress callback
    let state_clone = state.clone();
    let progress_callback = Box::new(move |progress: crate::core::progress::GenerationProgress| {
        state_clone.update_job_progress(
            job_id,
            progress.progress,
            Some(progress.step),
            Some(progress.message),
        );
    });

    // Convert cached data to MapData for rendering
    let map_data = crate::core::poster_generator::MapData {
        city: cached_data.city,
        country: cached_data.country,
        lat: cached_data.lat,
        lon: cached_data.lon,
        distance: cached_data.distance,
        streets: cached_data.streets,
        water: cached_data.water,
        parks: cached_data.parks,
    };

    let coordinates = format_coordinates(map_data.lat, map_data.lon);

    // Render using cached data (no network requests!)
    match generator.render_from_data(&map_data, &coordinates, &output_path, Some(progress_callback)) {
        Ok(()) => {
            state.complete_job(job_id, output_path.to_string_lossy().to_string());
        }
        Err(e) => {
            state.fail_job(job_id, e.to_string());
        }
    }
}

/// Estimate generation time in seconds based on distance
fn estimate_generation_time(distance: u32) -> u32 {
    // Rough estimate: 30 seconds base + 1 second per 1000m
    30 + distance / 1000
}
