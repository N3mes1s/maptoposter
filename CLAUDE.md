# CLAUDE.md - AI Assistant Guide for City Map Poster Generator

This document provides essential context for AI assistants working with this codebase.

## Project Overview

**Purpose**: Generate beautiful, minimalist map posters for any city in the world with customizable themes and zoom levels.

**Author**: Ankur Gupta
**License**: MIT
**Language**: Python 3.x

## Directory Structure

```
maptoposter/
├── create_map_poster.py    # Main application (single-file, ~470 lines)
├── requirements.txt        # Pinned Python dependencies (30 packages)
├── README.md               # User documentation with examples
├── LICENSE                 # MIT License
├── .gitignore              # Ignores cache/, env/
├── themes/                 # 17 JSON theme configuration files
│   ├── feature_based.json  # Default theme
│   ├── noir.json           # Black background with white roads
│   ├── japanese_ink.json   # Minimalist ink wash style
│   ├── neon_cyberpunk.json # Dark with electric pink/cyan
│   └── ... (13 more)
├── fonts/                  # Roboto font files (Bold, Regular, Light)
└── posters/                # Generated PNG output files
```

## Quick Start Commands

```bash
# Install dependencies
pip install -r requirements.txt

# Generate a poster (basic)
python create_map_poster.py --city "New York" --country "USA"

# Generate with specific theme and zoom
python create_map_poster.py -c "Tokyo" -C "Japan" -t japanese_ink -d 15000

# List all available themes
python create_map_poster.py --list-themes

# Show help and examples
python create_map_poster.py
```

## CLI Arguments

| Argument | Short | Required | Default | Description |
|----------|-------|----------|---------|-------------|
| `--city` | `-c` | Yes | - | City name |
| `--country` | `-C` | Yes | - | Country name |
| `--theme` | `-t` | No | feature_based | Theme name from themes/ |
| `--distance` | `-d` | No | 29000 | Map radius in meters |
| `--list-themes` | - | No | - | List available themes |

## Key Dependencies

| Package | Purpose |
|---------|---------|
| `osmnx` | Fetch OpenStreetMap street networks and features |
| `matplotlib` | Render map visualizations and export PNG |
| `geopy` | Geocode city names to lat/lon via Nominatim |
| `geopandas` | Handle geographic data (water, parks) |
| `numpy` | Create gradient fade effects |
| `tqdm` | Display progress bars |

## Code Architecture

### Data Flow

```
CLI args → Geocoding (Nominatim) → Data Fetching (OSMnx) → Rendering (matplotlib) → PNG export
```

### Rendering Pipeline (z-order layers)

```
z=11  Text labels (city, country, coordinates, attribution)
z=10  Gradient fades (top & bottom transparency overlays)
z=3   Roads (via ox.plot_graph with hierarchy colors/widths)
z=2   Parks (green polygons)
z=1   Water (blue polygons)
z=0   Background color
```

### Key Functions

| Function | Lines | Purpose |
|----------|-------|---------|
| `get_coordinates()` | 196-214 | Geocode city/country to lat/lon |
| `create_poster()` | 216-323 | Main rendering pipeline |
| `get_edge_colors_by_type()` | 134-165 | Map OSM highway types to colors |
| `get_edge_widths_by_type()` | 167-193 | Map OSM highway types to line widths |
| `create_gradient_fade()` | 100-132 | Create transparency gradient overlay |
| `load_theme()` | 66-95 | Load JSON theme with fallback |
| `load_fonts()` | 18-35 | Load Roboto font files |
| `generate_output_filename()` | 39-49 | Create unique timestamped filename |
| `get_available_themes()` | 51-64 | Scan themes/ directory |
| `list_themes()` | 381-404 | Display themes with descriptions |
| `print_examples()` | 325-379 | Show CLI usage examples |

### Global Constants

```python
THEMES_DIR = "themes"    # Theme JSON files location
FONTS_DIR = "fonts"      # Roboto font files location
POSTERS_DIR = "posters"  # Generated output location
```

## Theme System

### Theme JSON Structure

```json
{
  "name": "Theme Name",
  "description": "Theme description",
  "bg": "#FFFFFF",              // Background color
  "text": "#000000",            // Text color
  "gradient_color": "#FFFFFF",  // Fade overlay color
  "water": "#C0C0C0",           // Water features
  "parks": "#F0F0F0",           // Parks/green spaces
  "road_motorway": "#0A0A0A",   // Motorways (thickest)
  "road_primary": "#1A1A1A",    // Primary roads
  "road_secondary": "#2A2A2A",  // Secondary roads
  "road_tertiary": "#3A3A3A",   // Tertiary roads
  "road_residential": "#4A4A4A", // Residential (thinnest)
  "road_default": "#3A3A3A"     // Fallback color
}
```

