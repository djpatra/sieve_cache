use std::sync::{Mutex, RwLock};

use crate::cache_trait::SizeLimitedCache;

pub trait ShareableCache<Key, Value>: Send + Sync
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone
{
     fn get(&self, key: &Key) -> Option<Value>;

     fn set(&self, key: Key, value: Value);
}


pub struct SynchronizedShareableCache<Cache> {
    cache: Mutex<Cache>
}

pub struct LockBasedShareableCache<Cache> {
    cache: RwLock<Cache>
}

fn synchronized_cache<Cache, Key, Value>(cache: Cache) -> SynchronizedShareableCache<Cache> 
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
    Cache: SizeLimitedCache<Key, Value> 
{
    SynchronizedShareableCache { cache: Mutex::new(cache) }
}


fn locked_cache<Cache, Key, Value>(cache: Cache) -> LockBasedShareableCache<Cache> 
where
    Key: Eq + std::hash::Hash,
    Value: Clone,
    Cache: SizeLimitedCache<Key, Value> 
{
    LockBasedShareableCache { cache: RwLock::new(cache) }
}

impl<Key, Value, Cache> ShareableCache<Key, Value> for SynchronizedShareableCache<Cache> 
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
    Cache: SizeLimitedCache<Key, Value> + Send
{
    fn get(&self, key: &Key) -> Option<Value> {
        let mut res = None;
        if let Ok(lock) = self.cache.lock() {
            res = lock.get(key).cloned();
        }

        res
    }

    fn set(&self, key: Key, value: Value) {
        if let Ok(mut lock) = self.cache.lock() {
            lock.set(key, value);
        }
    }
}

impl<Key, Value, Cache> ShareableCache<Key, Value> for LockBasedShareableCache<Cache> 
where
    Key: Eq + std::hash::Hash + Clone,
    Value: Clone,
    Cache: SizeLimitedCache<Key, Value> + Sync + Send
{
    fn get(&self, key: &Key) -> Option<Value> {
        let mut res = None;
        if let Ok(lock) = self.cache.read() {
            res = lock.get(key).cloned()
        }

        res
    }

    fn set(&self, key: Key, value: Value) {
        if let Ok(mut lock) = self.cache.write() {
            lock.set(key, value);
        }
    }
}
