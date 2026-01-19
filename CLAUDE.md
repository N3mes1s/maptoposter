# CLAUDE.md - AI Assistant Guide for City Map Poster Generator

This document provides essential context for AI assistants working with this codebase.

## Project Overview

**Purpose**: Generate beautiful, minimalist map posters for any city in the world with customizable themes and zoom levels. Available as both a CLI tool and a web application.

**Author**: Ankur Gupta
**License**: MIT
**Language**: Python 3.10+
**Version**: 2.0.0
**Package Manager**: UV (fast Python package manager)

## Directory Structure

```
maptoposter/
├── api/                        # FastAPI backend
│   ├── __init__.py
│   ├── main.py                 # FastAPI app, routes, static file serving
│   ├── routes/
│   │   ├── themes.py           # GET /api/themes endpoints
│   │   ├── posters.py          # POST /api/posters, job management
│   │   ├── jobs.py             # Job status endpoints
│   │   └── locations.py        # GET /api/locations/search (Nominatim autocomplete)
│   ├── schemas/
│   │   └── poster.py           # Pydantic request/response models
│   └── workers/                # Background job processing (placeholder)
├── src/maptoposter/            # Core library (refactored modules)
│   ├── __init__.py
│   ├── config.py               # Centralized settings with env vars and validation
│   ├── exceptions.py           # Custom exception classes with details
│   ├── logging_config.py       # Logging setup and configuration
│   ├── core/
│   │   ├── geocoding.py        # Nominatim geocoding functions
│   │   └── poster_generator.py # PosterGenerator class, PosterRequest
│   ├── rendering/
│   │   ├── gradients.py        # Gradient fade overlays
│   │   ├── road_styles.py      # Road color/width mapping
│   │   └── typography.py       # Font loading and text rendering
│   └── themes/
│       └── loader.py           # Theme JSON loading utilities
├── tests/                      # Pytest test suite
│   ├── conftest.py             # Shared fixtures
│   ├── test_config.py          # Configuration tests
│   ├── test_exceptions.py      # Exception tests
│   ├── test_themes.py          # Theme loading tests
│   └── test_poster_generator.py # Generator tests
├── frontend/                   # Web UI
│   ├── index.html              # Main HTML page
│   ├── css/styles.css          # Styling
│   └── js/
│       ├── app.js              # Main application logic
│       └── api.js              # API client functions
├── themes/                     # 17 JSON theme configuration files
├── fonts/                      # Roboto font files (Bold, Regular, Light)
├── static/                     # Generated posters (served by API)
├── posters/                    # CLI-generated poster output
├── cli.py                      # Command-line interface wrapper
├── create_map_poster.py        # Legacy single-file script (~470 lines)
├── test_api.py                 # API integration tests
├── test_generation.py          # Poster generation tests
├── pyproject.toml              # Project metadata and dependencies (UV/PEP 621)
├── uv.lock                     # Locked dependencies (committed to git)
├── requirements.txt            # Legacy pip requirements (deprecated)
├── Dockerfile                  # Multi-stage Docker build with UV
├── docker-compose.yml          # Production compose config
├── docker-compose.dev.yml      # Development with hot reload
├── .env.example                # Environment variable template
├── README.md                   # User documentation
└── LICENSE                     # MIT License
```

## Quick Start Commands

### Prerequisites

Install UV (fast Python package manager):
```bash
# macOS/Linux
curl -LsSf https://astral.sh/uv/install.sh | sh

# Or with pip
pip install uv

# Or with Homebrew
brew install uv
```

### Setup and Run

```bash
# Clone and enter project
cd maptoposter

# Install dependencies (creates .venv automatically)
uv sync

# Generate a poster (basic)
uv run python cli.py --city "New York" --country "USA"

# Generate with specific theme and zoom
uv run python cli.py -c "Tokyo" -C "Japan" -t japanese_ink -d 15000

# List all available themes
uv run python cli.py --list-themes

# Or use the installed command
uv run maptoposter --city "Paris" --country "France"
```

### Web Application

