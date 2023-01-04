use std::fs;
use serde::{Deserialize, Serialize};

const TODO_FILE_NAME: &str = "data.json";

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub due: String,
    pub title: String,
    pub description: String
}

#[derive(Serialize, Deserialize)]
pub struct TodoList{
    pub todos: Vec<Todo>
}

pub fn read_todo_list() -> TodoList {
  let content = fs::read_to_string(TODO_FILE_NAME).unwrap();
  let content = content.trim();
  let todo_list: TodoList  = serde_json::from_str(content).unwrap();

  return todo_list;
}

pub fn show_all_todo_list(todo_list: &TodoList) {
    for todo in todo_list.todos.to_vec() {
        println!("- [] {}", todo.title);
    }
}

