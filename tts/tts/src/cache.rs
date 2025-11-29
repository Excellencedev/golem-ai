// Voice caching utilities for TTS providers
use std::cell::RefCell;
use std::time::{Duration, SystemTime};

/// Thread-local cache for voice lists to reduce API calls
#[derive(Clone)]
pub struct CacheEntry<T> {
    pub data: T,
    pub cached_at: SystemTime,
    pub ttl: Duration,
}

impl<T> CacheEntry<T> {
    pub fn new(data: T, ttl: Duration) -> Self {
        Self {
            data,
            cached_at: SystemTime::now(),
            ttl,
        }
    }

    pub fn is_expired(&self) -> bool {
        SystemTime::now()
            .duration_since(self.cached_at)
            .map(|elapsed| elapsed > self.ttl)
            .unwrap_or(true)
    }

    pub fn refresh(&mut self, data: T) {
        self.data = data;
        self.cached_at = SystemTime::now();
    }
}

/// Simple cache for voice data
pub struct VoiceCache<T> {
    cache: RefCell<Option<CacheEntry<T>>>,
    default_ttl: Duration,
}

impl<T: Clone> VoiceCache<T> {
    pub fn new(ttl_seconds: u64) -> Self {
        Self {
            cache: RefCell::new(None),
            default_ttl: Duration::from_secs(ttl_seconds),
        }
    }

    pub fn get(&self) -> Option<T> {
        let cache = self.cache.borrow();
        cache.as_ref().and_then(|entry| {
            if !entry.is_expired() {
                Some(entry.data.clone())
            } else {
                None
            }
        })
    }

    pub fn set(&self, data: T) {
        let mut cache = self.cache.borrow_mut();
        match cache.as_mut() {
            Some(entry) => entry.refresh(data),
            None => *cache = Some(CacheEntry::new(data, self.default_ttl)),
        }
    }

    pub fn clear(&self) {
        *self.cache.borrow_mut() = None;
    }

    pub fn is_valid(&self) -> bool {
        self.cache
            .borrow()
            .as_ref()
            .map(|entry| !entry.is_expired())
            .unwrap_or(false)
    }
}

impl<T: Clone> Default for VoiceCache<T> {
    fn default() -> Self {
        Self::new(300) // 5 minutes default TTL
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cache_entry_expiration() {
        let entry = CacheEntry::new("test".to_string(), Duration::from_millis(100));
        assert!(!entry.is_expired());

        std::thread::sleep(Duration::from_millis(150));
        assert!(entry.is_expired());
    }

    #[test]
    fn test_voice_cache() {
        let cache: VoiceCache<Vec<String>> = VoiceCache::new(1);

        assert!(cache.get().is_none());

        cache.set(vec!["voice1".to_string(), "voice2".to_string()]);
        assert!(cache.is_valid());

        let cached = cache.get();
        assert!(cached.is_some());
        assert_eq!(cached.unwrap().len(), 2);

        std::thread::sleep(Duration::from_secs(2));
        assert!(!cache.is_valid());
        assert!(cache.get().is_none());
    }
}
