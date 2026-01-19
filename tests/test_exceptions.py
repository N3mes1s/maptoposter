"""Tests for custom exceptions."""

import pytest
from src.maptoposter.exceptions import (
    MapToPosterError,
    GeocodingError,
    ThemeNotFoundError,
    DataFetchError,
    InvalidDistanceError,
    FontNotFoundError,
    InvalidInputError,
    APITimeoutError,
    PartialDataWarning,
)


class TestMapToPosterError:
    """Test base exception class."""

    def test_basic_error(self):
        """Test basic exception creation."""
        error = MapToPosterError("Test error")
        assert str(error) == "Test error"
        assert error.message == "Test error"
        assert error.details == {}

    def test_error_with_details(self):
        """Test exception with details."""
        error = MapToPosterError("Test error", {"key": "value"})
        assert error.details == {"key": "value"}


class TestGeocodingError:
    """Test GeocodingError exception."""

    def test_geocoding_error(self):
        """Test geocoding error creation."""
        error = GeocodingError("New York", "USA")
        assert "New York" in str(error)
        assert "USA" in str(error)
        assert error.city == "New York"
        assert error.country == "USA"

    def test_geocoding_error_with_reason(self):
        """Test geocoding error with custom reason."""
        error = GeocodingError("Invalid City", "Invalid Country", "API timeout")
        assert "API timeout" in str(error)


class TestThemeNotFoundError:
    """Test ThemeNotFoundError exception."""

    def test_theme_not_found(self):
        """Test theme not found error."""
        error = ThemeNotFoundError("invalid_theme")
        assert "invalid_theme" in str(error)
        assert error.theme_name == "invalid_theme"

    def test_theme_not_found_with_available(self):
        """Test theme not found with available themes listed."""
        available = ["noir", "blueprint", "sunset", "forest", "ocean", "extra"]
        error = ThemeNotFoundError("invalid_theme", available)
        assert "noir" in str(error)
        assert "+1 more" in str(error)  # Only shows first 5


class TestInvalidDistanceError:
    """Test InvalidDistanceError exception."""

    def test_distance_error(self):
        """Test distance error creation."""
        error = InvalidDistanceError(1000, 2000, 50000)
        assert "1000" in str(error)
        assert "2000" in str(error)
        assert "50000" in str(error)
        assert error.distance == 1000
        assert error.min_distance == 2000
        assert error.max_distance == 50000


class TestDataFetchError:
    """Test DataFetchError exception."""

    def test_data_fetch_error(self):
        """Test data fetch error."""
        error = DataFetchError("street network", "Connection timeout")
        assert "street network" in str(error)
        assert "Connection timeout" in str(error)
        assert error.data_type == "street network"


class TestAPITimeoutError:
    """Test APITimeoutError exception."""

    def test_timeout_error(self):
        """Test timeout error."""
        error = APITimeoutError("Nominatim", 10.0)
        assert "Nominatim" in str(error)
        assert "10" in str(error)


class TestPartialDataWarning:
    """Test PartialDataWarning exception."""

    def test_partial_data_warning(self):
        """Test partial data warning."""
        warning = PartialDataWarning("water features", "No data available")
        assert "water features" in str(warning)
        assert "Continuing without it" in str(warning)
        assert warning.feature == "water features"
