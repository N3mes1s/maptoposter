# MapToPoster Rust Server

A memory-efficient Rust rewrite of the MapToPoster API server.

## Why Rust?

The original Python implementation using OSMnx, Matplotlib, and GeoPandas has significant memory overhead:

| Metric | Python | Rust |
|--------|--------|------|
| Memory (idle) | 150-300MB | 10-20MB |
| Memory (per job) | 300-800MB | 50-150MB |
| Docker image | 1-2GB | 50-100MB |
| Cold start | 3-5s | <100ms |

This rewrite addresses Railway deployment crashes by reducing memory footprint by 60-80%.

## Building

```bash
# Development build
cargo build

# Release build (optimized)
cargo build --release

# Run locally
cargo run

# Run with logging
RUST_LOG=info cargo run
```

## Docker

```bash
# Build image
docker build -t maptoposter-rs .

# Run container
docker run -p 8000:8000 \
  -v $(pwd)/../themes:/app/themes:ro \
  -v $(pwd)/../fonts:/app/fonts:ro \
  -v $(pwd)/../frontend:/app/frontend:ro \
  maptoposter-rs

# Using docker-compose
docker-compose up
```

## API Endpoints

All endpoints are compatible with the original Python API:

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/health` | Health check |
| `GET` | `/health/ready` | Readiness check |
| `GET` | `/api/themes` | List themes |
| `GET` | `/api/themes/{name}` | Get theme details |
| `GET` | `/api/locations/search?q=<query>` | Search locations |
| `POST` | `/api/posters` | Create poster job |
| `GET` | `/api/posters/{id}` | Get job status |
| `GET` | `/api/posters/{id}/download` | Download poster |
| `GET` | `/api/posters/{id}/stream` | SSE progress stream |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | 8000 | Server port |
| `RUST_LOG` | info | Log level |
| `THEMES_DIR` | themes | Themes directory |
| `FONTS_DIR` | fonts | Fonts directory |
| `STATIC_DIR` | static | Output directory |
| `FRONTEND_DIR` | frontend | Frontend directory |
| `DEFAULT_THEME` | feature_based | Default theme |
| `DEFAULT_DISTANCE` | 15000 | Default map distance (m) |
| `MIN_DISTANCE` | 2000 | Minimum distance (m) |
| `MAX_DISTANCE` | 50000 | Maximum distance (m) |
| `OUTPUT_DPI` | 300 | Output resolution |
| `NOMINATIM_TIMEOUT` | 10 | Geocoding timeout (s) |
| `OSM_TIMEOUT` | 60 | OSM API timeout (s) |
| `MAX_CONCURRENT_JOBS` | 5 | Max parallel jobs |
| `JOB_TTL_HOURS` | 24 | Job retention time |

## Architecture

```
src/
├── main.rs           # Axum server setup
├── config.rs         # Environment configuration
├── error.rs          # Error types
├── api/
│   ├── handlers/     # HTTP handlers
│   ├── models.rs     # Request/response types
│   └── state.rs      # Application state
├── core/
│   ├── geocoding.rs  # Nominatim client
│   ├── osm_client.rs # Overpass API client
│   └── poster_generator.rs
├── rendering/
│   ├── canvas.rs     # tiny-skia wrapper
│   ├── gradients.rs  # Fade effects
│   ├── road_styles.rs
│   └── typography.rs # fontdue text
└── themes/
    └── loader.rs     # Theme JSON loading
```

## Technology Stack

- **Web Framework**: Axum (lowest memory footprint)
- **Async Runtime**: Tokio
- **HTTP Client**: reqwest (for Nominatim/Overpass)
- **2D Rendering**: tiny-skia (pure Rust)
- **Font Rendering**: fontdue (pure Rust)
- **Geometry**: geo crate

## Credits

- Original Python implementation by Ankur Gupta
- Rust rewrite by Giuseppe Massaro
- Map data © OpenStreetMap contributors
