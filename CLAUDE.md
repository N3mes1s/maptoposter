# CLAUDE.md - AI Assistant Guide for MapToPoster

This document provides essential context for AI assistants working with this codebase.

## Project Overview

**Purpose**: Generate beautiful, minimalist map posters for any city in the world with customizable themes and zoom levels.

**Author**: Ankur Gupta
**License**: MIT
**Language**: Rust
**Version**: 2.0.0

## Directory Structure

```
maptoposter/
├── maptoposter-rs/             # Rust server application
│   ├── Cargo.toml              # Rust dependencies
│   ├── Cargo.lock              # Locked dependency versions
│   └── src/
│       ├── main.rs             # Entry point, Axum routes
│       ├── config.rs           # Settings from env vars
│       ├── error.rs            # Custom error types
│       ├── api/
│       │   ├── mod.rs
│       │   ├── handlers/
│       │   │   ├── posters.rs  # POST /api/posters, job processing
│       │   │   ├── jobs.rs     # SSE progress streaming
│       │   │   └── themes.rs   # GET /api/themes
│       │   ├── models.rs       # Request/response structs
│       │   └── state.rs        # AppState, job storage, map cache
│       ├── core/
│       │   ├── geocoding.rs    # Nominatim geocoding
│       │   ├── osm_client.rs   # Overpass API with fallback mirrors
│       │   ├── poster_generator.rs  # Main generation logic
│       │   └── progress.rs     # Progress callback types
│       ├── rendering/
│       │   ├── canvas.rs       # tiny-skia rendering, coordinate transform
│       │   ├── gradients.rs    # Fade overlays
│       │   └── road_styles.rs  # Road width/color mapping
│       └── themes/
│           └── loader.rs       # Theme JSON loading
├── frontend/                   # Web UI
│   ├── index.html
│   ├── css/styles.css
│   └── js/
│       ├── app.js              # Main application logic
│       └── api.js              # API client with SSE
├── themes/                     # 35 JSON theme configuration files
├── fonts/                      # Roboto font files (Bold, Regular, Light)
├── static/                     # Generated posters (served by API)
├── posters/                    # Example posters
├── Dockerfile                  # Multi-stage Rust build
├── docker-compose.yml          # Production compose config
├── railway.toml                # Railway deployment config
└── README.md                   # User documentation
```

## Quick Start Commands

### Using Docker (Recommended)

```bash
docker compose up
# Open http://localhost:8000
```

### Building from Source

```bash
cd maptoposter-rs
cargo build --release

# Copy assets
cp -r ../themes ../fonts ../frontend .

# Run
./target/release/maptoposter-rs
```

### Development

```bash
cd maptoposter-rs

# Build debug
cargo build

# Run with hot reload (requires cargo-watch)
cargo watch -x run

# Run tests
cargo test

# Check code
cargo clippy

# Format code
cargo fmt
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Serve frontend |
| `GET` | `/health` | Health check |
| `GET` | `/api/themes` | List all 35 themes |
| `GET` | `/api/themes/:name` | Get theme details |
| `POST` | `/api/posters` | Create poster job |
| `GET` | `/api/posters/:id` | Get job status |
| `GET` | `/api/posters/:id/stream` | SSE progress stream |
| `GET` | `/api/posters/:id/download` | Download poster PNG |
| `POST` | `/api/posters/:id/rerender` | Re-render with cached data |

### POST /api/posters Request

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
  "message": "Rendering road network...",
  "download_url": "/api/posters/{job_id}/download"
}
```

### SSE Progress Events

```
event: progress
data: {"job_id":"...","status":"processing","percent":15,"step":"fetching_streets","message":"Fetching street network..."}

event: completed
data: {"job_id":"...","status":"completed","percent":100,"step":"completed","message":"Poster generated successfully","download_url":"..."}

event: error
data: {"message":"Generation failed: ..."}
```

## Key Dependencies (Cargo.toml)

| Crate | Purpose |
|-------|---------|
| `axum` | HTTP framework |
| `tokio` | Async runtime |
| `reqwest` | HTTP client for Nominatim/Overpass |
| `tiny-skia` | 2D rendering (PNG output) |
| `serde` | JSON serialization |
| `tracing` | Logging |
| `uuid` | Job IDs |

## Code Architecture

### Data Flow

```
API Request → Geocoding (Nominatim) → Data Fetching (Overpass) → Rendering (tiny-skia) → PNG
```

### Key Modules

| Module | Purpose |
|--------|---------|
| `api/handlers/posters.rs` | Job creation, background processing |
| `api/handlers/jobs.rs` | SSE streaming |
| `api/state.rs` | AppState, job storage, map data cache |
| `core/poster_generator.rs` | Main generation pipeline |
| `core/osm_client.rs` | Overpass API with mirror fallback |
| `core/geocoding.rs` | Nominatim geocoding |
| `rendering/canvas.rs` | Coordinate transform, drawing |

### Rendering Layers (z-order)

```
z=6   Text labels (city, country, coordinates)
z=5   Gradient fades (top & bottom)
z=4   Roads (sorted by highway type)
z=2   Parks (green polygons)
z=1   Water (blue polygons)
z=0   Background color
```

### Key Structs

