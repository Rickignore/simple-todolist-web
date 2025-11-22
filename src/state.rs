use std::{collections::HashMap, fs, path::Path};
use std::sync::Arc;
use parking_lot::Mutex;

use crate::models::Todo;
use serde_json;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<HashMap<String, Todo>>>,
    pub file_path: String,
}

impl AppState {
    pub fn new(file_path: &str) -> Self {
        // load JSON ke memory
        let todos = if Path::new(file_path).exists() {
            let data = fs::read_to_string(file_path).unwrap_or("[]".to_string());
            let vec: Vec<Todo> = serde_json::from_str(&data).unwrap_or(vec![]);
            let mut map = HashMap::new();
            for todo in vec {
                map.insert(todo.id.clone(), todo);
            }
            map
        } else {
            HashMap::new()
        };

        AppState {
            todos: Arc::new(Mutex::new(todos)),
            file_path: file_path.to_string(),
        }
    }

    pub fn save(&self) {
        let todos = self.todos.lock();
        let vec: Vec<Todo> = todos.values().cloned().collect();
        let data = serde_json::to_string_pretty(&vec).unwrap();
        fs::write(&self.file_path, data).unwrap();
    }
}