```bash
# Run API server
uv run uvicorn api.main:app --reload --port 8000

# Or with Docker
docker compose up

# Open browser to http://localhost:8000
```

### Running Tests

```bash
# Run all tests
uv run pytest

# Start API server first for integration tests, then:
uv run python test_api.py

# Test generation directly
uv run python test_generation.py
```

### Development

```bash
# Install with dev dependencies
uv sync --all-extras

# Run linter
uv run ruff check .

# Format code
uv run ruff format .

# Add a new dependency
uv add package-name

# Add a dev dependency
uv add --dev package-name

# Update all dependencies
uv lock --upgrade
uv sync
```

## UV Package Management

### Key Files

| File | Purpose |
|------|---------|
| `pyproject.toml` | Project metadata, dependencies, tool configs |
| `uv.lock` | Locked dependency versions (commit this!) |
| `.venv/` | Virtual environment (gitignored) |
| `requirements.txt` | Legacy file, kept for compatibility |

### Common UV Commands

```bash
# Install/sync dependencies
uv sync                      # Install all deps from lock file
uv sync --frozen             # Strict: fail if lock file outdated
uv sync --all-extras         # Include optional deps (redis, dev)

# Run commands in the environment
uv run python script.py      # Run Python script
uv run pytest                # Run pytest
uv run uvicorn ...           # Run uvicorn

# Manage dependencies
uv add fastapi               # Add to [project.dependencies]
uv add --dev pytest          # Add to [tool.uv.dev-dependencies]
uv add --optional redis rq   # Add to [project.optional-dependencies.redis]
uv remove package            # Remove a dependency

# Lock file management
uv lock                      # Update lock file
uv lock --upgrade            # Upgrade all packages
uv lock --upgrade-package X  # Upgrade specific package
```

### Optional Dependency Groups

```bash
# Install with Redis support
uv sync --extra redis

# Install with dev tools (pytest, ruff, httpx)
uv sync --group dev

# Install everything
uv sync --all-extras
```

## CLI Arguments

| Argument | Short | Required | Default | Description |
|----------|-------|----------|---------|-------------|
| `--city` | `-c` | Yes | - | City name |
| `--country` | `-C` | Yes | - | Country name |
| `--theme` | `-t` | No | feature_based | Theme name from themes/ |
| `--distance` | `-d` | No | 15000 | Map radius in meters |
| `--list-themes` | - | No | - | List available themes |

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Serve frontend |
| `GET` | `/health` | Basic health check |
| `GET` | `/health/ready` | Readiness check (verifies themes, fonts, directories) |
| `GET` | `/api/docs` | OpenAPI documentation |
| `GET` | `/api/themes` | List all themes |
| `GET` | `/api/themes/{name}` | Get theme details |
| `POST` | `/api/posters` | Create poster job |
| `GET` | `/api/posters/{job_id}` | Get job status |
| `GET` | `/api/posters/{job_id}/download` | Download completed poster |
| `GET` | `/api/locations/search?q=<query>` | Search for locations using Nominatim |

### POST /api/posters Request Body

```json
{
  "city": "Venice",
  "country": "Italy",
  "theme": "blueprint",
  "distance": 4000
}
```

### Job Status Response

```json
{
  "job_id": "uuid",
  "status": "queued|processing|completed|failed",
  "progress": 0.75,
  "current_step": "rendering_roads",
  "message": "Applying road hierarchy colors...",
  "download_url": "/api/posters/{job_id}/download",
  "error": null
}
```

## Key Dependencies

| Package | Purpose |
|---------|---------|
| `osmnx` | Fetch OpenStreetMap street networks and features |
| `matplotlib` | Render map visualizations and export PNG |
| `geopy` | Geocode city names to lat/lon via Nominatim |
| `geopandas` | Handle geographic data (water, parks) |
| `numpy` | Create gradient fade effects |
| `fastapi` | REST API framework |
| `uvicorn` | ASGI server |
| `pydantic` | Request/response validation |

## Code Architecture

### Data Flow

