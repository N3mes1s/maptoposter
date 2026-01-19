"""Theme loading and management."""

import json
from pathlib import Path
from typing import Dict, List, Optional, Any

from ..config import settings
from ..exceptions import ThemeNotFoundError


# Default theme fallback
DEFAULT_THEME = {
    "name": "Feature-Based Shading",
    "description": "Different shades for different road types and features with clear hierarchy",
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


def get_available_themes() -> List[str]:
    """
    Scans the themes directory and returns a list of available theme names.
    """
    themes_dir = settings.THEMES_DIR
    if not themes_dir.exists():
        themes_dir.mkdir(parents=True)
        return []

    themes = []
    for file in sorted(themes_dir.iterdir()):
        if file.suffix == '.json':
            themes.append(file.stem)
    return themes


def load_theme(theme_name: str = "feature_based") -> Dict[str, Any]:
    """
    Load theme from JSON file in themes directory.

    Args:
        theme_name: Name of the theme (without .json extension)

    Returns:
        Theme dictionary with all color and style settings

    Raises:
        ThemeNotFoundError: If theme file doesn't exist
    """
    theme_file = settings.THEMES_DIR / f"{theme_name}.json"

    if not theme_file.exists():
        if theme_name == "feature_based":
            return DEFAULT_THEME.copy()
        raise ThemeNotFoundError(f"Theme '{theme_name}' not found")

    with open(theme_file, 'r') as f:
        theme = json.load(f)

    # Ensure all required keys exist with fallbacks
    for key, value in DEFAULT_THEME.items():
        if key not in theme:
            theme[key] = value

    return theme


def get_theme_details(theme_name: str) -> Dict[str, Any]:
    """
    Get detailed information about a theme including colors for preview.

    Args:
        theme_name: Name of the theme

    Returns:
        Theme dictionary with all details
    """
    return load_theme(theme_name)


def get_all_themes_with_details() -> List[Dict[str, Any]]:
    """
    Get all available themes with their full details.

    Returns:
        List of theme dictionaries
    """
    themes = []
    for theme_name in get_available_themes():
        try:
            theme_data = load_theme(theme_name)
            theme_data['id'] = theme_name
            themes.append(theme_data)
        except ThemeNotFoundError:
            continue
    return themes
