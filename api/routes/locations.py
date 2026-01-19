"""Location search API routes using Nominatim."""

import time
from typing import List, Optional

from fastapi import APIRouter, Query
from pydantic import BaseModel

from src.maptoposter.config import settings
from src.maptoposter.logging_config import get_logger

logger = get_logger("api.locations")

router = APIRouter()


class LocationResult(BaseModel):
    """A location search result."""

    city: str
    country: str
    display_name: str
    lat: float
    lon: float
    type: str  # city, town, village, etc.


class LocationSearchResponse(BaseModel):
    """Response for location search."""

    results: List[LocationResult]
    query: str


# Simple in-memory cache to reduce Nominatim requests
_search_cache: dict = {}
_cache_ttl = 300  # 5 minutes


def _get_cached(query: str) -> Optional[List[LocationResult]]:
    """Get cached search results if available and not expired."""
    if query in _search_cache:
        result, timestamp = _search_cache[query]
        if time.time() - timestamp < _cache_ttl:
            return result
        else:
            del _search_cache[query]
    return None


def _set_cached(query: str, results: List[LocationResult]) -> None:
    """Cache search results."""
    # Limit cache size
    if len(_search_cache) > 1000:
        # Remove oldest entries
        oldest_keys = sorted(_search_cache.keys(), key=lambda k: _search_cache[k][1])[:100]
        for key in oldest_keys:
            del _search_cache[key]
    _search_cache[query] = (results, time.time())


@router.get("/search", response_model=LocationSearchResponse)
async def search_locations(
    q: str = Query(..., min_length=2, max_length=100, description="Search query"),
    limit: int = Query(8, ge=1, le=20, description="Maximum results to return")
) -> LocationSearchResponse:
    """
    Search for locations using Nominatim.

    Args:
        q: Search query (city name, address, etc.)
        limit: Maximum number of results

    Returns:
        List of matching locations
    """
    query = q.strip().lower()
    logger.info(f"Location search: '{query}' (limit={limit})")

    # Check cache first
    cached = _get_cached(query)
    if cached is not None:
        logger.debug(f"Cache hit for query: '{query}'")
        return LocationSearchResponse(results=cached[:limit], query=q)

    # Use Nominatim search API
    from geopy.geocoders import Nominatim
    from geopy.exc import GeocoderTimedOut, GeocoderServiceError

    geolocator = Nominatim(
        user_agent="maptoposter_search",
        timeout=settings.NOMINATIM_TIMEOUT
    )

    # Rate limiting
    time.sleep(settings.NOMINATIM_DELAY)

    try:
        # Search for places - prioritize cities, towns, villages
        locations = geolocator.geocode(
            query,
            exactly_one=False,
            limit=limit * 2,  # Get more results to filter
            addressdetails=True,
            featuretype=['city', 'town', 'village', 'hamlet', 'municipality']
        )
    except GeocoderTimedOut:
        logger.warning(f"Search timed out for: '{query}'")
        return LocationSearchResponse(results=[], query=q)
    except GeocoderServiceError as e:
        logger.error(f"Search service error: {e}")
        return LocationSearchResponse(results=[], query=q)
    except Exception as e:
        logger.error(f"Search error: {e}")
        return LocationSearchResponse(results=[], query=q)

    if not locations:
        logger.debug(f"No results found for: '{query}'")
        _set_cached(query, [])
        return LocationSearchResponse(results=[], query=q)

    # Parse results
    results: List[LocationResult] = []
    seen = set()  # Avoid duplicates

    for loc in locations:
        address = loc.raw.get('address', {})

        # Extract city name (try multiple fields)
        city = (
            address.get('city') or
            address.get('town') or
            address.get('village') or
            address.get('municipality') or
            address.get('hamlet') or
            address.get('county') or
            address.get('state') or
            loc.raw.get('name', '')
        )

        # Extract country
        country = address.get('country', '')

        if not city or not country:
            continue

        # Create unique key to avoid duplicates
        key = f"{city.lower()}|{country.lower()}"
        if key in seen:
            continue
        seen.add(key)

        # Determine place type
        place_type = loc.raw.get('type', 'place')
        if place_type in ['administrative', 'boundary']:
            place_type = loc.raw.get('class', 'city')

        results.append(LocationResult(
            city=city,
            country=country,
            display_name=f"{city}, {country}",
            lat=loc.latitude,
            lon=loc.longitude,
            type=place_type
        ))

        if len(results) >= limit:
            break

    logger.info(f"Found {len(results)} results for: '{query}'")
    _set_cached(query, results)

    return LocationSearchResponse(results=results, query=q)
