# MapToPoster Online Generator
# Multi-stage Dockerfile using UV for fast dependency management

FROM python:3.11-slim AS base

WORKDIR /app

# Install system dependencies for GDAL/geopandas
RUN apt-get update && apt-get install -y \
    libgdal-dev \
    libgeos-dev \
    libproj-dev \
    && rm -rf /var/lib/apt/lists/*

# Install UV
COPY --from=ghcr.io/astral-sh/uv:latest /uv /usr/local/bin/uv

# Copy dependency files first for better caching
COPY pyproject.toml uv.lock ./

# Install dependencies using UV (no venv needed in container)
RUN uv sync --frozen --no-dev --no-install-project

# Copy application code
COPY . .

# Install the project itself
RUN uv sync --frozen --no-dev

# Create directories
RUN mkdir -p /app/static /app/posters

# Production stage (default) - uses PORT env var for Railway/cloud deployments
FROM base AS production
ENV PORT=8000
EXPOSE ${PORT}
CMD uv run uvicorn api.main:app --host 0.0.0.0 --port ${PORT}

# API server stage (fixed port)
FROM base AS api
EXPOSE 8000
CMD ["uv", "run", "uvicorn", "api.main:app", "--host", "0.0.0.0", "--port", "8000"]

# Development stage with hot reload
FROM base AS dev
RUN uv sync --frozen
CMD ["uv", "run", "uvicorn", "api.main:app", "--host", "0.0.0.0", "--port", "8000", "--reload"]
