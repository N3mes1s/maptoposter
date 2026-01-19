"""Theme API routes."""

from fastapi import APIRouter, HTTPException
from typing import List

from src.maptoposter.themes.loader import (
    get_available_themes,
    load_theme,
    get_all_themes_with_details
)
from src.maptoposter.exceptions import ThemeNotFoundError
from api.schemas.poster import ThemeInfo, ThemeListResponse

router = APIRouter()


@router.get("", response_model=ThemeListResponse)
async def list_themes():
    """Get list of all available themes."""
    themes_data = get_all_themes_with_details()

    themes = []
    for t in themes_data:
        themes.append(ThemeInfo(
            id=t.get('id', 'unknown'),
            name=t.get('name', t.get('id', 'Unknown')),
            description=t.get('description'),
            bg=t.get('bg', '#FFFFFF'),
            text=t.get('text', '#000000'),
            road_motorway=t.get('road_motorway', '#000000'),
            road_primary=t.get('road_primary', '#333333'),
            water=t.get('water', '#C0C0C0'),
            parks=t.get('parks', '#F0F0F0')
        ))

    return ThemeListResponse(themes=themes)


@router.get("/{theme_name}", response_model=ThemeInfo)
async def get_theme(theme_name: str):
    """Get details of a specific theme."""
    try:
        theme = load_theme(theme_name)
        return ThemeInfo(
            id=theme_name,
            name=theme.get('name', theme_name),
            description=theme.get('description'),
            bg=theme.get('bg', '#FFFFFF'),
            text=theme.get('text', '#000000'),
            road_motorway=theme.get('road_motorway', '#000000'),
            road_primary=theme.get('road_primary', '#333333'),
            water=theme.get('water', '#C0C0C0'),
            parks=theme.get('parks', '#F0F0F0')
        )
    except ThemeNotFoundError:
        raise HTTPException(status_code=404, detail=f"Theme '{theme_name}' not found")
