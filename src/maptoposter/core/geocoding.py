"""Geocoding utilities using Nominatim."""

import time
from typing import Tuple
from geopy.geocoders import Nominatim

from ..config import settings
from ..exceptions import GeocodingError


def get_coordinates(city: str, country: str) -> Tuple[float, float]:
    """
    Fetches coordinates for a given city and country using geopy.
    Includes rate limiting to be respectful to the geocoding service.

    Args:
        city: City name
        country: Country name

    Returns:
        Tuple of (latitude, longitude)

    Raises:
        GeocodingError: If coordinates cannot be found
    """
    geolocator = Nominatim(user_agent="maptoposter_generator")

    # Add a small delay to respect Nominatim's usage policy
    time.sleep(settings.NOMINATIM_DELAY)

    location = geolocator.geocode(f"{city}, {country}")

    if location:
        return (location.latitude, location.longitude)
    else:
        raise GeocodingError(f"Could not find coordinates for {city}, {country}")


def format_coordinates(lat: float, lon: float) -> str:
    """
    Format coordinates as a human-readable string.

    Args:
        lat: Latitude
        lon: Longitude

    Returns:
        Formatted coordinate string
    """
    lat_dir = "N" if lat >= 0 else "S"
    lon_dir = "E" if lon >= 0 else "W"
    return f"{abs(lat):.4f}° {lat_dir} / {abs(lon):.4f}° {lon_dir}"
