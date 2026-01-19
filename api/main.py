"""FastAPI application for MapToPoster online generator."""

from fastapi import FastAPI
from fastapi.staticfiles import StaticFiles
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import FileResponse
from pathlib import Path

from api.routes import themes, posters, jobs

# Get the base directory
BASE_DIR = Path(__file__).parent.parent

app = FastAPI(
    title="MapToPoster API",
    description="Generate beautiful minimalist city map posters",
    version="2.0.0",
    docs_url="/api/docs",
    redoc_url="/api/redoc"
)

# CORS middleware
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # Configure for production
    allow_credentials=True,
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


@app.get("/")
async def serve_frontend():
    """Serve the frontend HTML."""
    return FileResponse(frontend_dir / "index.html")


@app.get("/css/{filename}")
async def serve_css(filename: str):
    """Serve CSS files."""
    return FileResponse(frontend_dir / "css" / filename, media_type="text/css")


@app.get("/js/{filename}")
async def serve_js(filename: str):
    """Serve JavaScript files."""
    return FileResponse(frontend_dir / "js" / filename, media_type="application/javascript")


@app.get("/health")
async def health_check():
    """Health check endpoint."""
    return {"status": "healthy", "version": "2.0.0"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
