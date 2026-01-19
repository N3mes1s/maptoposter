"""Geocoding utilities using Nominatim."""

import time
from typing import Tuple

from geopy.geocoders import Nominatim
from geopy.exc import GeocoderTimedOut, GeocoderServiceError

from ..config import settings
from ..exceptions import GeocodingError, APITimeoutError
from ..logging_config import get_logger

logger = get_logger("geocoding")


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
        APITimeoutError: If the geocoding service times out
    """
    query = f"{city}, {country}"
    logger.info(f"Geocoding location: {query}")

    geolocator = Nominatim(
        user_agent="maptoposter_generator",
        timeout=settings.NOMINATIM_TIMEOUT
    )

    # Add a small delay to respect Nominatim's usage policy
    logger.debug(f"Rate limiting: waiting {settings.NOMINATIM_DELAY}s before request")
    time.sleep(settings.NOMINATIM_DELAY)

    try:
        logger.debug(f"Sending geocode request (timeout={settings.NOMINATIM_TIMEOUT}s)")
        location = geolocator.geocode(query)
    except GeocoderTimedOut as e:
        logger.error(f"Geocoding timed out after {settings.NOMINATIM_TIMEOUT}s: {e}")
        raise APITimeoutError("Nominatim", settings.NOMINATIM_TIMEOUT)
    except GeocoderServiceError as e:
        logger.error(f"Geocoding service error: {e}")
        raise GeocodingError(city, country, f"Service error: {e}")
    except Exception as e:
        logger.error(f"Unexpected geocoding error: {e}")
        raise GeocodingError(city, country, str(e))

    if location:
        lat, lon = location.latitude, location.longitude
        logger.info(f"Successfully geocoded {query} -> ({lat:.4f}, {lon:.4f})")
        return (lat, lon)
    else:
        logger.warning(f"No results found for location: {query}")
        raise GeocodingError(city, country, "No results found")


def format_coordinates(lat: float, lon: float) -> str:
    """
    Format coordinates as a human-readable string.

    Args:
        lat: Latitude
        lon: Longitude

    Returns:
        Formatted coordinate string (e.g., "45.4408° N / 12.3155° E")
    """
    lat_dir = "N" if lat >= 0 else "S"
    lon_dir = "E" if lon >= 0 else "W"
    return f"{abs(lat):.4f}° {lat_dir} / {abs(lon):.4f}° {lon_dir}"
