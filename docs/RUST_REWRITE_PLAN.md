# Rust Rewrite Implementation Plan for MapToPoster

## Executive Summary

This plan outlines a comprehensive strategy to rewrite the MapToPoster Python FastAPI server in Rust to address Railway deployment memory crashes. The current Python stack using OSMnx, Matplotlib, and GeoPandas is memory-intensive due to Python's GC overhead, NumPy/Pandas memory allocations, and matplotlib's figure management. A Rust implementation can reduce memory footprint by 60-80% while maintaining API compatibility.

---

## 1. Problem Analysis

### Current Memory Issues

| Component | Memory Impact |
|-----------|---------------|
| Python FastAPI idle | 150-300MB |
| OSMnx NetworkX Graph | 100-400MB per job |
| GeoPandas GeoDataFrame | 50-200MB per job |
| Matplotlib Figure | 50-150MB per job |
| io.BytesIO PNG buffer | 10-50MB per job |

**Total per job**: 300-800MB, causing Railway OOM crashes

### Expected Rust Improvements

| Metric | Python (Current) | Rust (Expected) |
|--------|------------------|-----------------|
| Cold start | 3-5s | <100ms |
| Memory idle | 150-300MB | 10-20MB |
| Memory per job | 300-800MB | 50-150MB |
| Generation time | 30-90s | 15-45s |
| Docker image | 1-2GB | 50-100MB |
| Concurrent jobs | 2-3 | 10-20 |

---

## 2. Technology Stack

### 2.1 Web Framework: Axum

**Why Axum over Actix-web**:
- Lower memory footprint (critical for Railway)
- Native Tokio integration
- Easier learning curve
- ~95% of Actix performance

```toml
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["cors", "fs", "trace"] }
```

### 2.2 OSM Data Fetching: Direct Overpass API

Replace OSMnx with direct Overpass QL queries:

```rust
// Equivalent to ox.graph_from_point()
let query = format!(r#"
    [out:json][timeout:90];
    (
      way["highway"~"motorway|trunk|primary|secondary|tertiary|residential"](around:{distance},{lat},{lon});
    );
    out body;
    >;
    out skel qt;
"#);
```

```toml
reqwest = { version = "0.12", features = ["json"] }
```

### 2.3 Geometry: GeoRust Ecosystem

```toml
geo = "0.28"
geo-types = "0.7"
geojson = "0.24"
proj = "0.27"  # Coordinate projections
```

**Type Mapping**:
| Python (GeoPandas) | Rust (geo) |
|--------------------|------------|
| Point | `geo::Point<f64>` |
| LineString | `geo::LineString<f64>` |
| Polygon | `geo::Polygon<f64>` |
| GeoDataFrame | `Vec<Feature>` |

### 2.4 Image Rendering: tiny-skia + fontdue

```toml
tiny-skia = "0.11"  # 2D rendering
png = "0.17"        # PNG encoding
fontdue = "0.9"     # Font rasterization
```

**Why this combination**:
- tiny-skia: ~200KB binary addition, pure Rust, fast
- fontdue: Fastest pure Rust font rasterizer
- Both are `no_std` compatible for minimal overhead

### 2.5 Geocoding: Direct Nominatim HTTP

Replace `geopy.geocoders.Nominatim` with direct HTTP:

```rust
let url = format!(
    "https://nominatim.openstreetmap.org/search?q={},{}&format=json",
    city, country
);
```

---

## 3. Architecture Design

### 3.1 Directory Structure

