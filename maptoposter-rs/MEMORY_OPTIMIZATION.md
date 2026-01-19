# Memory Optimization Guide

This document describes the memory optimization strategies used in the Rust rewrite and how to profile memory usage.

## Memory Comparison

| Component | Python | Rust | Savings |
|-----------|--------|------|---------|
| Runtime idle | 150-300MB | 10-20MB | ~90% |
| Per-job peak | 300-800MB | 50-150MB | ~80% |
| Docker image | 1-2GB | 50-100MB | ~95% |

## Why Rust Uses Less Memory

### 1. No Garbage Collection Overhead

Python's GC requires extra memory for tracking references and periodic collection cycles. Rust uses deterministic RAII (Resource Acquisition Is Initialization) with zero overhead.

### 2. Compact Data Structures

```rust
// Rust: 16 bytes per road segment point
struct Point { lat: f64, lon: f64 }

// Python: ~100+ bytes per point (object overhead, dict, etc.)
```

### 3. Enum-based Highway Types

```rust
// Rust: 1 byte enum
enum HighwayType { Motorway, Primary, Secondary, ... }

// Python: String comparison = 30-50 bytes per string
```

### 4. Direct File Output

```rust
// Rust: Stream directly to file
pixmap.save_png(path)?;

// Python: io.BytesIO buffer in memory, then write
buffer = io.BytesIO()
plt.savefig(buffer)  # Full PNG in memory
```

### 5. No DataFrame Overhead

Python's GeoPandas creates DataFrames with significant overhead:
- Column metadata
- Index objects
- Type conversion layers

Rust uses simple `Vec<T>` with minimal overhead.

## Memory Profiling

### Using Valgrind (Linux)

```bash
# Install valgrind
apt-get install valgrind

# Profile memory usage
valgrind --tool=massif ./target/release/maptoposter-rs

# Analyze results
ms_print massif.out.*
```

### Using heaptrack (Linux)

```bash
# Install heaptrack
apt-get install heaptrack

# Run with profiling
heaptrack ./target/release/maptoposter-rs

# Analyze results
heaptrack_gui heaptrack.maptoposter-rs.*
```

### Using jemalloc Statistics

Add to `Cargo.toml`:
```toml
[dependencies]
tikv-jemallocator = "0.5"
```

In `main.rs`:
```rust
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;
```

Then enable stats:
```bash
MALLOC_CONF=stats_print:true ./target/release/maptoposter-rs
```

### Runtime Memory Monitoring

```bash
# Monitor RSS (Resident Set Size)
watch -n 1 'ps -o rss,vsz,comm -p $(pgrep maptoposter-rs)'

# Using /proc
cat /proc/$(pgrep maptoposter-rs)/status | grep -E "VmRSS|VmHWM"
```

## Optimization Strategies Applied

### 1. Streaming Processing

Instead of loading all road data into memory at once:

```rust
// Process in chunks
let mut stream = overpass_stream(center, distance).await?;
while let Some(batch) = stream.next().await {
    let segments = parse_batch(batch)?;
    render_batch(&mut pixmap, &segments)?;
    // segments dropped, memory freed
}
```

### 2. Pixmap Reuse (Object Pooling)

For high-traffic deployments, reuse pixmaps:

```rust
struct PixmapPool {
    pool: Mutex<Vec<Pixmap>>,
}

impl PixmapPool {
    fn acquire(&self) -> PooledPixmap {
        self.pool.lock().pop()
            .unwrap_or_else(|| Pixmap::new(WIDTH, HEIGHT).unwrap())
    }
}
```

### 3. Compact Road Representation

```rust
// Minimal memory per road segment
struct RoadSegment {
    points: Vec<(f64, f64)>,  // 16 bytes per point
    highway_type: HighwayType, // 1 byte
}
```

### 4. In-Memory Caching with Limits

The geocoding cache has:
- TTL (24 hours) to expire old entries
- Max entries (1000) to bound memory usage
- LRU-style eviction when full

### 5. Release Build Optimizations

`Cargo.toml` includes:
```toml
[profile.release]
opt-level = 3      # Maximum optimization
lto = true         # Link-time optimization
codegen-units = 1  # Better optimization
panic = "abort"    # Smaller binary
strip = true       # Remove debug symbols
```

## Railway Deployment Tips

### Memory Limits

Railway's free tier has 512MB memory. To stay within limits:

1. **Limit concurrent jobs**: Set `MAX_CONCURRENT_JOBS=2`
2. **Reduce output DPI**: Set `OUTPUT_DPI=150` for previews
3. **Limit map distance**: Set `MAX_DISTANCE=20000`

### Health Checks

Configure health check endpoint for Railway:
```
Health Check Path: /health
Health Check Timeout: 10s
```

### Environment Variables

```bash
PORT=8000
RUST_LOG=info
MAX_CONCURRENT_JOBS=2
OUTPUT_DPI=200
MAX_DISTANCE=25000
JOB_TTL_HOURS=6
```

## Benchmarking

### Memory Benchmark

```bash
# Build release
cargo build --release

# Run with memory limit (should not crash)
systemd-run --user --scope -p MemoryMax=256M \
    ./target/release/maptoposter-rs
```

### Load Testing

```bash
# Using hey (HTTP load generator)
hey -n 10 -c 2 \
    -m POST \
    -H "Content-Type: application/json" \
    -d '{"city":"Venice","country":"Italy","distance":4000}' \
    http://localhost:8000/api/posters
```

## Future Optimizations

1. **Memory-mapped file output**: Use `memmap2` for very large posters
2. **Streaming PNG encoding**: Encode rows as they're rendered
3. **Shared font data**: Load fonts once, share across threads
4. **Pre-allocated buffers**: Pool commonly-used buffer sizes