### Available Themes (17 total)

`feature_based` (default), `noir`, `japanese_ink`, `neon_cyberpunk`, `blueprint`, `sunset`, `forest`, `ocean`, `midnight_blue`, `warm_beige`, `pastel_dream`, `contrast_zones`, `gradient_roads`, `monochrome_blue`, `autumn`, `copper_patina`, `terracotta`

### Adding a New Theme

1. Create `themes/my_theme.json` with the structure above
2. The theme is automatically available via `--theme my_theme`

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
- **UPPERCASE** for module constants
- **f-strings** for string formatting
- **Docstrings** on all functions
- **No type hints** (older Python style)

### Error Handling Pattern

```python
try:
    water = ox.features_from_point(point, tags={...}, dist=dist)
except:
    water = None  # Graceful degradation
```

### Progress Feedback

```python
print(f"✓ Success message")
print(f"✗ Error message")
print(f"⚠ Warning message")
```

### Rate Limiting

```python
time.sleep(1)    # Nominatim API respect
time.sleep(0.5)  # Between OSM requests
```

## Common Development Tasks

### Add a New Map Layer (e.g., railways)

```python
# In create_poster(), after parks fetch (~line 240):
try:
    railways = ox.features_from_point(point, tags={'railway': 'rail'}, dist=dist)
except:
    railways = None

# Plot before roads (~line 258):
if railways is not None and not railways.empty:
    railways.plot(ax=ax, color=THEME.get('railway', '#888888'),
                  linewidth=0.5, zorder=2.5)
```

### Add a New Theme Property

1. Add to theme JSON: `"railway": "#FF0000"`
2. Use in code: `THEME.get('railway', '#default')`
3. Add fallback in `load_theme()` default dict

### Modify Typography Positioning

Text uses `transform=ax.transAxes` (0-1 normalized coordinates):
```
y=0.14   City name (spaced letters)
y=0.125  Decorative line
y=0.10   Country name
y=0.07   Coordinates
y=0.02   Attribution (bottom-right)
```

## OSMnx Patterns

```python
# Street network (all road types)
G = ox.graph_from_point(point, dist=dist, network_type='all')

# Water features
water = ox.features_from_point(point, tags={'natural': 'water'}, dist=dist)

# Parks
parks = ox.features_from_point(point, tags={'leisure': 'park'}, dist=dist)

# Buildings
buildings = ox.features_from_point(point, tags={'building': True}, dist=dist)

# Different network types
network_type='drive'  # Roads only (faster)
network_type='bike'   # Bike paths
network_type='walk'   # Pedestrian paths
```

## Performance Notes

- **Large distances (>20km)**: Slow downloads, high memory usage
- **Quick previews**: Change `dpi=300` to `dpi=150` in `plt.savefig()`
- **Faster renders**: Use `network_type='drive'` instead of `'all'`
- **Rate limits**: Nominatim requires 1 second between requests

## Testing

There are no automated tests in this codebase. Testing is done manually:

```bash
# Quick test with small distance
python create_map_poster.py -c "Venice" -C "Italy" -t blueprint -d 4000

# Test a new theme
python create_map_poster.py -c "Tokyo" -C "Japan" -t my_new_theme -d 8000
```

## Output Format

- **Format**: PNG at 300 DPI
- **Filename**: `{city}_{theme}_{YYYYMMDD_HHMMSS}.png`
- **Location**: `posters/` directory
- **Size**: 3-16 MB depending on map complexity

## Important Notes for AI Assistants

1. **Single-file architecture**: All code is in `create_map_poster.py` (~470 lines)
2. **No tests**: Changes should be manually verified
3. **Theme validation**: Themes must exist in `themes/` directory before use
4. **Fonts required**: Roboto fonts must be present in `fonts/` directory
5. **API rate limits**: Nominatim has strict rate limits (1 req/sec)
6. **Global state**: `THEME` is loaded as a module-level variable after CLI parsing
7. **Graceful degradation**: Missing water/parks features don't crash the app
8. **Dependencies are pinned**: All versions locked in requirements.txt

## File Modification Guidelines

When modifying this codebase:

1. **For new features**: Add to `create_poster()` function, maintaining z-order
2. **For new themes**: Create JSON file in `themes/` directory
3. **For CLI changes**: Modify argparse section at bottom of file
4. **For styling changes**: Modify `get_edge_colors_by_type()` and `get_edge_widths_by_type()`
5. **For typography changes**: Modify text positioning section in `create_poster()`

## Distance Guide

| Distance | Best for |
|----------|----------|
| 4000-6000m | Small/dense cities (Venice, Amsterdam center) |
| 8000-12000m | Medium cities, focused downtown (Paris, Barcelona) |
| 15000-20000m | Large metros, full city view (Tokyo, Mumbai) |
