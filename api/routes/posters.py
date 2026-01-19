"""Poster generation API routes."""

import uuid
from typing import Dict, Any

from fastapi import APIRouter, HTTPException, BackgroundTasks
from fastapi.responses import FileResponse

from src.maptoposter.config import settings
from src.maptoposter.themes.loader import get_available_themes, load_theme
from src.maptoposter.logging_config import get_logger
from src.maptoposter.exceptions import (
    GeocodingError,
    DataFetchError,
    ThemeNotFoundError,
    InvalidDistanceError,
    APITimeoutError,
)
from api.schemas.poster import PosterCreateRequest, PosterCreateResponse, JobStatus

logger = get_logger("api.posters")

router = APIRouter()

# In-memory job storage (replace with Redis in production)
jobs: Dict[str, Dict[str, Any]] = {}


def estimate_generation_time(distance: int) -> int:
    """
    Estimate generation time based on distance.

    Args:
        distance: Map radius in meters

    Returns:
        Estimated time in seconds
    """
    if distance <= 5000:
        return 30
    elif distance <= 10000:
        return 45
    elif distance <= 20000:
        return 60
    else:
        return 90


@router.post("", response_model=PosterCreateResponse)
async def create_poster(
    request: PosterCreateRequest,
    background_tasks: BackgroundTasks
) -> PosterCreateResponse:
    """
    Queue a new poster generation job.

    Args:
        request: Poster creation parameters
        background_tasks: FastAPI background tasks handler

    Returns:
        Job creation response with job_id and estimated time
    """
    logger.info(
        f"Creating poster job: city={request.city}, country={request.country}, "
        f"theme={request.theme}, distance={request.distance}"
    )

    # Validate theme exists
    available_themes = get_available_themes()
    if request.theme not in available_themes:
        logger.warning(f"Invalid theme requested: {request.theme}")
        raise HTTPException(
            status_code=400,
            detail=f"Theme '{request.theme}' not found. Available: {', '.join(available_themes[:5])}"
        )

    # Create job
    job_id = str(uuid.uuid4())
    logger.info(f"Created job {job_id} for {request.city}, {request.country}")

    jobs[job_id] = {
        "status": "queued",
        "progress": 0,
        "current_step": None,
        "message": "Job queued",
        "download_url": None,
        "preview_url": None,
        "error": None,
        "request": {
            "city": request.city,
            "country": request.country,
            "theme": request.theme,
            "distance": request.distance
        }
    }

    # Queue background task
    background_tasks.add_task(
        process_poster_job,
        job_id,
        request.city,
        request.country,
        request.theme,
        request.distance
    )

    estimated_time = estimate_generation_time(request.distance)
    logger.debug(f"Job {job_id} queued, estimated time: {estimated_time}s")

    return PosterCreateResponse(
        job_id=job_id,
        status="queued",
        estimated_time=estimated_time
    )


@router.get("/{job_id}", response_model=JobStatus)
async def get_poster_status(job_id: str) -> JobStatus:
    """
    Get the status of a poster generation job.

    Args:
        job_id: UUID of the job

    Returns:
        Current job status
    """
    if job_id not in jobs:
        logger.warning(f"Job not found: {job_id}")
        raise HTTPException(status_code=404, detail="Job not found")

    job = jobs[job_id]
    return JobStatus(
        job_id=job_id,
        status=job["status"],
        progress=job["progress"],
        current_step=job["current_step"],
        message=job["message"],
        download_url=job["download_url"],
        preview_url=job["preview_url"],
        error=job["error"]
    )


@router.get("/{job_id}/download")
async def download_poster(job_id: str) -> FileResponse:
    """
    Download the generated poster.

    Args:
        job_id: UUID of the completed job

    Returns:
        PNG file response
    """
    if job_id not in jobs:
        logger.warning(f"Download requested for unknown job: {job_id}")
        raise HTTPException(status_code=404, detail="Job not found")

    job = jobs[job_id]
    if job["status"] != "completed":
        logger.warning(f"Download requested for incomplete job: {job_id} (status={job['status']})")
        raise HTTPException(status_code=400, detail="Poster not ready yet")

    file_path = settings.STATIC_DIR / f"{job_id}.png"
    if not file_path.exists():
        logger.error(f"Poster file missing for completed job: {job_id}")
        raise HTTPException(status_code=404, detail="Poster file not found")

    # Sanitize city name for filename
    city = settings.sanitize_filename(job["request"]["city"])
    theme = settings.sanitize_filename(job["request"]["theme"])

    logger.info(f"Serving poster download for job {job_id}")
    return FileResponse(
        path=file_path,
        filename=f"{city}_{theme}_poster.png",
        media_type="image/png"
    )


