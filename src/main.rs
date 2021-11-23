use std::net::SocketAddr;
use std::sync::Arc;

use axum::{routing::get, AddExtensionLayer, Router};

mod routes;

#[tokio::main]
async fn main() {
    let client = Arc::new(reqwest::Client::new());

    let app = Router::new()
        .route("/", get(routes::index))
        .route("/search", get(routes::search))
        .layer(AddExtensionLayer::new(client));

    let address = match std::env::var("PORT") {
        Ok(port) => SocketAddr::from(([0, 0, 0, 0], port.parse::<u16>().unwrap())),
        Err(_e) => SocketAddr::from(([127, 0, 0, 1], 8080)),
    };

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