```
maptoposter-rs/
├── Cargo.toml
├── src/
│   ├── main.rs                    # Entry point, Axum setup
│   ├── lib.rs                     # Library exports
│   ├── config.rs                  # Settings (env vars)
│   ├── error.rs                   # Custom error types
│   │
│   ├── api/
│   │   ├── mod.rs
│   │   ├── handlers/
│   │   │   ├── mod.rs
│   │   │   ├── posters.rs         # POST /api/posters, GET status
│   │   │   ├── themes.rs          # GET /api/themes
│   │   │   ├── locations.rs       # GET /api/locations/search
│   │   │   ├── jobs.rs            # SSE streaming
│   │   │   └── health.rs          # Health checks
│   │   ├── models.rs              # Request/Response structs
│   │   └── state.rs               # AppState, job storage
│   │
│   ├── core/
│   │   ├── mod.rs
│   │   ├── geocoding.rs           # Nominatim client
│   │   ├── osm_client.rs          # Overpass API client
│   │   ├── poster_generator.rs    # Main generation logic
│   │   └── progress.rs            # Progress tracking
│   │
│   ├── rendering/
│   │   ├── mod.rs
│   │   ├── canvas.rs              # tiny-skia wrapper
│   │   ├── road_styles.rs         # Highway type -> style
│   │   ├── gradients.rs           # Gradient fade effects
│   │   └── typography.rs          # fontdue text rendering
│   │
│   └── themes/
│       ├── mod.rs
│       └── loader.rs              # JSON theme loading
│
├── themes/                        # Copy from Python project
├── fonts/                         # Copy from Python project
└── static/                        # Generated posters
```

### 3.2 Memory-Efficient Data Structures

```rust
// Minimal road segment representation
struct RoadSegment {
    points: Vec<(f64, f64)>,  // Coordinates
    highway_type: HighwayType, // Enum, not String
    width: f32,
    color: u32,               // RGBA as u32
}

// Enum for highway types (vs string comparisons)
#[derive(Clone, Copy)]
enum HighwayType {
    Motorway,
    Primary,
    Secondary,
    Tertiary,
    Residential,
    Default,
}
```

### 3.3 Concurrent Job Processing

```rust
use tokio::sync::{mpsc, RwLock};
use std::collections::HashMap;
use uuid::Uuid;

struct AppState {
    jobs: RwLock<HashMap<Uuid, JobState>>,
    job_sender: mpsc::Sender<JobRequest>,
}

// Background worker with bounded channel
async fn job_worker(
    mut receiver: mpsc::Receiver<JobRequest>,
    state: Arc<AppState>
) {
    while let Some(job) = receiver.recv().await {
        let result = generate_poster(&job).await;
        // Update job state
    }
}
```

---

## 4. API Endpoint Mapping

All existing frontend API calls must work unchanged:

| Endpoint | Python | Rust Handler |
|----------|--------|--------------|
| `GET /` | `serve_frontend()` | `ServeFile` |
| `GET /health` | `health_check()` | `health::check` |
| `GET /health/ready` | `readiness_check()` | `health::ready` |
| `GET /api/themes` | `list_themes()` | `themes::list` |
| `GET /api/themes/{name}` | `get_theme()` | `themes::get` |
| `POST /api/posters` | `create_poster()` | `posters::create` |
| `GET /api/posters/{id}` | `get_status()` | `posters::status` |
| `GET /api/posters/{id}/download` | `download_poster()` | `posters::download` |
| `GET /api/locations/search` | `search_locations()` | `locations::search` |

### Request/Response Models

```rust
#[derive(Deserialize)]
pub struct PosterCreateRequest {
    pub city: String,
    pub country: String,
    #[serde(default = "default_theme")]
    pub theme: String,
    #[serde(default = "default_distance")]
    pub distance: u32,
}

#[derive(Serialize)]
pub struct JobStatus {
    pub job_id: String,
    pub status: String,
    pub progress: f32,
    pub current_step: Option<String>,
    pub message: Option<String>,
    pub download_url: Option<String>,
    pub error: Option<String>,
}
```

---

## 5. Critical Implementation Details

### 5.1 Overpass API Query (Streets)

