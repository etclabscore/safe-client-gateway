use crate::cache::cache_operations::CacheResponse;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/health")]
pub async fn health(context: RequestContext) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context, "1") // TODO remove hardcoded chain_id hack
        .resp_generator(|| async { Ok(String::new()) })
        .execute()
        .await
}
