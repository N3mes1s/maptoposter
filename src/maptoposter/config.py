"""Central configuration for MapToPoster."""

from pathlib import Path
from typing import Optional
import os


class Settings:
    """Application settings with environment variable support."""

    def __init__(self):
        self.BASE_DIR = Path(__file__).parent.parent.parent.parent
        self.THEMES_DIR = self.BASE_DIR / "themes"
        self.FONTS_DIR = self.BASE_DIR / "fonts"
        self.STATIC_DIR = self.BASE_DIR / "static"
        self.POSTERS_DIR = self.BASE_DIR / "posters"

        # API settings
        self.REDIS_URL = os.getenv("REDIS_URL", "redis://localhost:6379")
        self.MAX_DISTANCE = int(os.getenv("MAX_DISTANCE", "50000"))
        self.MIN_DISTANCE = int(os.getenv("MIN_DISTANCE", "2000"))
        self.DEFAULT_DISTANCE = int(os.getenv("DEFAULT_DISTANCE", "15000"))
        self.DEFAULT_THEME = os.getenv("DEFAULT_THEME", "feature_based")

        # Rate limiting
        self.NOMINATIM_DELAY = float(os.getenv("NOMINATIM_DELAY", "1.0"))
        self.OSM_DELAY = float(os.getenv("OSM_DELAY", "0.5"))

        # Output settings
        self.OUTPUT_DPI = int(os.getenv("OUTPUT_DPI", "300"))
        self.PREVIEW_DPI = int(os.getenv("PREVIEW_DPI", "72"))

        # Ensure directories exist
        self.STATIC_DIR.mkdir(exist_ok=True)
        self.POSTERS_DIR.mkdir(exist_ok=True)


settings = Settings()
