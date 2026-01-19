"""Font loading and typography utilities."""

from pathlib import Path
from typing import Dict, Optional
from matplotlib.font_manager import FontProperties

from ..config import settings
from ..exceptions import FontNotFoundError


def load_fonts() -> Optional[Dict[str, Path]]:
    """
    Load Roboto fonts from the fonts directory.

    Returns:
        Dict with font paths for different weights, or None if fonts not found
    """
    fonts = {
        'bold': settings.FONTS_DIR / 'Roboto-Bold.ttf',
        'regular': settings.FONTS_DIR / 'Roboto-Regular.ttf',
        'light': settings.FONTS_DIR / 'Roboto-Light.ttf'
    }

    # Verify fonts exist
    for weight, path in fonts.items():
        if not path.exists():
            return None

    return fonts


def get_font_properties(fonts: Optional[Dict[str, Path]], weight: str, size: int) -> FontProperties:
    """
    Get FontProperties for the specified weight and size.

    Args:
        fonts: Dict with font paths, or None for system fallback
        weight: Font weight ('bold', 'regular', 'light')
        size: Font size in points

    Returns:
        FontProperties object
    """
    if fonts and weight in fonts:
        return FontProperties(fname=str(fonts[weight]), size=size)

    # Fallback to system fonts
    weight_map = {
        'bold': 'bold',
        'regular': 'normal',
        'light': 'normal'
    }
    return FontProperties(family='monospace', weight=weight_map.get(weight, 'normal'), size=size)
