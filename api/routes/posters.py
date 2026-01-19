"""Poster generation API routes."""

import uuid
import json
from typing import Optional
from fastapi import APIRouter, HTTPException, BackgroundTasks
from fastapi.responses import FileResponse

from src.maptoposter.config import settings
from src.maptoposter.themes.loader import get_available_themes
from api.schemas.poster import PosterCreateRequest, PosterCreateResponse, JobStatus

router = APIRouter()

# In-memory job storage (replace with Redis in production)
jobs: dict = {}


def estimate_generation_time(distance: int) -> int:
    """Estimate generation time based on distance."""
    # Rough estimate: larger areas take longer
    if distance <= 5000:
        return 30
    elif distance <= 10000:
        return 45
    elif distance <= 20000:
        return 60
    else:
        return 90


@router.post("", response_model=PosterCreateResponse)
async def create_poster(request: PosterCreateRequest, background_tasks: BackgroundTasks):
    """Queue a new poster generation job."""
    # Validate theme exists
    available_themes = get_available_themes()
    if request.theme not in available_themes:
        raise HTTPException(
            status_code=400,
            detail=f"Theme '{request.theme}' not found. Available: {', '.join(available_themes)}"
        )

    # Create job
    job_id = str(uuid.uuid4())

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

    return PosterCreateResponse(
        job_id=job_id,
        status="queued",
        estimated_time=estimate_generation_time(request.distance)
    )


@router.get("/{job_id}", response_model=JobStatus)
async def get_poster_status(job_id: str):
    """Get the status of a poster generation job."""
    if job_id not in jobs:
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
async def download_poster(job_id: str):
    """Download the generated poster."""
    if job_id not in jobs:
        raise HTTPException(status_code=404, detail="Job not found")

    job = jobs[job_id]
    if job["status"] != "completed":
        raise HTTPException(status_code=400, detail="Poster not ready yet")

    file_path = settings.STATIC_DIR / f"{job_id}.png"
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="Poster file not found")

    # Get city name for filename
    city = job["request"]["city"].lower().replace(' ', '_')
    theme = job["request"]["theme"]

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
):
    """Background task to process poster generation."""
    from src.maptoposter.core.geocoding import get_coordinates
    from src.maptoposter.core.poster_generator import PosterGenerator, PosterRequest
    from src.maptoposter.themes.loader import load_theme
    from src.maptoposter.exceptions import GeocodingError, DataFetchError

    def update_progress(step: str, progress: float, message: str):
        jobs[job_id].update({
            "status": "processing",
            "progress": progress,
            "current_step": step,
            "message": message
        })

    try:
        # Step 1: Geocoding
        update_progress("geocoding", 0.10, f"Looking up {city}, {country}...")

        try:
            coordinates = get_coordinates(city, country)
        except GeocodingError as e:
            jobs[job_id].update({
                "status": "failed",
                "error": str(e)
            })
            return

        # Step 2: Load theme
        update_progress("loading_theme", 0.15, f"Loading {theme_name} theme...")
        theme = load_theme(theme_name)

        # Step 3: Generate poster
        generator = PosterGenerator(theme)
        request = PosterRequest(
            city=city,
            country=country,
            theme_name=theme_name,
            distance=distance,
            dpi=settings.OUTPUT_DPI
        )

        def progress_callback(prog):
            # Map generator progress (0.2-1.0) to overall progress (0.15-0.95)
            overall = 0.15 + (prog.progress * 0.80)
            update_progress(prog.step, overall, prog.message)

        buffer = generator.generate(request, coordinates, progress_callback)

        # Step 4: Save file
        update_progress("saving", 0.95, "Saving poster...")
        file_path = settings.STATIC_DIR / f"{job_id}.png"
        with open(file_path, 'wb') as f:
            f.write(buffer.getvalue())

        # Mark complete
        jobs[job_id].update({
            "status": "completed",
            "progress": 1.0,
            "current_step": "completed",
            "message": "Poster generation complete!",
            "download_url": f"/api/posters/{job_id}/download"
        })

    except Exception as e:
        jobs[job_id].update({
            "status": "failed",
            "error": str(e)
        })
