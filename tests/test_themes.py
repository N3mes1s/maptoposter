"""Tests for theme loading functionality."""

import pytest
import json
from pathlib import Path

from src.maptoposter.themes.loader import (
    load_theme,
    get_available_themes,
    get_all_themes_with_details,
)
from src.maptoposter.exceptions import ThemeNotFoundError


class TestGetAvailableThemes:
    """Test get_available_themes function."""

    def test_returns_list(self, themes_dir):
        """Test that function returns a list."""
        themes = get_available_themes()
        assert isinstance(themes, list)

    def test_contains_default_theme(self, themes_dir):
        """Test that feature_based theme exists."""
        themes = get_available_themes()
        assert "feature_based" in themes

    def test_contains_known_themes(self, themes_dir):
        """Test that known themes exist."""
        themes = get_available_themes()
        expected = ["noir", "blueprint", "japanese_ink"]
        for theme in expected:
            assert theme in themes, f"Expected theme '{theme}' not found"


class TestLoadTheme:
    """Test load_theme function."""

    def test_load_existing_theme(self):
        """Test loading an existing theme."""
        theme = load_theme("feature_based")
        assert isinstance(theme, dict)
        assert "bg" in theme
        assert "text" in theme
        assert "water" in theme
        assert "parks" in theme

    def test_load_nonexistent_theme(self):
        """Test loading a non-existent theme raises error."""
        with pytest.raises(ThemeNotFoundError) as exc_info:
            load_theme("nonexistent_theme_12345")
        assert "nonexistent_theme_12345" in str(exc_info.value)

    def test_theme_has_required_keys(self):
        """Test that loaded theme has all required keys."""
        theme = load_theme("noir")
        required_keys = [
            "bg", "text", "gradient_color", "water", "parks",
            "road_motorway", "road_primary", "road_secondary",
            "road_tertiary", "road_residential", "road_default"
        ]
        for key in required_keys:
            assert key in theme, f"Required key '{key}' missing from theme"

    def test_theme_colors_are_valid(self):
        """Test that theme colors are valid hex codes."""
        theme = load_theme("blueprint")
        color_keys = ["bg", "text", "water", "parks"]
        for key in color_keys:
            color = theme[key]
            assert color.startswith("#"), f"Color '{key}' should start with #"
            assert len(color) == 7, f"Color '{key}' should be 7 chars (e.g., #FFFFFF)"


class TestGetAllThemesWithDetails:
    """Test get_all_themes_with_details function."""

    def test_returns_list(self):
        """Test that function returns a list."""
        themes = get_all_themes_with_details()
        assert isinstance(themes, list)
        assert len(themes) > 0

    def test_themes_have_required_fields(self):
        """Test that each theme has required fields."""
        themes = get_all_themes_with_details()
        for theme in themes:
            assert "id" in theme
            assert "name" in theme or "id" in theme  # At minimum, id should be present
            assert "bg" in theme


class TestThemeFileIntegrity:
    """Test theme file integrity."""

    def test_all_theme_files_are_valid_json(self, themes_dir):
        """Test that all theme files are valid JSON."""
        if not themes_dir.exists():
            pytest.skip("Themes directory not found")

        for theme_file in themes_dir.glob("*.json"):
            with open(theme_file) as f:
                try:
                    data = json.load(f)
                    assert isinstance(data, dict), f"{theme_file.name} should contain a dict"
                except json.JSONDecodeError as e:
                    pytest.fail(f"Invalid JSON in {theme_file.name}: {e}")

    def test_theme_count(self, themes_dir):
        """Test that we have the expected number of themes."""
        if not themes_dir.exists():
            pytest.skip("Themes directory not found")

        theme_count = len(list(themes_dir.glob("*.json")))
        assert theme_count >= 10, f"Expected at least 10 themes, found {theme_count}"
