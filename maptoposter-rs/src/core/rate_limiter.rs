use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

use parking_lot::Mutex;
use tokio::time::sleep;

/// A simple rate limiter that ensures minimum delay between requests
pub struct RateLimiter {
    last_request: Mutex<HashMap<String, Instant>>,
    min_delay: Duration,
}

impl RateLimiter {
    /// Create a new rate limiter with the specified minimum delay between requests
    pub fn new(min_delay_secs: f64) -> Self {
        Self {
            last_request: Mutex::new(HashMap::new()),
            min_delay: Duration::from_secs_f64(min_delay_secs),
        }
    }

    /// Wait if necessary to respect rate limits, then record this request
    pub async fn wait(&self, key: &str) {
        let now = Instant::now();
        let wait_duration = {
            let mut last = self.last_request.lock();
            if let Some(last_time) = last.get(key) {
                let elapsed = now.duration_since(*last_time);
                if elapsed < self.min_delay {
                    Some(self.min_delay - elapsed)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(duration) = wait_duration {
            tracing::debug!("Rate limiting: waiting {:?} for {}", duration, key);
            sleep(duration).await;
        }

        // Record this request
        let mut last = self.last_request.lock();
        last.insert(key.to_string(), Instant::now());
    }
}

/// Simple in-memory cache with TTL
pub struct Cache<V> {
    entries: Mutex<HashMap<String, CacheEntry<V>>>,
    ttl: Duration,
    max_entries: usize,
}

struct CacheEntry<V> {
    value: V,
    inserted_at: Instant,
}

impl<V: Clone> Cache<V> {
    /// Create a new cache with the specified TTL and max entries
    pub fn new(ttl_secs: u64, max_entries: usize) -> Self {
        Self {
            entries: Mutex::new(HashMap::new()),
            ttl: Duration::from_secs(ttl_secs),
            max_entries,
        }
    }

    /// Get a value from the cache if it exists and hasn't expired
    pub fn get(&self, key: &str) -> Option<V> {
        let mut entries = self.entries.lock();

        if let Some(entry) = entries.get(key) {
            if entry.inserted_at.elapsed() < self.ttl {
                return Some(entry.value.clone());
            } else {
                // Entry expired, remove it
                entries.remove(key);
            }
        }
        None
    }

    /// Insert a value into the cache
    pub fn insert(&self, key: String, value: V) {
        let mut entries = self.entries.lock();

        // If we're at capacity, remove oldest entries
        if entries.len() >= self.max_entries {
            self.evict_oldest(&mut entries);
        }

        entries.insert(key, CacheEntry {
            value,
            inserted_at: Instant::now(),
        });
    }

    /// Remove expired and oldest entries to make room
    fn evict_oldest(&self, entries: &mut HashMap<String, CacheEntry<V>>) {
        let now = Instant::now();

        // First, remove all expired entries
        entries.retain(|_, entry| entry.inserted_at.elapsed() < self.ttl);

        // If still at capacity, remove oldest entries
        if entries.len() >= self.max_entries {
            let mut oldest_key: Option<String> = None;
            let mut oldest_time = now;

            for (key, entry) in entries.iter() {
                if entry.inserted_at < oldest_time {
                    oldest_time = entry.inserted_at;
                    oldest_key = Some(key.clone());
                }
            }

            if let Some(key) = oldest_key {
                entries.remove(&key);
            }
        }
    }

    /// Clear all entries from the cache
    pub fn clear(&self) {
        self.entries.lock().clear();
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.entries.lock().len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.lock().is_empty()
    }
}

/// Global rate limiters for external APIs
pub struct ApiRateLimiters {
    pub nominatim: RateLimiter,
    pub overpass: RateLimiter,
}

impl ApiRateLimiters {
    pub fn new(nominatim_delay: f64, overpass_delay: f64) -> Self {
        Self {
            nominatim: RateLimiter::new(nominatim_delay),
            overpass: RateLimiter::new(overpass_delay),
        }
    }
}

impl Default for ApiRateLimiters {
    fn default() -> Self {
        Self::new(1.0, 0.5) // Nominatim: 1 req/sec, Overpass: 2 req/sec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_basic() {
        let cache: Cache<String> = Cache::new(60, 100);

        cache.insert("key1".to_string(), "value1".to_string());
        assert_eq!(cache.get("key1"), Some("value1".to_string()));
        assert_eq!(cache.get("key2"), None);
    }

    #[test]
    fn test_cache_max_entries() {
        let cache: Cache<i32> = Cache::new(60, 2);

        cache.insert("key1".to_string(), 1);
        cache.insert("key2".to_string(), 2);
        cache.insert("key3".to_string(), 3);

        // Should have evicted oldest entry
        assert!(cache.len() <= 2);
    }
}
