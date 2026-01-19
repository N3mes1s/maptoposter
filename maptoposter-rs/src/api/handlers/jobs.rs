use std::sync::Arc;
use std::time::Duration;

use axum::{
    extract::{Path, State},
    response::sse::{Event, Sse},
};
use futures::stream::Stream;
use tokio_stream::wrappers::IntervalStream;
use tokio_stream::StreamExt;
use uuid::Uuid;

use crate::api::models::{JobStatus, ProgressUpdate};
use crate::api::state::AppState;
use crate::error::{AppError, Result};

/// Stream job progress updates via Server-Sent Events
pub async fn stream_progress(
    State(state): State<Arc<AppState>>,
    Path(job_id): Path<String>,
) -> Result<Sse<impl Stream<Item = std::result::Result<Event, std::convert::Infallible>>>> {
    let uuid = Uuid::parse_str(&job_id).map_err(|_| AppError::JobNotFound(job_id.clone()))?;

    // Verify job exists
    if state.get_job(uuid).is_none() {
        return Err(AppError::JobNotFound(job_id));
    }

    // Create an interval stream that polls job status
    let interval = tokio::time::interval(Duration::from_millis(500));
    let stream = IntervalStream::new(interval);

    let state_clone = state.clone();
    let stream = stream
        .map(move |_| {
            let job = state_clone.get_job(uuid);

            match job {
                Some(job) => {
                    let update = ProgressUpdate {
                        job_id: job.id.to_string(),
                        status: job.status,
                        progress: job.progress,
                        step: job.current_step.unwrap_or_default(),
                        message: job.message.unwrap_or_default(),
                    };

                    let data = serde_json::to_string(&update).unwrap_or_default();

                    // Check if job is terminal (completed or failed)
                    let is_terminal =
                        job.status == JobStatus::Completed || job.status == JobStatus::Failed;

                    if is_terminal {
                        // Send final event and signal end
                        Event::default().data(data).event("completed")
                    } else {
                        Event::default().data(data).event("progress")
                    }
                }
                None => {
                    // Job no longer exists
                    Event::default()
                        .data("{\"error\": \"Job not found\"}")
                        .event("error")
                }
            }
        })
        // Note: SSE will continue until client disconnects
        // The "complete" event tells client to close connection
        .map(Ok);

    Ok(Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(15))
            .text("keep-alive"),
    ))
}
