use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use std::sync::Arc;

pub enum ChainCache {
    Mainnet,
    Other,
}

#[rocket::async_trait]
pub trait CacheManager: Sync + Send {
    async fn cache_for_chain(&self, chain_cache: &ChainCache) -> Arc<dyn Cache>;
    async fn cache_for_chain_id(&self, chain_id: &str) -> Arc<dyn Cache>;
}

pub struct RedisCacheManager {
    mainnet_cache: Arc<dyn Cache>,
    default_cache: Arc<dyn Cache>,
}

impl RedisCacheManager {
    pub async fn new() -> Self {
        RedisCacheManager {
            mainnet_cache: Arc::new(ServiceCache::new_mainnet_cache().await),
            default_cache: Arc::new(ServiceCache::new_default_cache().await),
        }
    }
}

#[rocket::async_trait]
impl CacheManager for RedisCacheManager {
    async fn cache_for_chain(&self, chain_cache: &ChainCache) -> Arc<dyn Cache> {
        match chain_cache {
            ChainCache::Mainnet => self.mainnet_cache.clone(),
            ChainCache::Other => self.default_cache.clone(),
        }
    }

    async fn cache_for_chain_id(&self, chain_id: &str) -> Arc<dyn Cache> {
        let chain_cache = if chain_id == "1" {
            ChainCache::Mainnet
        } else {
            ChainCache::Other
        };
        let cache = &self.cache_for_chain(&chain_cache).await;
        cache.clone()
    }
}