```rust
pub async fn fetch_street_network(
    center: (f64, f64),
    distance: u32,
) -> Result<Vec<RoadSegment>> {
    let query = format!(r#"
        [out:json][timeout:90];
        (
          way["highway"~"motorway|trunk|primary|secondary|tertiary|residential|living_street"](around:{distance},{lat},{lon});
        );
        out body;
        >;
        out skel qt;
    "#, distance = distance, lat = center.0, lon = center.1);

    let client = reqwest::Client::new();
    let response = client
        .post("https://overpass-api.de/api/interpreter")
        .body(query)
        .send()
        .await?;

    parse_overpass_response(response.json().await?)
}
```

### 5.2 Water/Parks Features Query

```rust
pub async fn fetch_water_features(
    center: (f64, f64),
    distance: u32,
) -> Result<Vec<Polygon<f64>>> {
    let query = format!(r#"
        [out:json][timeout:60];
        (
          way["natural"="water"](around:{distance},{lat},{lon});
          way["waterway"="riverbank"](around:{distance},{lat},{lon});
          relation["natural"="water"](around:{distance},{lat},{lon});
        );
        out body;
        >;
        out skel qt;
    "#, distance = distance, lat = center.0, lon = center.1);
    // ... fetch and parse
}
```

### 5.3 Rendering Pipeline

```rust
use tiny_skia::{Pixmap, Paint, Path, Stroke, Transform};

pub fn render_poster(
    roads: &[RoadSegment],
    water: &[Polygon<f64>],
    parks: &[Polygon<f64>],
    theme: &Theme,
    city: &str,
    country: &str,
    coords: (f64, f64),
) -> Result<Pixmap> {
    let width = 3600;  // 12 inches * 300 DPI
    let height = 4800; // 16 inches * 300 DPI
    let mut pixmap = Pixmap::new(width, height)?;

    // Fill background
    pixmap.fill(theme.bg_color());

    // Z-order: water -> parks -> roads -> gradients -> text
    render_water(&mut pixmap, water, theme)?;
    render_parks(&mut pixmap, parks, theme)?;
    render_roads(&mut pixmap, roads, theme)?;
    render_gradient_fade(&mut pixmap, theme, Location::Top)?;
    render_gradient_fade(&mut pixmap, theme, Location::Bottom)?;
    render_typography(&mut pixmap, city, country, coords, theme)?;

    Ok(pixmap)
}
```

### 5.4 Gradient Fade Effect

```rust
pub fn render_gradient_fade(
    pixmap: &mut Pixmap,
    theme: &Theme,
    location: Location,
) {
    let height = pixmap.height();
    let width = pixmap.width();
    let gradient_height = (height as f32 * 0.25) as u32;

    let rgb = theme.gradient_color_rgb();

    for y in 0..gradient_height {
        let alpha = match location {
            Location::Bottom => (255.0 * (1.0 - y as f32 / gradient_height as f32)) as u8,
            Location::Top => (255.0 * y as f32 / gradient_height as f32) as u8,
        };
        let color = tiny_skia::Color::from_rgba8(rgb.0, rgb.1, rgb.2, alpha);
        // Blend onto pixmap
    }
}
```

### 5.5 Typography with fontdue

```rust
use fontdue::{Font, FontSettings};

struct FontSet {
    bold: Font,
    regular: Font,
    light: Font,
}

fn render_text(
    pixmap: &mut Pixmap,
    text: &str,
    font: &Font,
    size: f32,
    color: Color,
    x: f32,
    y: f32,
) {
    for c in text.chars() {
        let (metrics, bitmap) = font.rasterize(c, size);
        // Blend bitmap onto pixmap at position
    }
}
```

---

## 6. Migration Strategy

### Recommended: Incremental Migration

**Phase 1: Core Infrastructure**
- Set up Axum server with health endpoints
- Implement theme loading (JSON parsing)
- Implement Nominatim geocoding client
- Implement location search endpoint

**Phase 2: Data Fetching**
- Implement Overpass API client for streets
- Implement water/parks feature fetching
- Parse Overpass JSON into Rust structs
- Add rate limiting and caching

