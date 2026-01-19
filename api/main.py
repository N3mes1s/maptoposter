"""FastAPI application for MapToPoster online generator."""

import os
from contextlib import asynccontextmanager
from pathlib import Path
from typing import AsyncGenerator

from fastapi import FastAPI, HTTPException
from fastapi.staticfiles import StaticFiles
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import FileResponse

from api.routes import themes, posters, jobs
from src.maptoposter.config import settings
from src.maptoposter.logging_config import setup_logging, get_logger

# Initialize logging
setup_logging(level=settings.LOG_LEVEL, json_format=settings.LOG_JSON)
logger = get_logger("api")

# Get the base directory
BASE_DIR = Path(__file__).parent.parent


@asynccontextmanager
async def lifespan(app: FastAPI) -> AsyncGenerator[None, None]:
    """Application lifespan handler for startup and shutdown events."""
    # Startup
    logger.info("Starting MapToPoster API")
    logger.info(f"CORS origins: {settings.CORS_ORIGINS}")
    logger.info(f"Log level: {settings.LOG_LEVEL}")

    # Verify required directories exist
    if not settings.THEMES_DIR.exists():
        logger.error(f"Themes directory not found: {settings.THEMES_DIR}")
    if not settings.FONTS_DIR.exists():
        logger.error(f"Fonts directory not found: {settings.FONTS_DIR}")

    settings.STATIC_DIR.mkdir(exist_ok=True)
    settings.POSTERS_DIR.mkdir(exist_ok=True)

    logger.info("MapToPoster API started successfully")

    yield

    # Shutdown
    logger.info("Shutting down MapToPoster API")


app = FastAPI(
    title="MapToPoster API",
    description="Generate beautiful minimalist city map posters",
    version="2.0.0",
    docs_url="/api/docs",
    redoc_url="/api/redoc",
    lifespan=lifespan
)

# CORS middleware - configurable via environment
app.add_middleware(
    CORSMiddleware,
    allow_origins=settings.CORS_ORIGINS,
    allow_credentials=settings.CORS_ALLOW_CREDENTIALS if settings.CORS_ORIGINS != ["*"] else False,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Mount static files for generated posters
static_dir = BASE_DIR / "static"
static_dir.mkdir(exist_ok=True)
app.mount("/static", StaticFiles(directory=str(static_dir)), name="static")

# API routes
app.include_router(themes.router, prefix="/api/themes", tags=["themes"])
app.include_router(posters.router, prefix="/api/posters", tags=["posters"])
app.include_router(jobs.router, prefix="/api/jobs", tags=["jobs"])


# Serve frontend
frontend_dir = BASE_DIR / "frontend"


def _validate_filename(filename: str) -> str:
    """
    Validate filename to prevent path traversal attacks.

    Args:
        filename: The filename to validate

    Returns:
        Sanitized filename

    Raises:
        HTTPException: If filename is invalid
    """
    # Get just the base filename, removing any path components
    safe_name = os.path.basename(filename)

    # Check for empty or dangerous names
    if not safe_name or safe_name.startswith('.'):
        raise HTTPException(status_code=400, detail="Invalid filename")

    return safe_name


@app.get("/")
async def serve_frontend() -> FileResponse:
    """Serve the frontend HTML."""
    index_path = frontend_dir / "index.html"
    if not index_path.exists():
        logger.error(f"Frontend index.html not found at {index_path}")
        raise HTTPException(status_code=404, detail="Frontend not found")
    return FileResponse(index_path)


@app.get("/css/{filename}")
async def serve_css(filename: str) -> FileResponse:
    """Serve CSS files."""
    safe_filename = _validate_filename(filename)
    file_path = frontend_dir / "css" / safe_filename
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="CSS file not found")
    return FileResponse(file_path, media_type="text/css")


@app.get("/js/{filename}")
async def serve_js(filename: str) -> FileResponse:
    """Serve JavaScript files."""
    safe_filename = _validate_filename(filename)
    file_path = frontend_dir / "js" / safe_filename
    if not file_path.exists():
        raise HTTPException(status_code=404, detail="JavaScript file not found")
    return FileResponse(file_path, media_type="application/javascript")


@app.get("/health")
async def health_check() -> dict:
    """
    Health check endpoint.

    Returns basic health status. For detailed checks, use /health/ready.
    """
    return {"status": "healthy", "version": "2.0.0"}


@app.get("/health/ready")
async def readiness_check() -> dict:
    """
    Readiness check endpoint.

    Verifies that all required resources are available.
    """
    issues = []

    # Check themes directory
    if not settings.THEMES_DIR.exists():
        issues.append("Themes directory not found")
    else:
        theme_count = len(list(settings.THEMES_DIR.glob("*.json")))
        if theme_count == 0:
            issues.append("No themes available")

    # Check fonts directory
    if not settings.FONTS_DIR.exists():
        issues.append("Fonts directory not found")

    # Check static directory is writable
    if not os.access(settings.STATIC_DIR, os.W_OK):
        issues.append("Static directory not writable")

    if issues:
        return {
            "status": "unhealthy",
            "version": "2.0.0",
            "issues": issues
        }

    return {
        "status": "ready",
        "version": "2.0.0",
        "themes_available": theme_count,
    }


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(
        app,
        host=settings.API_HOST,
        port=settings.API_PORT
    )
