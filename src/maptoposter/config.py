"""Central configuration for MapToPoster."""

import os
import re
from pathlib import Path
from typing import List, Optional


class Settings:
    """Application settings with environment variable support and validation."""

    def __init__(self):
        # Base directories - config.py is at src/maptoposter/config.py
        self.BASE_DIR = Path(__file__).parent.parent.parent
        self.THEMES_DIR = self.BASE_DIR / "themes"
        self.FONTS_DIR = self.BASE_DIR / "fonts"
        self.STATIC_DIR = self.BASE_DIR / "static"
        self.POSTERS_DIR = self.BASE_DIR / "posters"

        # API settings
        self.REDIS_URL = os.getenv("REDIS_URL", "redis://localhost:6379")
        self.API_HOST = os.getenv("API_HOST", "0.0.0.0")
        self.API_PORT = int(os.getenv("API_PORT", "8000"))

        # Distance settings
        self.MAX_DISTANCE = int(os.getenv("MAX_DISTANCE", "50000"))
        self.MIN_DISTANCE = int(os.getenv("MIN_DISTANCE", "2000"))
        self.DEFAULT_DISTANCE = int(os.getenv("DEFAULT_DISTANCE", "15000"))
        self.DEFAULT_THEME = os.getenv("DEFAULT_THEME", "feature_based")

        # Rate limiting (seconds between requests)
        self.NOMINATIM_DELAY = float(os.getenv("NOMINATIM_DELAY", "1.0"))
        self.OSM_DELAY = float(os.getenv("OSM_DELAY", "0.5"))

        # Timeout settings (seconds)
        self.NOMINATIM_TIMEOUT = float(os.getenv("NOMINATIM_TIMEOUT", "10.0"))
        self.OSM_TIMEOUT = float(os.getenv("OSM_TIMEOUT", "60.0"))

        # Output settings
        self.OUTPUT_DPI = int(os.getenv("OUTPUT_DPI", "300"))
        self.PREVIEW_DPI = int(os.getenv("PREVIEW_DPI", "72"))

        # CORS settings
        self._cors_origins = os.getenv("CORS_ORIGINS", "*")
        self.CORS_ALLOW_CREDENTIALS = os.getenv("CORS_ALLOW_CREDENTIALS", "false").lower() == "true"

        # Logging settings
        self.LOG_LEVEL = os.getenv("LOG_LEVEL", "INFO").upper()
        self.LOG_JSON = os.getenv("LOG_JSON", "false").lower() == "true"

        # Job settings
        self.JOB_TTL_HOURS = int(os.getenv("JOB_TTL_HOURS", "24"))
        self.MAX_CONCURRENT_JOBS = int(os.getenv("MAX_CONCURRENT_JOBS", "5"))

        # Ensure directories exist
        self.STATIC_DIR.mkdir(exist_ok=True)
        self.POSTERS_DIR.mkdir(exist_ok=True)

    @property
    def CORS_ORIGINS(self) -> List[str]:
        """Parse CORS origins from environment variable."""
        if self._cors_origins == "*":
            return ["*"]
        return [origin.strip() for origin in self._cors_origins.split(",") if origin.strip()]

    def validate_distance(self, distance: int) -> bool:
        """Check if distance is within valid range."""
        return self.MIN_DISTANCE <= distance <= self.MAX_DISTANCE

    def validate_city_name(self, name: str) -> bool:
        """
        Validate city/country name for safe filename generation.

        Allows letters, numbers, spaces, hyphens, apostrophes, and periods.
        """
        if not name or len(name) > 100:
            return False
        # Allow international characters, spaces, hyphens, apostrophes, periods
        pattern = r'^[\w\s\-\'\.]+$'
        return bool(re.match(pattern, name, re.UNICODE))

    def sanitize_filename(self, name: str) -> str:
        """
        Sanitize a string for safe use in filenames.

        Removes path separators and other dangerous characters.
        """
        # Remove path separators and null bytes
        sanitized = name.replace("/", "_").replace("\\", "_").replace("\x00", "")
        # Remove other potentially dangerous characters
        sanitized = re.sub(r'[<>:"|?*]', "_", sanitized)
        # Limit length
        return sanitized[:50]


settings = Settings()