**Phase 3: Rendering Engine**
- Implement tiny-skia canvas wrapper
- Port road rendering with highway styling
- Port water/parks polygon filling
- Implement gradient fade effects
- Implement typography with fontdue

**Phase 4: Job Management**
- Implement async job queue with Tokio
- Implement SSE progress streaming
- Implement job status tracking
- Add file serving for completed posters

**Phase 5: Integration & Testing**
- Full API compatibility testing
- Memory profiling and optimization
- Docker image creation
- Railway deployment testing

---

## 7. Docker Deployment

### Optimized Rust Dockerfile

```dockerfile
# Build stage
FROM rust:1.75-slim AS builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

# Cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src

# Build application
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/maptoposter-rs .
COPY themes ./themes
COPY fonts ./fonts
COPY frontend ./frontend

RUN mkdir -p static posters

ENV PORT=8000
EXPOSE ${PORT}

CMD ["./maptoposter-rs"]
```

**Expected Image Size**: ~50-80MB (vs ~1GB+ for Python)

---

## 8. Memory Optimization Strategies

### 8.1 Streaming Processing

```rust
// Process roads in chunks to avoid loading all into memory
async fn fetch_and_render_roads(
    pixmap: &mut Pixmap,
    center: (f64, f64),
    distance: u32,
) -> Result<()> {
    let mut stream = overpass_stream(center, distance).await?;

    while let Some(batch) = stream.next().await {
        let segments = parse_road_batch(batch?)?;
        render_roads_batch(pixmap, &segments)?;
        // segments dropped, memory freed
    }
    Ok(())
}
```

### 8.2 Object Pooling

```rust
struct PixmapPool {
    pool: Mutex<Vec<Pixmap>>,
    width: u32,
    height: u32,
}

impl PixmapPool {
    fn acquire(&self) -> PooledPixmap {
        self.pool.lock().pop()
            .unwrap_or_else(|| Pixmap::new(self.width, self.height).unwrap())
    }
}
```

### 8.3 Direct File Output

```rust
fn save_poster(pixmap: &Pixmap, path: &Path) -> Result<()> {
    pixmap.save_png(path)?;  // Direct to file, no buffer
    Ok(())
}
```

---

## 9. Dependencies (Cargo.toml)

```toml
[package]
name = "maptoposter-rs"
version = "2.0.0"
edition = "2021"

[dependencies]
# Web framework
axum = "0.7"
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.5", features = ["cors", "fs", "trace"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# HTTP client
reqwest = { version = "0.12", features = ["json"] }

# Geometry
geo = "0.28"
geo-types = "0.7"

# Rendering
tiny-skia = "0.11"
png = "0.17"
fontdue = "0.9"

# Utilities
uuid = { version = "1", features = ["v4", "serde"] }
thiserror = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
```

---

## 10. Risk Mitigation

### Rendering Fidelity
- Create visual regression test suite
- Compare outputs pixel-by-pixel
- Allow configurable tolerance

### Overpass API Rate Limits
- Implement request queuing with backoff
- Add caching layer
- Consider self-hosted Overpass for production

### Font Rendering Differences
- Use exact same Roboto font files
- Adjust sizing/positioning to match
- Document visual differences

---

## 11. Critical Files Reference

Files to port from Python:
- `src/maptoposter/core/poster_generator.py` - Core rendering logic
- `api/routes/posters.py` - API endpoint patterns
- `src/maptoposter/rendering/road_styles.py` - Highway classification
- `src/maptoposter/rendering/gradients.py` - Gradient algorithm
- `src/maptoposter/rendering/typography.py` - Text positioning

---

## 12. Success Criteria

1. **Memory**: Idle <50MB, per-job <200MB
2. **API**: 100% compatibility with existing frontend
3. **Docker**: Image size <100MB
4. **Performance**: Generation time comparable or faster
5. **Railway**: No OOM crashes under normal load
