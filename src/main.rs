mod state;
mod models;
mod routes;

use axum::Router;
use routes::create_routes;
use state::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    let app_state = AppState::new("todos.json"); // JSON file

    let app = create_routes(app_state);

    println!("Server berjalan di http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

axum::serve(listener, app.into_make_service())
    .await
    .unwrap();

}