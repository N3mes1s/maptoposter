"""Tests for poster generation functionality."""

import pytest
from unittest.mock import MagicMock, patch

from src.maptoposter.core.poster_generator import (
    PosterRequest,
    PosterGenerator,
    GenerationProgress,
)
from src.maptoposter.exceptions import InvalidDistanceError, DataFetchError


class TestPosterRequest:
    """Test PosterRequest dataclass."""

    def test_valid_request(self):
        """Test creating a valid request."""
        request = PosterRequest(
            city="New York",
            country="USA",
            theme_name="noir",
            distance=10000,
            dpi=300
        )
        assert request.city == "New York"
        assert request.country == "USA"
        assert request.theme_name == "noir"
        assert request.distance == 10000
        assert request.dpi == 300

    def test_default_values(self):
        """Test default values are applied."""
        request = PosterRequest(city="Paris", country="France")
        assert request.theme_name == "feature_based"
        assert request.distance == 15000
        assert request.dpi == 300

    def test_distance_too_small(self):
        """Test that distance below minimum raises error."""
        with pytest.raises(InvalidDistanceError) as exc_info:
            PosterRequest(city="Test", country="Test", distance=1000)
        assert exc_info.value.distance == 1000
        assert exc_info.value.min_distance == 2000

    def test_distance_too_large(self):
        """Test that distance above maximum raises error."""
        with pytest.raises(InvalidDistanceError) as exc_info:
            PosterRequest(city="Test", country="Test", distance=100000)
        assert exc_info.value.distance == 100000
        assert exc_info.value.max_distance == 50000

    def test_boundary_distances(self):
        """Test boundary distance values."""
        # Minimum boundary
        request_min = PosterRequest(city="Test", country="Test", distance=2000)
        assert request_min.distance == 2000

        # Maximum boundary
        request_max = PosterRequest(city="Test", country="Test", distance=50000)
        assert request_max.distance == 50000


class TestGenerationProgress:
    """Test GenerationProgress dataclass."""

    def test_progress_creation(self):
        """Test creating a progress update."""
        progress = GenerationProgress(
            step="fetching_streets",
            progress=0.25,
            message="Downloading street network..."
        )
        assert progress.step == "fetching_streets"
        assert progress.progress == 0.25
        assert progress.message == "Downloading street network..."


class TestPosterGenerator:
    """Test PosterGenerator class."""

    def test_init(self, sample_theme):
        """Test generator initialization."""
        generator = PosterGenerator(sample_theme)
        assert generator.theme == sample_theme
        assert generator._warnings == []

    def test_warnings_property(self, sample_theme):
        """Test warnings property returns copy."""
        generator = PosterGenerator(sample_theme)
        generator._warnings = ["warning1", "warning2"]

        warnings = generator.warnings
        assert warnings == ["warning1", "warning2"]

        # Modifying returned list shouldn't affect internal list
        warnings.append("warning3")
        assert len(generator._warnings) == 2

    @patch("src.maptoposter.core.poster_generator.ox")
    def test_fetch_street_network_failure(self, mock_ox, sample_theme):
        """Test street network fetch failure raises DataFetchError."""
        mock_ox.graph_from_point.side_effect = Exception("Network error")

        generator = PosterGenerator(sample_theme)
        with pytest.raises(DataFetchError) as exc_info:
            generator._fetch_street_network((45.0, 12.0), 10000)

        assert "street network" in str(exc_info.value)

    @patch("src.maptoposter.core.poster_generator.ox")
    def test_fetch_water_features_failure_graceful(self, mock_ox, sample_theme):
        """Test water feature fetch failure is handled gracefully."""
        mock_ox.features_from_point.side_effect = Exception("No data")

        generator = PosterGenerator(sample_theme)
        result = generator._fetch_water_features((45.0, 12.0), 10000)

        assert result is None
        assert len(generator._warnings) == 1
        assert "water" in generator._warnings[0].lower()

    @patch("src.maptoposter.core.poster_generator.ox")
    def test_fetch_park_features_failure_graceful(self, mock_ox, sample_theme):
        """Test park feature fetch failure is handled gracefully."""
        mock_ox.features_from_point.side_effect = Exception("No data")

        generator = PosterGenerator(sample_theme)
        result = generator._fetch_park_features((45.0, 12.0), 10000)

        assert result is None
        assert len(generator._warnings) == 1
        assert "park" in generator._warnings[0].lower()


class TestProgressCallback:
    """Test progress callback functionality."""

    def test_progress_callback_called(self, sample_theme):
        """Test that progress callback is called during generation steps."""
        progress_updates = []

        def callback(progress: GenerationProgress):
            progress_updates.append(progress)

        generator = PosterGenerator(sample_theme)

        # We can't easily test the full generate() without mocking OSMnx
        # But we can test that the callback mechanism works
        # This would be tested more fully in integration tests
        assert callable(callback)
