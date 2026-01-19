"""Font loading and typography utilities."""

from pathlib import Path
from typing import Dict, Literal, Optional
from matplotlib.font_manager import FontProperties

from ..config import settings
from ..logging_config import get_logger

logger = get_logger("rendering.typography")

# Font weight type
FontWeight = Literal["bold", "regular", "light"]

# Expected font files
FONT_FILES: Dict[FontWeight, str] = {
    "bold": "Roboto-Bold.ttf",
    "regular": "Roboto-Regular.ttf",
    "light": "Roboto-Light.ttf",
}


def load_fonts() -> Optional[Dict[FontWeight, Path]]:
    """
    Load Roboto fonts from the fonts directory.

    Looks for Roboto-Bold.ttf, Roboto-Regular.ttf, and Roboto-Light.ttf
    in the configured fonts directory.

    Returns:
        Dict with font paths for different weights, or None if any font is missing
    """
    logger.debug(f"Loading fonts from {settings.FONTS_DIR}")

    fonts: Dict[FontWeight, Path] = {}
    for weight, filename in FONT_FILES.items():
        font_path = settings.FONTS_DIR / filename
        if not font_path.exists():
            logger.warning(f"Font file not found: {font_path}")
            return None
        fonts[weight] = font_path

    logger.debug(f"Successfully loaded {len(fonts)} font files")
    return fonts


def get_font_properties(
    fonts: Optional[Dict[FontWeight, Path]],
    weight: FontWeight,
    size: int
) -> FontProperties:
    """
    Get FontProperties for the specified weight and size.

    If custom fonts are available, uses those. Otherwise falls back
    to system monospace fonts.

    Args:
        fonts: Dict with font paths, or None for system fallback
        weight: Font weight ('bold', 'regular', 'light')
        size: Font size in points

    Returns:
        FontProperties object configured for the specified weight and size
    """
    if fonts and weight in fonts:
        logger.debug(f"Using custom font: {fonts[weight].name} at size {size}")
        return FontProperties(fname=str(fonts[weight]), size=size)

    # Fallback to system fonts
    logger.debug(f"Using system fallback font for weight '{weight}' at size {size}")
    weight_map: Dict[FontWeight, str] = {
        "bold": "bold",
        "regular": "normal",
        "light": "normal",
    }
    return FontProperties(
        family="monospace",
        weight=weight_map.get(weight, "normal"),
        size=size
    )
