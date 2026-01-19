# MapToPoster API - Rust Server (Memory-Efficient)
# Build stage
FROM rust:1.80-slim-bookworm AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Copy Rust project
COPY maptoposter-rs/Cargo.toml maptoposter-rs/Cargo.lock ./

# Create dummy source to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release && rm -rf src target/release/deps/maptoposter*

# Copy actual source and build
COPY maptoposter-rs/src ./src
RUN touch src/main.rs && cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies (including OpenSSL for native-tls)
RUN apt-get update && apt-get install -y \
    ca-certificates \
    curl \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the binary from builder
COPY --from=builder /app/target/release/maptoposter-rs ./maptoposter-rs

# Copy static assets
COPY themes ./themes
COPY fonts ./fonts
COPY frontend ./frontend

# Create directories for runtime
RUN mkdir -p static posters

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

# Run the server
CMD ["./maptoposter-rs"]
