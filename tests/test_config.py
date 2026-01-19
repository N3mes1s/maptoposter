"""Tests for configuration module."""

import pytest
from src.maptoposter.config import Settings


class TestSettings:
    """Test Settings class."""

    def test_default_values(self):
        """Test that default values are set correctly."""
        settings = Settings()

        assert settings.MIN_DISTANCE == 2000
        assert settings.MAX_DISTANCE == 50000
        assert settings.DEFAULT_DISTANCE == 15000
        assert settings.DEFAULT_THEME == "feature_based"
        assert settings.OUTPUT_DPI == 300
        assert settings.PREVIEW_DPI == 72

    def test_validate_distance_valid(self):
        """Test distance validation with valid values."""
        settings = Settings()

        assert settings.validate_distance(2000) is True
        assert settings.validate_distance(15000) is True
        assert settings.validate_distance(50000) is True

    def test_validate_distance_invalid(self):
        """Test distance validation with invalid values."""
        settings = Settings()

        assert settings.validate_distance(1999) is False
        assert settings.validate_distance(50001) is False
        assert settings.validate_distance(0) is False
        assert settings.validate_distance(-1000) is False

    def test_validate_city_name_valid(self):
        """Test city name validation with valid names."""
        settings = Settings()

        assert settings.validate_city_name("New York") is True
        assert settings.validate_city_name("São Paulo") is True
        assert settings.validate_city_name("Paris") is True
        assert settings.validate_city_name("St. Petersburg") is True
        assert settings.validate_city_name("O'Brien") is True

    def test_validate_city_name_invalid(self):
        """Test city name validation with invalid names."""
        settings = Settings()

        assert settings.validate_city_name("") is False
        assert settings.validate_city_name("a" * 101) is False  # Too long

    def test_sanitize_filename(self):
        """Test filename sanitization."""
        settings = Settings()

        # Normal names
        assert settings.sanitize_filename("New York") == "New York"
        assert settings.sanitize_filename("test") == "test"

        # Path traversal attempts
        assert "/" not in settings.sanitize_filename("../../../etc/passwd")
        assert "\\" not in settings.sanitize_filename("..\\..\\etc\\passwd")

        # Special characters
        assert settings.sanitize_filename("file<>:name") == "file___name"

        # Null bytes
        assert "\x00" not in settings.sanitize_filename("file\x00name")

        # Length limit
        assert len(settings.sanitize_filename("a" * 100)) <= 50

    def test_cors_origins_default(self):
        """Test default CORS origins."""
        settings = Settings()
        assert settings.CORS_ORIGINS == ["*"]

    def test_directories_created(self, tmp_path, monkeypatch):
        """Test that required directories are created."""
        # This tests that STATIC_DIR and POSTERS_DIR are created
        settings = Settings()
        assert settings.STATIC_DIR.exists()
        assert settings.POSTERS_DIR.exists()