```
CLI/API Request → Geocoding (Nominatim) → Data Fetching (OSMnx) → Rendering (matplotlib) → PNG export
```

### Module Responsibilities

| Module | Purpose |
|--------|---------|
| `api/main.py` | FastAPI app setup, CORS, route mounting |
| `api/routes/posters.py` | Poster creation, job management, background tasks |
| `api/routes/themes.py` | Theme listing and retrieval |
| `src/maptoposter/config.py` | Centralized settings, env var support |
| `src/maptoposter/core/poster_generator.py` | `PosterGenerator` class with progress callbacks |
| `src/maptoposter/core/geocoding.py` | `get_coordinates()`, coordinate formatting |
| `src/maptoposter/rendering/road_styles.py` | Road color/width by OSM highway type |
| `src/maptoposter/rendering/gradients.py` | Top/bottom fade overlays |
| `src/maptoposter/themes/loader.py` | Theme JSON loading with fallbacks |

### Rendering Pipeline (z-order layers)

```
z=11  Text labels (city, country, coordinates, attribution)
z=10  Gradient fades (top & bottom transparency overlays)
z=3   Roads (via ox.plot_graph with hierarchy colors/widths)
z=2   Parks (green polygons)
z=1   Water (blue polygons)
z=0   Background color
```

### Key Classes

```python
# PosterRequest - generation parameters
@dataclass
class PosterRequest:
    city: str
    country: str
    theme_name: str = "feature_based"
    distance: int = 15000
    dpi: int = 300

# PosterGenerator - main generation logic
class PosterGenerator:
    def __init__(self, theme: Dict[str, Any])
    def generate(self, request, coordinates, progress_callback) -> io.BytesIO

# GenerationProgress - progress updates
@dataclass
class GenerationProgress:
    step: str           # e.g., "fetching_streets", "rendering_roads"
    progress: float     # 0.0 to 1.0
    message: str        # Human-readable status
```

## Environment Configuration

Copy `.env.example` to `.env` and customize:

```bash
# API settings
API_HOST=0.0.0.0
API_PORT=8000
REDIS_URL=redis://localhost:6379

# CORS settings
CORS_ORIGINS=*                    # Or comma-separated: http://localhost:3000,https://example.com
CORS_ALLOW_CREDENTIALS=false

# Map generation settings
MAX_DISTANCE=50000
MIN_DISTANCE=2000
DEFAULT_DISTANCE=15000
DEFAULT_THEME=feature_based

# Rate limiting (seconds)
NOMINATIM_DELAY=1.0
OSM_DELAY=0.5

# Timeout settings (seconds)
NOMINATIM_TIMEOUT=10.0
OSM_TIMEOUT=60.0

# Output settings
OUTPUT_DPI=300
PREVIEW_DPI=72

# Job settings
JOB_TTL_HOURS=24
MAX_CONCURRENT_JOBS=5

# Logging
LOG_LEVEL=INFO                    # DEBUG, INFO, WARNING, ERROR, CRITICAL
LOG_JSON=false                    # Set to true for production
```

## Theme System

### Theme JSON Structure

```json
{
  "name": "Theme Name",
  "description": "Theme description",
  "bg": "#FFFFFF",
  "text": "#000000",
  "gradient_color": "#FFFFFF",
  "water": "#C0C0C0",
  "parks": "#F0F0F0",
  "road_motorway": "#0A0A0A",
  "road_primary": "#1A1A1A",
  "road_secondary": "#2A2A2A",
  "road_tertiary": "#3A3A3A",
  "road_residential": "#4A4A4A",
  "road_default": "#3A3A3A"
}
```

### Available Themes (17 total)

`feature_based` (default), `noir`, `japanese_ink`, `neon_cyberpunk`, `blueprint`, `sunset`, `forest`, `ocean`, `midnight_blue`, `warm_beige`, `pastel_dream`, `contrast_zones`, `gradient_roads`, `monochrome_blue`, `autumn`, `copper_patina`, `terracotta`

### Adding a New Theme

