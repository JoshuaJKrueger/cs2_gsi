mod handler;
mod logger;

use axum::{Router, routing::post};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new().route("/", post(handler::handle_gsi_post));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .expect("Failed to bind to port 3000");

    println!("GSI Server started");
    println!("Waiting for Game State Integration POST requests");

    axum::serve(listener, app).await.expect("Server error");
}
