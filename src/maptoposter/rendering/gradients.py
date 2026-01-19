"""Gradient fade effects for map posters."""

from typing import Literal

import numpy as np
import matplotlib.colors as mcolors
from matplotlib.axes import Axes
from numpy.typing import NDArray

from ..logging_config import get_logger

logger = get_logger("rendering.gradients")


def create_gradient_fade(
    ax: Axes,
    color: str,
    location: Literal["bottom", "top"] = "bottom",
    zorder: int = 10
) -> None:
    """
    Creates a fade effect at the top or bottom of the map.

    The gradient transitions from opaque to transparent, creating a smooth
    fade effect that can be used to overlay text areas.

    Args:
        ax: Matplotlib axes object to add the gradient to
        color: Hex color string for the gradient (e.g., "#FFFFFF")
        location: Position of the gradient - "bottom" or "top"
        zorder: Z-order for layering (higher = on top)

    Raises:
        ValueError: If color is not a valid color string
    """
    logger.debug(f"Creating {location} gradient fade with color {color}")

    # Create gradient array
    vals: NDArray[np.float64] = np.linspace(0, 1, 256).reshape(-1, 1)
    gradient: NDArray[np.float64] = np.hstack((vals, vals))

    # Convert color to RGB
    try:
        rgb = mcolors.to_rgb(color)
    except ValueError as e:
        logger.error(f"Invalid color value: {color}")
        raise ValueError(f"Invalid color: {color}") from e

    # Build RGBA colormap
    my_colors: NDArray[np.float64] = np.zeros((256, 4))
    my_colors[:, 0] = rgb[0]  # Red
    my_colors[:, 1] = rgb[1]  # Green
    my_colors[:, 2] = rgb[2]  # Blue

    # Set alpha gradient based on location
    if location == "bottom":
        my_colors[:, 3] = np.linspace(1, 0, 256)  # Opaque to transparent
        extent_y_start = 0
        extent_y_end = 0.25
    else:  # top
        my_colors[:, 3] = np.linspace(0, 1, 256)  # Transparent to opaque
        extent_y_start = 0.75
        extent_y_end = 1.0

    custom_cmap = mcolors.ListedColormap(my_colors)

    # Calculate extent in data coordinates
    xlim = ax.get_xlim()
    ylim = ax.get_ylim()
    y_range = ylim[1] - ylim[0]

    y_bottom = ylim[0] + y_range * extent_y_start
    y_top = ylim[0] + y_range * extent_y_end

    # Add gradient image to axes
    ax.imshow(
        gradient,
        extent=[xlim[0], xlim[1], y_bottom, y_top],
        aspect="auto",
        cmap=custom_cmap,
        zorder=zorder,
        origin="lower"
    )

    logger.debug(f"Added {location} gradient from y={y_bottom:.2f} to y={y_top:.2f}")
