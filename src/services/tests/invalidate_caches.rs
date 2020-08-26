use crate::models::backend::webhooks::Payload;
use crate::services::hooks::invalidate_caches;
use mockall::predicate::*;
use crate::utils::cache::*;
use crate::utils::context::ContextCache;
use reqwest::blocking::Client;
use anyhow::Result;
use rocket::response::content;
use rocket_contrib::databases::redis::{self, pipe, Commands, Iter, PipelineCommands};
use serde::ser::Serialize;
use serde_json;

struct MockCache;

impl Cache for MockCache{
    fn fetch(&self, id: &str) -> Option<String> {
        unimplemented!()
    }

    fn create(&self, id: &String, dest: &String, timeout: usize) {
        unimplemented!()
    }

    fn invalidate_pattern(&self, pattern: &String) {
        unimplemented!()
    }

    fn _invalidate(&self, id: &String) {
        unimplemented!()
    }

    fn cache_resp<R>(&self, key: &String, timeout: usize, resp: impl Fn() -> Result<R>) -> Result<content::Json<String>> where R: Serialize {
        unimplemented!()
    }

    fn request_cached(&self, client: &Client, url: &String, timeout: usize) -> Result<String> {
        unimplemented!()
    }
}

struct TestContext(MockCache);

impl ContextCache for TestContext {
    type Cache = MockCache;

    fn cache(&self) -> &Self::Cache {
        &self.0
    }
}

#[test]
fn invalidate_with_empty_payload() {
    let payload = Payload {
        address: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        details: None,
    };

    let mock_cache = MockCache;
    // let mut mock_cache = MockCache::new();
    // mock_cache
    //     .expect_fetch()
    //     .times(0);
    // mock_cache
    //     .expect_create()
    //     .times(0);
    // mock_cache
    //     .expect_invalidate_pattern()
    //     .with(eq(String::from("*0x1230B3d59858296A31053C1b8562Ecf89A2f888b*")))
    //     .return_const(());
    // mock_cache
    //     .expect__invalidate()
    //     .times(0);

    let context = TestContext(mock_cache);

    invalidate_caches(&context, &payload);
}
