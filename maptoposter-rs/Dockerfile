# Build stage
FROM rust:1.75-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create a dummy project to cache dependencies
COPY Cargo.toml Cargo.lock* ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src target/release/deps/maptoposter*

# Build the actual application
COPY src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary
COPY --from=builder /app/target/release/maptoposter-rs .

# Copy static assets (from parent directory)
# These will be mounted or copied during deployment
COPY --from=builder /app/target/release/maptoposter-rs .

# Create directories for runtime
RUN mkdir -p static posters themes fonts frontend

# Set environment variables
ENV PORT=8000
ENV RUST_LOG=info
ENV THEMES_DIR=/app/themes
ENV FONTS_DIR=/app/fonts
ENV STATIC_DIR=/app/static
ENV FRONTEND_DIR=/app/frontend

EXPOSE ${PORT}

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:${PORT}/health || exit 1

CMD ["./maptoposter-rs"]
