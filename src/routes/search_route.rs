use std::sync::Arc;

use axum::extract::{Extension, Query};
use reqwest::Client;
use serde::Deserialize;

pub async fn search(info: Query<Params>, client: Extension<Arc<Client>>) -> &'static str {
    let response = client
        .get(format!("https://en.wikipedia.org/wiki/{}", info.query))
        .send()
        .await
        .and_then(|r| r.error_for_status());

    return match response {
        Ok(_) => "success",
        Err(_) => "oh snap",
    };
}

#[derive(Deserialize)]
pub struct Params {
    query: String,
}
