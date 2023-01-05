use std::fs;
use crate::todo;

const TODO_FILE_NAME: &str = "data.json";

pub fn read_todo_list() -> todo::TodoList {
  let content = fs::read_to_string(TODO_FILE_NAME).unwrap();
  let content = content.trim();
  let todo_list: todo::TodoList  = serde_json::from_str(content).unwrap();

  return todo_list;
}