async def process_poster_job(
    job_id: str,
    city: str,
    country: str,
    theme_name: str,
    distance: int
) -> None:
    """
    Background task to process poster generation.

    Args:
        job_id: UUID of the job
        city: City name
        country: Country name
        theme_name: Name of the theme to use
        distance: Map radius in meters
    """
    from src.maptoposter.core.geocoding import get_coordinates
    from src.maptoposter.core.poster_generator import (
        PosterGenerator,
        PosterRequest,
        GenerationProgress,
    )

    logger.info(f"Starting job {job_id}: {city}, {country} with theme {theme_name}")

    def update_progress(step: str, progress: float, message: str) -> None:
        jobs[job_id].update({
            "status": "processing",
            "progress": progress,
            "current_step": step,
            "message": message
        })
        logger.debug(f"Job {job_id} progress: {step} - {progress:.0%}")

    try:
        # Step 1: Geocoding
        update_progress("geocoding", 0.10, f"Looking up {city}, {country}...")

        try:
            coordinates = get_coordinates(city, country)
            logger.info(f"Job {job_id}: Geocoded to {coordinates}")
        except (GeocodingError, APITimeoutError) as e:
            logger.error(f"Job {job_id} failed during geocoding: {e}")
            jobs[job_id].update({
                "status": "failed",
                "error": str(e)
            })
            return

        # Step 2: Load theme
        update_progress("loading_theme", 0.15, f"Loading {theme_name} theme...")
        try:
            theme = load_theme(theme_name)
        except ThemeNotFoundError as e:
            logger.error(f"Job {job_id} failed: theme not found: {e}")
            jobs[job_id].update({
                "status": "failed",
                "error": str(e)
            })
            return

        # Step 3: Generate poster
        try:
            generator = PosterGenerator(theme)
            request = PosterRequest(
                city=city,
                country=country,
                theme_name=theme_name,
                distance=distance,
                dpi=settings.OUTPUT_DPI
            )
        except InvalidDistanceError as e:
            logger.error(f"Job {job_id} failed: invalid distance: {e}")
            jobs[job_id].update({
                "status": "failed",
                "error": str(e)
            })
            return

        def progress_callback(prog: GenerationProgress) -> None:
            # Map generator progress (0.2-1.0) to overall progress (0.15-0.95)
            overall = 0.15 + (prog.progress * 0.80)
            update_progress(prog.step, overall, prog.message)

        try:
            buffer = generator.generate(request, coordinates, progress_callback)
        except DataFetchError as e:
            logger.error(f"Job {job_id} failed during generation: {e}")
            jobs[job_id].update({
                "status": "failed",
                "error": str(e)
            })
            return

        # Step 4: Save file
        update_progress("saving", 0.95, "Saving poster...")
        file_path = settings.STATIC_DIR / f"{job_id}.png"
        with open(file_path, 'wb') as f:
            f.write(buffer.getvalue())

        logger.info(f"Job {job_id} completed successfully, saved to {file_path}")

        # Check for warnings from generation
        if generator.warnings:
            logger.warning(f"Job {job_id} completed with warnings: {generator.warnings}")

        # Mark complete
        jobs[job_id].update({
            "status": "completed",
            "progress": 1.0,
            "current_step": "completed",
            "message": "Poster generation complete!",
            "download_url": f"/api/posters/{job_id}/download"
        })

    except Exception as e:
        logger.exception(f"Job {job_id} failed with unexpected error: {e}")
        jobs[job_id].update({
            "status": "failed",
            "error": f"Unexpected error: {str(e)}"
        })
