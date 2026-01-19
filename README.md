# MapToPoster

Generate beautiful, minimalist map posters for any city in the world. Built with Rust for performance and memory efficiency.

<img src="posters/singapore_neon_cyberpunk_20260108_184503.png" width="250">
<img src="posters/dubai_midnight_blue_20260108_174920.png" width="250">

## Features

- **35 Themes**: From minimalist noir to vibrant neon cyberpunk
- **Any Location**: Generate posters for any city worldwide
- **Customizable Radius**: From 2km neighborhood views to 20km city-wide maps
- **High Resolution**: 300 DPI output ready for printing
- **Instant Theme Switch**: Change themes without re-fetching map data
- **Real-time Progress**: SSE-based progress streaming
- **Memory Efficient**: Rust server uses ~50MB idle, ~200MB during generation

## Live Demo

**[maptoposter-production.up.railway.app](https://maptoposter-production.up.railway.app)**

## Examples

| Country | City | Theme | Poster |
|:-------:|:----:|:-----:|:------:|
| USA | San Francisco | sunset | <img src="posters/san_francisco_sunset_20260108_184122.png" width="250"> |
| Spain | Barcelona | warm_beige | <img src="posters/barcelona_warm_beige_20260108_172924.png" width="250"> |
| Italy | Venice | blueprint | <img src="posters/venice_blueprint_20260108_165527.png" width="250"> |
| Japan | Tokyo | japanese_ink | <img src="posters/tokyo_japanese_ink_20260108_165830.png" width="250"> |
| India | Mumbai | contrast_zones | <img src="posters/mumbai_contrast_zones_20260108_170325.png" width="250"> |
| Morocco | Marrakech | terracotta | <img src="posters/marrakech_terracotta_20260108_180821.png" width="250"> |
| Singapore | Singapore | neon_cyberpunk | <img src="posters/singapore_neon_cyberpunk_20260108_184503.png" width="250"> |
| Australia | Melbourne | forest | <img src="posters/melbourne_forest_20260108_181459.png" width="250"> |
| UAE | Dubai | midnight_blue | <img src="posters/dubai_midnight_blue_20260108_174920.png" width="250"> |

## Quick Start

### Using Docker (Recommended)

```bash
git clone https://github.com/N3mes1s/maptoposter.git
cd maptoposter

docker compose up

# Open http://localhost:8000
```

### Building from Source

```bash
# Prerequisites: Rust 1.75+
cd maptoposter-rs
cargo build --release

# Copy assets and run
cp -r ../themes ../fonts ../frontend .
./target/release/maptoposter-rs
```

## API

### Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| `GET` | `/` | Web interface |
| `GET` | `/health` | Health check |
| `GET` | `/api/themes` | List all 35 themes |
| `POST` | `/api/posters` | Create poster job |
| `GET` | `/api/posters/:id` | Get job status |
| `GET` | `/api/posters/:id/stream` | SSE progress stream |
| `GET` | `/api/posters/:id/download` | Download poster PNG |
| `POST` | `/api/posters/:id/rerender` | Re-render with new theme |

### Create a Poster

```bash
curl -X POST http://localhost:8000/api/posters \
  -H "Content-Type: application/json" \
  -d '{
    "city": "Tokyo",
    "country": "Japan",
    "theme": "japanese_ink",
    "distance": 15000
  }'
```

Response:
```json
{
  "job_id": "550e8400-e29b-41d4-a716-446655440000",
  "status": "queued",
  "estimated_time": 40
}
```

### Re-render with Different Theme

After generating a poster, instantly switch themes without re-downloading map data:

```bash
curl -X POST http://localhost:8000/api/posters/{job_id}/rerender \
  -H "Content-Type: application/json" \
  -d '{"theme": "neon_cyberpunk"}'
```

## Themes (35 Available)

### Classic
- `feature_based` - Default balanced theme
- `noir` - Dark and dramatic
- `blueprint` - Technical drawing style
- `japanese_ink` - Minimalist sumi-e inspired

### Warm Tones
- `sunset` - Orange and purple gradients
- `warm_beige` - Soft earth tones
- `terracotta` - Mediterranean warmth
- `autumn` - Fall colors
- `copper_patina` - Aged metal look
- `desert` - Sandy warmth
- `coral` - Tropical vibes

### Cool Tones
- `ocean` - Deep sea blues
- `midnight_blue` - Night sky
- `arctic` - Ice and snow
- `sapphire` - Royal blue elegance

### Nature
- `forest` - Deep greens
- `sage` - Soft botanical
- `mint` - Fresh and clean
- `emerald` - Rich green

### Modern
- `neon_cyberpunk` - Futuristic glow
- `neon_pink` - Hot pink accents
- `electric_blue` - Vibrant azure
- `pastel_dream` - Soft pastels

### Monochrome
- `monochrome_blue` - Blue scale
- `sepia` - Vintage photograph
- `concrete` - Urban gray
- `platinum` - Silver tones
- `gradient_roads` - Smooth gradient shading
- `contrast_zones` - High contrast urban density

### Luxury
- `rose_gold` - Elegant pink gold
- `gold` - Classic gold
- `ruby` - Deep red
- `mocha` - Rich coffee
- `lavender` - Soft purple
- `vintage_paper` - Aged parchment

## Distance Guide

| Distance | Best for |
|----------|----------|
| 4000-6000m | Small/dense cities (Venice, Amsterdam center) |
| 8000-12000m | Medium cities, focused downtown (Paris, Barcelona) |
| 15000-20000m | Large metros, full city view (Tokyo, Mumbai) |

## Environment Variables

| Variable | Default | Description |
|----------|---------|-------------|
| `PORT` | `8000` | Server port |
| `MAX_DISTANCE` | `20000` | Maximum radius in meters |
| `MIN_DISTANCE` | `2000` | Minimum radius in meters |
| `DEFAULT_DISTANCE` | `15000` | Default radius |
| `OSM_TIMEOUT` | `120` | Overpass API timeout (seconds) |
| `NOMINATIM_TIMEOUT` | `10` | Geocoding timeout (seconds) |
| `RUST_LOG` | `info` | Log level (debug, info, warn, error) |

## Architecture

```
maptoposter/
├── maptoposter-rs/         # Rust server
│   ├── src/
│   │   ├── main.rs         # Entry point, routes
│   │   ├── api/            # HTTP handlers, SSE streaming
│   │   ├── core/           # Poster generation, OSM client
│   │   ├── rendering/      # Canvas, road styles
│   │   └── themes/         # Theme loader
│   └── Cargo.toml
├── frontend/               # Web UI
│   ├── index.html
│   ├── css/styles.css
│   └── js/
│       ├── app.js          # Main application
│       └── api.js          # API client
├── themes/                 # 35 theme JSON files
├── fonts/                  # Roboto fonts
├── Dockerfile
└── docker-compose.yml
```

## Adding Custom Themes

Create a JSON file in `themes/` directory:

```json
{
  "name": "My Theme",
  "description": "Description of the theme",
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

## Data Sources

- **Geocoding**: [Nominatim](https://nominatim.org/) (OpenStreetMap)
- **Map Data**: [Overpass API](https://overpass-api.de/) with fallback mirrors

## Performance

| Metric | Value |
|--------|-------|
| Memory (idle) | ~50 MB |
| Memory (generating) | ~200 MB |
| Initial generation | 15-30 seconds |
| Theme re-render | ~3 seconds |
| Output size | 3-10 MB PNG |

## License

MIT License - See [LICENSE](LICENSE)

## Credits

- Map data: OpenStreetMap contributors
- Fonts: Roboto by Google