1. Create `themes/my_theme.json` with the structure above
2. The theme is automatically available via CLI `--theme my_theme` or API

## Road Hierarchy (OSM Highway Types)

| Road Type | Line Width | Priority |
|-----------|------------|----------|
| motorway, motorway_link | 1.2 | Highest |
| trunk, primary, primary_link | 1.0 | High |
| secondary, secondary_link | 0.8 | Medium |
| tertiary, tertiary_link | 0.6 | Low |
| residential, living_street | 0.4 | Lowest |

## Code Conventions

### Style Patterns

- **snake_case** for functions and variables
- **PascalCase** for classes
- **UPPERCASE** for module constants
- **f-strings** for string formatting
- **Type hints** in new code (dataclasses, function signatures)
- **Docstrings** on all functions and classes

### Error Handling

```python
# Custom exceptions in src/maptoposter/exceptions.py
# All exceptions include message and details dict

class MapToPosterError(Exception):       # Base class
    def __init__(self, message: str, details: Optional[dict] = None)

class GeocodingError(MapToPosterError):  # Failed to geocode location
    def __init__(self, city: str, country: str, reason: str = "Location not found")

class ThemeNotFoundError(MapToPosterError):  # Theme doesn't exist
    def __init__(self, theme_name: str, available_themes: Optional[list] = None)

class DataFetchError(MapToPosterError):  # Failed to fetch OSM data
    def __init__(self, data_type: str, reason: str = "Unknown error")

class InvalidDistanceError(MapToPosterError):  # Distance out of range
    def __init__(self, distance: int, min_distance: int, max_distance: int)

class InvalidInputError(MapToPosterError):  # Invalid user input
class APITimeoutError(MapToPosterError):    # External API timeout
class PartialDataWarning(MapToPosterError): # Non-fatal, generation continues

# Graceful degradation for optional features (water, parks)
try:
    water = ox.features_from_point(point, tags={...}, dist=dist)
except Exception as e:
    logger.warning(f"Could not fetch water features: {e}")
    water = None  # Render without water layer
```

### Progress Feedback

```python
# CLI uses emoji indicators
print(f"✓ Success message")
print(f"✗ Error message")
print(f"⚠ Warning message")

# API uses progress callbacks
def progress_callback(prog: GenerationProgress):
    print(f"[{int(prog.progress * 100):3d}%] {prog.message}")
```

## Docker Usage

### Production

```bash
docker compose up -d
# API available at http://localhost:8000
```

### Development (with hot reload)

```bash
docker compose -f docker-compose.dev.yml up
```

### Build stages

- `base` - Python environment with GDAL dependencies and UV
- `api` - Production API server
- `dev` - Development with `--reload` and dev dependencies

## Common Development Tasks

### Add a New Dependency

```bash
# Runtime dependency
uv add package-name

# Dev-only dependency
uv add --dev package-name

# Optional dependency group
uv add --optional redis redis
```

### Add a New API Endpoint

1. Create route function in `api/routes/` appropriate file
2. Add to router with appropriate prefix
3. Define Pydantic schemas in `api/schemas/poster.py` if needed

### Add a New Map Layer (e.g., railways)

```python
# In src/maptoposter/core/poster_generator.py, in generate():
try:
    railways = ox.features_from_point(point, tags={'railway': 'rail'}, dist=dist)
except Exception:
    railways = None

# Plot before roads (zorder between parks and roads):
if railways is not None and not railways.empty:
    railways.plot(ax=ax, facecolor=self.theme.get('railway', '#888888'),
                  linewidth=0.5, zorder=2.5)
```

### Add a New Theme Property

1. Add to theme JSON: `"railway": "#FF0000"`
2. Use in code: `self.theme.get('railway', '#default')`
3. Update `api/schemas/poster.py` ThemeInfo if exposed via API

### Modify Typography Positioning

Text uses `transform=ax.transAxes` (0-1 normalized coordinates):
```
y=0.14   City name (spaced letters)
y=0.125  Decorative line
y=0.10   Country name
y=0.07   Coordinates
y=0.02   Attribution (bottom-right)
```

