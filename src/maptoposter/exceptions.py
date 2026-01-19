"""Custom exceptions for MapToPoster."""

from typing import Optional


class MapToPosterError(Exception):
    """Base exception for MapToPoster."""

    def __init__(self, message: str, details: Optional[dict] = None):
        super().__init__(message)
        self.message = message
        self.details = details or {}


class GeocodingError(MapToPosterError):
    """Failed to geocode location."""

    def __init__(self, city: str, country: str, reason: str = "Location not found"):
        message = f"Failed to geocode '{city}, {country}': {reason}"
        super().__init__(message, {"city": city, "country": country, "reason": reason})
        self.city = city
        self.country = country


class ThemeNotFoundError(MapToPosterError):
    """Theme does not exist."""

    def __init__(self, theme_name: str, available_themes: Optional[list] = None):
        message = f"Theme '{theme_name}' not found"
        if available_themes:
            message += f". Available: {', '.join(available_themes[:5])}"
            if len(available_themes) > 5:
                message += f" (+{len(available_themes) - 5} more)"
        super().__init__(message, {"theme": theme_name, "available": available_themes})
        self.theme_name = theme_name


class DataFetchError(MapToPosterError):
    """Failed to fetch OSM data."""

    def __init__(self, data_type: str, reason: str = "Unknown error"):
        message = f"Failed to fetch {data_type} data: {reason}"
        super().__init__(message, {"data_type": data_type, "reason": reason})
        self.data_type = data_type


class InvalidDistanceError(MapToPosterError):
    """Distance is out of valid range."""

    def __init__(self, distance: int, min_distance: int, max_distance: int):
        message = f"Distance {distance}m is out of range ({min_distance}m - {max_distance}m)"
        super().__init__(
            message,
            {"distance": distance, "min": min_distance, "max": max_distance}
        )
        self.distance = distance
        self.min_distance = min_distance
        self.max_distance = max_distance


class FontNotFoundError(MapToPosterError):
    """Required font file not found."""

    def __init__(self, font_path: str):
        message = f"Font file not found: {font_path}"
        super().__init__(message, {"path": font_path})
        self.font_path = font_path


class InvalidInputError(MapToPosterError):
    """Invalid user input."""

    def __init__(self, field: str, value: str, reason: str):
        message = f"Invalid {field}: '{value}' - {reason}"
        super().__init__(message, {"field": field, "value": value, "reason": reason})


class APITimeoutError(MapToPosterError):
    """External API request timed out."""

    def __init__(self, service: str, timeout: float):
        message = f"{service} API request timed out after {timeout}s"
        super().__init__(message, {"service": service, "timeout": timeout})


class PartialDataWarning(MapToPosterError):
    """
    Feature data couldn't be fetched but generation can continue.

    This is not a fatal error - the poster can still be generated
    but some features (like water or parks) may be missing.
    """

    def __init__(self, feature: str, reason: str):
        message = f"Could not fetch {feature}: {reason}. Continuing without it."
        super().__init__(message, {"feature": feature, "reason": reason})
        self.feature = feature
