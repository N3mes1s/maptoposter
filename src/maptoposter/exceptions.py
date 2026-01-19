"""Custom exceptions for MapToPoster."""


class MapToPosterError(Exception):
    """Base exception for MapToPoster."""
    pass


class GeocodingError(MapToPosterError):
    """Failed to geocode location."""
    pass


class ThemeNotFoundError(MapToPosterError):
    """Theme does not exist."""
    pass


class DataFetchError(MapToPosterError):
    """Failed to fetch OSM data."""
    pass


class InvalidDistanceError(MapToPosterError):
    """Distance is out of valid range."""
    pass


class FontNotFoundError(MapToPosterError):
    """Required font file not found."""
    pass
