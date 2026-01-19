# MapToPoster API - Production Dockerfile
FROM python:3.11-slim

WORKDIR /app

# Install system dependencies for GDAL/geopandas
RUN apt-get update && apt-get install -y \
    libgdal-dev \
    libgeos-dev \
    libproj-dev \
    gcc \
    g++ \
    && rm -rf /var/lib/apt/lists/*

# Copy requirements first for better caching
COPY requirements.txt .

# Install Python dependencies
RUN pip install --no-cache-dir -r requirements.txt

# Copy application code
COPY . .

# Create necessary directories
RUN mkdir -p /app/static /app/posters

# Set PYTHONPATH for src.maptoposter imports
ENV PYTHONPATH=/app

# Default port (Railway overrides via $PORT)
ENV PORT=8000

EXPOSE ${PORT}

# Use shell form to allow $PORT expansion
CMD python -m uvicorn api.main:app --host 0.0.0.0 --port $PORT
