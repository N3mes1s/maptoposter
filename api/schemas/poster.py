"""Pydantic schemas for poster API."""

from typing import Optional, List
from pydantic import BaseModel, Field, field_validator


class PosterCreateRequest(BaseModel):
    """Request body for creating a new poster."""
    city: str = Field(..., min_length=1, max_length=100, description="City name")
    country: str = Field(..., min_length=1, max_length=100, description="Country name")
    theme: str = Field(default="feature_based", description="Theme name")
    distance: int = Field(default=15000, ge=2000, le=50000, description="Map radius in meters")

    @field_validator('city', 'country')
    @classmethod
    def strip_whitespace(cls, v):
        return v.strip()


class PosterCreateResponse(BaseModel):
    """Response after creating a poster job."""
    job_id: str
    status: str
    estimated_time: int  # seconds


class JobStatus(BaseModel):
    """Current status of a poster generation job."""
    job_id: str
    status: str  # queued, processing, completed, failed
    progress: float = 0
    current_step: Optional[str] = None
    message: Optional[str] = None
    download_url: Optional[str] = None
    preview_url: Optional[str] = None
    error: Optional[str] = None


class ThemeInfo(BaseModel):
    """Theme information for the API."""
    id: str
    name: str
    description: Optional[str] = None
    bg: str
    text: str
    road_motorway: str
    road_primary: str
    water: str
    parks: str


class ThemeListResponse(BaseModel):
    """Response with list of available themes."""
    themes: List[ThemeInfo]