## Performance Notes

- **Large distances (>20km)**: Slow downloads, high memory usage
- **Quick previews**: Use `PREVIEW_DPI=72` env var or lower dpi parameter
- **Faster renders**: Use `network_type='drive'` instead of `'all'`
- **Rate limits**: Nominatim requires 1 second between requests
- **Job timeout**: Long-running jobs may timeout; consider smaller distances

## Testing

```bash
# Run pytest test suite (41 tests)
uv run pytest

# Run with verbose output
uv run pytest -v

# Run specific test file
uv run pytest tests/test_config.py

# Run with coverage
uv run pytest --cov=src/maptoposter

# API integration tests (requires running server on port 8002)
uv run python test_api.py

# Quick manual test with small distance
uv run python cli.py -c "Venice" -C "Italy" -t blueprint -d 4000
```

### Test Suite Structure

```
tests/
├── conftest.py             # Shared fixtures (sample_theme, coordinates, paths)
├── test_config.py          # Settings validation, distance bounds, filename sanitization
├── test_exceptions.py      # Exception class behavior and messages
├── test_themes.py          # Theme loading, validation, file integrity
└── test_poster_generator.py # PosterRequest validation, graceful degradation
```

### Test Coverage

- **Unit tests** (`tests/`): Config, exceptions, themes, poster generation
- **Integration tests** (`test_api.py`): Health, themes, frontend, API docs, poster generation
- **Manual tests** (`test_generation.py`): Direct poster generation without API

## Output Format

- **Format**: PNG at configurable DPI (default 300)
- **CLI Filename**: `{city}_{theme}_{YYYYMMDD_HHMMSS}.png`
- **API Filename**: `{job_id}.png` (served from static/)
- **Size**: 3-16 MB depending on map complexity

## Important Notes for AI Assistants

1. **Use UV for all Python operations**: Always use `uv run`, `uv add`, `uv sync`
2. **Lock file is committed**: `uv.lock` should be committed to version control
3. **Run tests after changes**: `uv run pytest` - 41 tests should pass
4. **Dual architecture**: Both legacy single-file (`create_map_poster.py`) and modular (`src/maptoposter/`) exist
5. **Prefer modular code**: New features should use the modular structure
6. **Use proper logging**: Import `from src.maptoposter.logging_config import get_logger`
7. **Use custom exceptions**: Import from `src.maptoposter.exceptions` for specific error types
8. **Validate inputs**: Use `settings.validate_distance()` and `settings.sanitize_filename()`
9. **API uses background tasks**: Poster generation is async via FastAPI BackgroundTasks
10. **In-memory job storage**: Current implementation stores jobs in dict; Redis support is scaffolded
11. **Theme validation**: Themes must exist in `themes/` directory before use
12. **Fonts required**: Roboto fonts must be present in `fonts/` directory
13. **API rate limits**: Nominatim has strict rate limits (1 req/sec)
14. **Graceful degradation**: Missing water/parks features log warnings but don't crash
15. **CORS configurable**: Set `CORS_ORIGINS` env var for production
16. **Static files**: Generated posters served from `static/` directory

## File Modification Guidelines

When modifying this codebase:

1. **For new features**: Add to `src/maptoposter/` modules, maintain separation of concerns
2. **For API changes**: Update routes in `api/routes/`, schemas in `api/schemas/`
3. **For new themes**: Create JSON file in `themes/` directory
4. **For CLI changes**: Modify `cli.py`
5. **For rendering changes**: Modify appropriate module in `src/maptoposter/rendering/`
6. **For configuration**: Add to `src/maptoposter/config.py` and `.env.example`
7. **For new dependencies**: Use `uv add package-name`, then commit updated `uv.lock`

## Distance Guide

| Distance | Best for |
|----------|----------|
| 4000-6000m | Small/dense cities (Venice, Amsterdam center) |
| 8000-12000m | Medium cities, focused downtown (Paris, Barcelona) |
| 15000-20000m | Large metros, full city view (Tokyo, Mumbai) |
