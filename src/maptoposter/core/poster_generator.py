"""Main poster generation logic."""

import io
import time
from dataclasses import dataclass
from typing import Callable, Optional, Dict, Any, Tuple

import osmnx as ox
import matplotlib.pyplot as plt

from ..config import settings
from ..exceptions import DataFetchError
from ..rendering.road_styles import get_edge_colors_by_type, get_edge_widths_by_type
from ..rendering.gradients import create_gradient_fade
from ..rendering.typography import load_fonts, get_font_properties
from .geocoding import format_coordinates


@dataclass
class PosterRequest:
    """Request parameters for poster generation."""
    city: str
    country: str
    theme_name: str = "feature_based"
    distance: int = 15000
    dpi: int = 300


@dataclass
class GenerationProgress:
    """Progress update during generation."""
    step: str
    progress: float  # 0.0 to 1.0 (0-100%)
    message: str


class PosterGenerator:
    """Generates city map posters with customizable themes."""

    def __init__(self, theme: Dict[str, Any]):
        """
        Initialize the generator with a theme.

        Args:
            theme: Theme dictionary with color and style settings
        """
        self.theme = theme
        self.fonts = load_fonts()

    def generate(
        self,
        request: PosterRequest,
        coordinates: Tuple[float, float],
        progress_callback: Optional[Callable[[GenerationProgress], None]] = None
    ) -> io.BytesIO:
        """
        Generate a poster and return as bytes buffer.

        Args:
            request: PosterRequest with generation parameters
            coordinates: (latitude, longitude) tuple
            progress_callback: Optional callback for progress updates

        Returns:
            BytesIO buffer containing the PNG image
        """
        def update_progress(step: str, progress: float, message: str):
            if progress_callback:
                progress_callback(GenerationProgress(step, progress, message))

        point = coordinates
        dist = request.distance

        # Step 1: Fetch street network (20-40%)
        update_progress("fetching_streets", 0.20, "Downloading street network...")
        try:
            G = ox.graph_from_point(point, dist=dist, dist_type='bbox', network_type='all')
        except Exception as e:
            raise DataFetchError(f"Failed to fetch street network: {e}")

        time.sleep(settings.OSM_DELAY)

        # Step 2: Fetch water features (40-50%)
        update_progress("fetching_water", 0.40, "Downloading water features...")
        try:
            water = ox.features_from_point(
                point, tags={'natural': 'water', 'waterway': 'riverbank'}, dist=dist
            )
        except Exception:
            water = None

        time.sleep(settings.OSM_DELAY)

        # Step 3: Fetch parks (50-60%)
        update_progress("fetching_parks", 0.50, "Downloading parks and green spaces...")
        try:
            parks = ox.features_from_point(
                point, tags={'leisure': 'park', 'landuse': 'grass'}, dist=dist
            )
        except Exception:
            parks = None

        # Step 4: Render map (60-90%)
        update_progress("rendering", 0.60, "Rendering map layers...")

        fig, ax = plt.subplots(figsize=(12, 16), facecolor=self.theme['bg'])
        ax.set_facecolor(self.theme['bg'])
        ax.set_position([0, 0, 1, 1])

        # Plot water layer
        if water is not None and not water.empty:
            water.plot(ax=ax, facecolor=self.theme['water'], edgecolor='none', zorder=1)

        # Plot parks layer
        if parks is not None and not parks.empty:
            parks.plot(ax=ax, facecolor=self.theme['parks'], edgecolor='none', zorder=2)

        update_progress("rendering_roads", 0.70, "Applying road hierarchy colors...")

        # Plot roads
        edge_colors = get_edge_colors_by_type(G, self.theme)
        edge_widths = get_edge_widths_by_type(G)

        ox.plot_graph(
            G, ax=ax, bgcolor=self.theme['bg'],
            node_size=0,
            edge_color=edge_colors,
            edge_linewidth=edge_widths,
            show=False, close=False
        )

        # Add gradient fades
        create_gradient_fade(ax, self.theme['gradient_color'], location='bottom', zorder=10)
        create_gradient_fade(ax, self.theme['gradient_color'], location='top', zorder=10)

        update_progress("adding_typography", 0.85, "Adding typography...")

        # Add typography
        self._add_typography(ax, request.city, request.country, coordinates)

        # Step 5: Save to buffer (90-100%)
        update_progress("saving", 0.90, "Saving poster...")

        buffer = io.BytesIO()
        plt.savefig(buffer, format='png', dpi=request.dpi, facecolor=self.theme['bg'])
        plt.close(fig)
        buffer.seek(0)

        update_progress("completed", 1.0, "Poster generation complete!")

        return buffer

    def _add_typography(
        self,
        ax,
        city: str,
        country: str,
        coordinates: Tuple[float, float]
    ):
        """Add text elements to the poster."""
        # Font properties
        font_main = get_font_properties(self.fonts, 'bold', 60)
        font_sub = get_font_properties(self.fonts, 'light', 22)
        font_coords = get_font_properties(self.fonts, 'regular', 14)
        font_attr = get_font_properties(self.fonts, 'light', 8)

        # City name (spaced letters)
        spaced_city = "  ".join(list(city.upper()))
        ax.text(0.5, 0.14, spaced_city, transform=ax.transAxes,
                color=self.theme['text'], ha='center', fontproperties=font_main, zorder=11)

        # Decorative line
        ax.plot([0.4, 0.6], [0.125, 0.125], transform=ax.transAxes,
                color=self.theme['text'], linewidth=1, zorder=11)

        # Country name
        ax.text(0.5, 0.10, country.upper(), transform=ax.transAxes,
                color=self.theme['text'], ha='center', fontproperties=font_sub, zorder=11)

        # Coordinates
        coords_str = format_coordinates(*coordinates)
        ax.text(0.5, 0.07, coords_str, transform=ax.transAxes,
                color=self.theme['text'], alpha=0.7, ha='center',
                fontproperties=font_coords, zorder=11)

        # Attribution
        ax.text(0.98, 0.02, "© OpenStreetMap contributors", transform=ax.transAxes,
                color=self.theme['text'], alpha=0.5, ha='right', va='bottom',
                fontproperties=font_attr, zorder=11)


def generate_poster(
    city: str,
    country: str,
    theme: Dict[str, Any],
    coordinates: Tuple[float, float],
    distance: int = 15000,
    dpi: int = 300,
    progress_callback: Optional[Callable[[GenerationProgress], None]] = None
) -> io.BytesIO:
    """
    Convenience function to generate a poster.

    Args:
        city: City name
        country: Country name
        theme: Theme dictionary
        coordinates: (latitude, longitude) tuple
        distance: Map radius in meters
        dpi: Output DPI
        progress_callback: Optional callback for progress updates

    Returns:
        BytesIO buffer containing the PNG image
    """
    generator = PosterGenerator(theme)
    request = PosterRequest(
        city=city,
        country=country,
        distance=distance,
        dpi=dpi
    )
    return generator.generate(request, coordinates, progress_callback)
