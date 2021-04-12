use crate::cache::cache::Cache;
use crate::cache::cache_operations::{CacheResponse, RequestCached};
use crate::cache::inner_cache::CachedWithCode;
use crate::utils::errors::{ApiError, ApiResult};
use rocket::response::content;
use serde::Serialize;
use std::time::Duration;

pub const CACHE_REQS_PREFIX: &'static str = "c_reqs";
pub const CACHE_RESP_PREFIX: &'static str = "c_resp";

pub(super) fn cache_response<S>(
    cache: &impl Cache,
    cache_response: &CacheResponse<S>,
) -> ApiResult<content::Json<String>>
where
    S: Serialize,
{
    let cache_key = format!("{}_{}", CACHE_RESP_PREFIX, cache_response.key);
    let cached = cache.fetch(&cache_key);
    match cached {
        Some(value) => Ok(content::Json(value)),
        None => {
            let resp_string = serde_json::to_string(&cache_response.generate()?)?;
            cache.create(&cache_key, &resp_string, cache_response.duration);
            Ok(content::Json(resp_string))
        }
    }
}

pub(super) fn request_cached(
    cache: &dyn Cache,
    client: &reqwest::blocking::Client,
    operation: &RequestCached,
) -> ApiResult<String> {
    request_cached_priv(
        cache,
        client,
        &operation.url,
        operation.cache_duration,
        operation.error_cache_duration,
        operation.cache_all_errors,
        operation.request_timeout,
    )
}

fn request_cached_priv(
    cache: &dyn Cache,
    client: &reqwest::blocking::Client,
    url: &str,
    cache_duration: usize,
    error_cache_duration: usize,
    cache_all_errors: bool,
    request_timeout: u64,
) -> ApiResult<String> {
    let cache_key = format!("{}_{}", CACHE_REQS_PREFIX, &url);
    match cache.fetch(&cache_key) {
        Some(cached) => CachedWithCode::split(&cached).to_result(),
        None => {
            let mut request = client.get(url);
            if request_timeout > 0 {
                request = request.timeout(Duration::from_millis(request_timeout));
            }
            let response = request.send().map_err(|err| {
                if cache_all_errors {
                    cache.create(
                        &cache_key,
                        &CachedWithCode::join(500, &format!("{:?}", &err)),
                        error_cache_duration,
                    );
                }
                err
            })?;
            let status_code = response.status().as_u16();

            // Early return and no caching if the error is a 500 or greater
            let is_server_error = response.status().is_server_error();
            if !cache_all_errors && is_server_error {
                return Err(ApiError::from_backend_error(
                    42,
                    &format!("Got server error for {}", response.text()?),
                ));
            }

            let is_client_error = response.status().is_client_error();
            let raw_data = response.text()?;

            if is_client_error || is_server_error {
                cache.create(
                    &cache_key,
                    &CachedWithCode::join(status_code, &raw_data),
                    error_cache_duration,
                );
                Err(ApiError::from_backend_error(status_code, &raw_data))
            } else {
                cache.create(
                    &cache_key,
                    &CachedWithCode::join(status_code, &raw_data),
                    cache_duration,
                );
                Ok(raw_data.to_string())
            }
        }
    }
}
