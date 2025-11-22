use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: String,
    pub text: String,
    pub done: bool,
}

#[derive(Deserialize)]
pub struct CreateTodo {
    pub text: String,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub text: Option<String>,
    pub done: Option<bool>,
}