"""Pytest configuration and shared fixtures."""

import pytest
from pathlib import Path
from typing import Dict, Any


@pytest.fixture
def sample_theme() -> Dict[str, Any]:
    """Return a sample theme dictionary for testing."""
    return {
        "name": "Test Theme",
        "description": "A theme for testing",
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


@pytest.fixture
def sample_coordinates() -> tuple:
    """Return sample coordinates (Venice, Italy)."""
    return (45.4408, 12.3155)


@pytest.fixture
def project_root() -> Path:
    """Return the project root directory."""
    return Path(__file__).parent.parent


@pytest.fixture
def themes_dir(project_root: Path) -> Path:
    """Return the themes directory."""
    return project_root / "themes"


@pytest.fixture
def fonts_dir(project_root: Path) -> Path:
    """Return the fonts directory."""
    return project_root / "fonts"
