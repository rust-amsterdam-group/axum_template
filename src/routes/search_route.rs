use std::sync::Arc;

use axum::extract::{Extension, Query};
use reqwest::Client;
use serde::Deserialize;

pub async fn search(params: Query<Params>, client: Extension<Arc<Client>>) -> &'static str {
    client
        .get(format!("https://en.wikipedia.org/wiki/{}", params.query))
        .send()
        .await
        .and_then(|r| r.error_for_status())
        .map_or("oh snap!", |_| "success")
}

#[derive(Deserialize)]
pub struct Params {
    query: String,
}
