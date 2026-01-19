"""Theme loading and management."""

import json
from pathlib import Path
from typing import Dict, List, Any

from ..config import settings
from ..exceptions import ThemeNotFoundError
from ..logging_config import get_logger

logger = get_logger("themes")


# Default theme fallback
DEFAULT_THEME: Dict[str, str] = {
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

# Required theme keys for validation
REQUIRED_THEME_KEYS = ["bg", "text", "water", "parks", "road_default"]


def get_available_themes() -> List[str]:
    """
    Scans the themes directory and returns a list of available theme names.

    Returns:
        List of theme names (without .json extension)
    """
    themes_dir = settings.THEMES_DIR

    if not themes_dir.exists():
        logger.warning(f"Themes directory does not exist: {themes_dir}")
        themes_dir.mkdir(parents=True)
        logger.info(f"Created themes directory: {themes_dir}")
        return []

    themes = []
    for file in sorted(themes_dir.iterdir()):
        if file.suffix == '.json':
            themes.append(file.stem)

    logger.debug(f"Found {len(themes)} themes in {themes_dir}")
    return themes


def load_theme(theme_name: str = "feature_based") -> Dict[str, Any]:
    """
    Load theme from JSON file in themes directory.

    Args:
        theme_name: Name of the theme (without .json extension)

    Returns:
        Theme dictionary with all color and style settings

    Raises:
        ThemeNotFoundError: If theme file doesn't exist or is invalid
    """
    theme_file = settings.THEMES_DIR / f"{theme_name}.json"
    logger.debug(f"Loading theme: {theme_name} from {theme_file}")

    if not theme_file.exists():
        if theme_name == "feature_based":
            logger.debug("Using built-in feature_based theme")
            return DEFAULT_THEME.copy()
        available = get_available_themes()
        logger.warning(f"Theme '{theme_name}' not found. Available: {available}")
        raise ThemeNotFoundError(theme_name, available)

    try:
        with open(theme_file, 'r', encoding='utf-8') as f:
            theme = json.load(f)
    except json.JSONDecodeError as e:
        logger.error(f"Invalid JSON in theme file {theme_file}: {e}")
        raise ThemeNotFoundError(
            theme_name,
            get_available_themes()
        )
    except IOError as e:
        logger.error(f"Error reading theme file {theme_file}: {e}")
        raise ThemeNotFoundError(theme_name, get_available_themes())

    # Validate theme has required keys
    missing_keys = [key for key in REQUIRED_THEME_KEYS if key not in theme]
    if missing_keys:
        logger.warning(
            f"Theme '{theme_name}' missing keys: {missing_keys}. Using defaults."
        )

    # Ensure all required keys exist with fallbacks
    for key, value in DEFAULT_THEME.items():
        if key not in theme:
            theme[key] = value

    logger.debug(f"Successfully loaded theme: {theme_name}")
    return theme


def get_theme_details(theme_name: str) -> Dict[str, Any]:
    """
    Get detailed information about a theme including colors for preview.

    Args:
        theme_name: Name of the theme

    Returns:
        Theme dictionary with all details

    Raises:
        ThemeNotFoundError: If theme doesn't exist
    """
    return load_theme(theme_name)


def get_all_themes_with_details() -> List[Dict[str, Any]]:
    """
    Get all available themes with their full details.

    Returns:
        List of theme dictionaries with 'id' field added
    """
    themes = []
    theme_names = get_available_themes()
    logger.debug(f"Loading details for {len(theme_names)} themes")

    for theme_name in theme_names:
        try:
            theme_data = load_theme(theme_name)
            theme_data['id'] = theme_name
            themes.append(theme_data)
        except ThemeNotFoundError as e:
            logger.warning(f"Skipping theme {theme_name}: {e}")
            continue

    logger.info(f"Loaded {len(themes)} themes with details")
    return themes
