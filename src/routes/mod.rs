pub mod todos;

use axum::Router;
use tower_http::services::ServeDir;

use crate::state::AppState;

pub fn create_routes(state: AppState) -> Router {
    let api = Router::new()
        .nest("/api", todos::routes(state.clone()));

    let static_files = ServeDir::new("static");

    Router::new()
    .nest_service("/", static_files)
    .merge(api)

}