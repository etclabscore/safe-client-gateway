use crate::cache::manager::CacheManager;
use crate::cache::Cache;
use crate::config::scheme;
use crate::utils::http_client::HttpClient;
use rocket::request::{self, FromRequest, Request};
use std::sync::Arc;

pub struct RequestContext {
    pub request_id: String,
    pub host: String,
    http_client: Arc<dyn HttpClient>,
    cache_manager: Arc<dyn CacheManager>,
}

impl RequestContext {
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    pub fn cache_manager(&self) -> Arc<dyn CacheManager> {
        self.cache_manager.clone()
    }

    pub async fn cache(&self, chain_id: &str) -> Arc<dyn Cache> {
        self.cache_manager()
            .cache_for_chain_id(chain_id)
            .await
            .clone()
    }

    #[cfg(test)]
    pub async fn setup_for_test(
        request_id: String,
        host: String,
        http_client: &Arc<dyn HttpClient>,
        cache_manager: &Arc<dyn CacheManager>,
    ) -> Self {
        cache.invalidate_pattern("*").await;

        RequestContext {
            request_id,
            host,
            http_client: http_client.clone(),
            cache_manager: cache_manager.clone(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestContext {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cache_manager = request
            .rocket()
            .state::<Arc<dyn CacheManager>>()
            .expect("ServiceCache unavailable. Is it added to rocket instance?")
            .clone();
        let http_client = request
            .rocket()
            .state::<Arc<dyn HttpClient>>()
            .expect("HttpClient unavailable. Is it added to rocket instance?")
            .clone();
        let host = request
            .headers()
            .get_one("Host")
            .expect("Request Host must be available");

        let uri = request.uri().to_string();
        let host = format!("{}://{}", scheme(), host.to_string());

        return request::Outcome::Success(RequestContext {
            request_id: uri,
            host,
            cache_manager,
            http_client,
        });
    }
}
