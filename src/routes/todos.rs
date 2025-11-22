use axum::{
    Router,
    extract::{State, Path},
    routing::{get, post, put, delete},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use crate::state::AppState;
use crate::models::{Todo, CreateTodo, UpdateTodo};

pub fn routes(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(list).post(create))
        .route("/todos/:id", get(read).put(update).delete(remove))
        .with_state(state)
}

async fn list(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock();
    Json(todos.values().cloned().collect())
}

async fn create(State(state): State<AppState>, Json(payload): Json<CreateTodo>) -> (StatusCode, Json<Todo>) {
    let id = Uuid::new_v4().to_string();
    let todo = Todo { id: id.clone(), text: payload.text, done: false };

    state.todos.lock().insert(id, todo.clone());
    state.save(); // save to JSON

    (StatusCode::CREATED, Json(todo))
}

async fn read(State(state): State<AppState>, Path(id): Path<String>) -> Result<Json<Todo>, StatusCode> {
    let todos = state.todos.lock();
    todos.get(&id).cloned().map(Json).ok_or(StatusCode::NOT_FOUND)
}

async fn update(State(state): State<AppState>, Path(id): Path<String>, Json(payload): Json<UpdateTodo>) -> Result<Json<Todo>, StatusCode> {
    let mut todos = state.todos.lock();
    let todo = todos.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(t) = payload.text { todo.text = t; }
    if let Some(done) = payload.done { todo.done = done; }

    state.save(); // save to JSON
    Ok(Json(todo.clone()))
}

async fn remove(State(state): State<AppState>, Path(id): Path<String>) -> StatusCode {
    state.todos.lock().remove(&id);
    state.save(); // save to JSON
    StatusCode::NO_CONTENT
}