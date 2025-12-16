use crate::Result;
use std::collections::HashMap;
use std::hash::Hash;
use std::sync::LazyLock;
use std::time::Duration;
use std::time::SystemTime;
use common::prelude::tokio::sync::RwLock;

pub struct Cache<Key: Hash, Value> {
    cache: LazyLock<RwLock<HashMap<Key, CacheEntry<Value>>>>,
    lifetime: Duration,
}

struct CacheEntry<Value> {
    entry: Value,
    expiry: SystemTime,
}

impl<Value> CacheEntry<Value> {
    pub fn expired(&self) -> bool {
        self.expiry < SystemTime::now()
    }
}

impl<Key: Hash + Eq + Clone, Value: Clone> Cache<Key, Value> {
    pub const fn new(lifetime: Duration) -> Self {
        Self {
            cache: LazyLock::new(|| RwLock::new(HashMap::new())),
            lifetime,
        }
    }

    pub async fn get<Fut: Future<Output = Result<Value>>>(&self, key: Key, or: impl Fn() -> Fut) -> Result<Value> {
        let read = self.cache.read().await;

        if let Some(value) = read.get(&key)
            && !value.expired() {
            log::trace!("Reusing cache value: {lifetime:?}", lifetime=&self.lifetime);
            return Ok(value.entry.clone());
        }

        drop(read);

        log::trace!("Fetching new value");
        let mut write = self.cache.write().await;
        let _ = write.insert(
            key.clone(),
            CacheEntry {
                entry: or().await?,
                expiry: SystemTime::now() + self.lifetime,
            },
        );

        Ok(write
            .get(&key)
            .expect("The key we just set apparently was empty")
            .entry
            .clone())
    }

    pub async fn get_last_modified_time(&self, key: &Key) -> Option<SystemTime> {
        self.cache.read().await.get(key).map(|i| i.expiry.clone())
    }
}