```rust
// Poster request parameters
pub struct PosterRequest {
    pub city: String,
    pub country: String,
    pub theme_name: String,
    pub distance: u32,
    pub dpi: u32,
}

// Job state stored in AppState
pub struct Job {
    pub id: Uuid,
    pub status: JobStatus,
    pub progress: f32,
    pub current_step: Option<String>,
    pub message: Option<String>,
    pub error: Option<String>,
    pub output_path: Option<String>,
}

// Cached map data for re-rendering
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

// Progress updates
pub struct GenerationProgress {
    pub step: String,
    pub progress: f32,
    pub message: String,
}
```

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8000` | Server port |
| `RUST_LOG` | `info` | Log level |
| `THEMES_DIR` | `/app/themes` | Theme JSON directory |
| `FONTS_DIR` | `/app/fonts` | Font files directory |
| `STATIC_DIR` | `/app/static` | Generated posters |
| `FRONTEND_DIR` | `/app/frontend` | Frontend files |
| `MAX_DISTANCE` | `20000` | Maximum radius (meters) |
| `MIN_DISTANCE` | `2000` | Minimum radius (meters) |
| `DEFAULT_DISTANCE` | `15000` | Default radius |
| `OSM_TIMEOUT` | `120` | Overpass API timeout (seconds) |
| `NOMINATIM_TIMEOUT` | `10` | Geocoding timeout (seconds) |

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

### Available Themes (35 total)

Classic: `feature_based`, `noir`, `japanese_ink`, `blueprint`
Warm: `sunset`, `warm_beige`, `terracotta`, `autumn`, `copper_patina`, `desert`, `coral`
Cool: `ocean`, `midnight_blue`, `arctic`, `sapphire`
Nature: `forest`, `sage`, `mint`, `emerald`
Modern: `neon_cyberpunk`, `neon_pink`, `electric_blue`, `pastel_dream`
Monochrome: `monochrome_blue`, `sepia`, `concrete`, `platinum`, `gradient_roads`, `contrast_zones`
Luxury: `rose_gold`, `gold`, `ruby`, `mocha`, `lavender`, `vintage_paper`

## Road Hierarchy

| Highway Type | Line Width |
|--------------|------------|
| motorway, motorway_link | 1.2 |
| trunk, primary, primary_link | 1.0 |
| secondary, secondary_link | 0.8 |
| tertiary, tertiary_link | 0.6 |
| residential, living_street | 0.4 |
| service, unclassified | 0.3 |

## Error Handling

```rust
// Custom error types in error.rs
pub enum AppError {
    JobNotFound(String),
    ThemeNotFound(String),
    InvalidDistance(String),
    Geocoding(String),
    DataFetch(String),
    Rendering(String),
    Internal(String),
}

// All errors implement IntoResponse for Axum
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Returns JSON: {"error": "...", "message": "...", "detail": "..."}
    }
}
```

## Job Processing

Jobs run in background tasks with:
- 3-minute timeout for full generation
- 30-second timeout for re-renders
- Panic handling (caught and marked as failed)
- Timeout handling (marked as failed with message)

```rust
// In posters.rs
tokio::spawn(async move {
    let result = tokio::time::timeout(
        job_timeout,
        AssertUnwindSafe(process_poster_job(...)).catch_unwind()
    ).await;

    match result {
        Ok(Ok(())) => { /* success */ }
        Ok(Err(_panic)) => { state.fail_job(job_id, "crashed") }
        Err(_timeout) => { state.fail_job(job_id, "timed out") }
    }
});
```

## Overpass API

Uses multiple mirrors with fallback:
1. `overpass-api.de` (main)
2. `maps.mail.ru` (fast)
3. `overpass.kumi.systems` (backup)

Queries use JSON output with configurable timeout.

## Performance Notes

| Metric | Value |
|--------|-------|
| Memory (idle) | ~50 MB |
| Memory (generating) | ~200 MB |
| Initial generation | 15-30 seconds |
| Theme re-render | ~3 seconds |
| Output size | 3-10 MB PNG |

## Development Guidelines

### Adding a New API Endpoint

1. Add handler in `api/handlers/`
2. Add route in `main.rs`
3. Add request/response types in `api/models.rs`

### Adding a New Theme

1. Create `themes/my_theme.json`
2. Theme is automatically available

### Modifying Rendering

1. Canvas operations in `rendering/canvas.rs`
2. Road styling in `rendering/road_styles.rs`
3. Gradient effects in `rendering/gradients.rs`

### Adding New Map Features

1. Add Overpass query in `core/osm_client.rs`
2. Add rendering in `core/poster_generator.rs`
3. Add theme color in theme JSON

## Important Notes for AI Assistants

1. **Rust project**: Use `cargo build`, `cargo test`, `cargo clippy`
2. **Async code**: Everything uses Tokio async runtime
3. **Error handling**: Use `?` operator with `Result<T, AppError>`
4. **Job state**: Jobs stored in `HashMap<Uuid, Job>` in `AppState`
5. **Map caching**: Cached data allows instant theme switching
6. **SSE streaming**: Progress updates via `axum::response::sse`
7. **Fonts**: Roboto fonts required in `fonts/` directory
8. **Themes**: JSON files in `themes/` directory (35 total)
9. **Static files**: Generated PNGs served from `static/`
10. **Timeouts**: Jobs have 3-minute timeout, re-renders 30 seconds

## File Modification Guidelines

| Change Type | Location |
|-------------|----------|
| API routes | `main.rs` |
| Request handlers | `api/handlers/` |
| Data models | `api/models.rs`, `api/state.rs` |
| Business logic | `core/poster_generator.rs` |
| Map data fetching | `core/osm_client.rs` |
| Rendering | `rendering/canvas.rs` |
| Configuration | `config.rs` |
| New themes | `themes/*.json` |
| Frontend | `frontend/` |

## Distance Guide

| Distance | Best for |
|----------|----------|
| 4000-6000m | Small/dense cities (Venice, Amsterdam center) |
| 8000-12000m | Medium cities (Paris, Barcelona) |
| 15000-20000m | Large metros (Tokyo, Mumbai) |
