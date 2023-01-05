use crate::todo;
use serde_json::Result;
use std::fs;

const TODO_FILE_NAME: &str = "data.json";

pub fn read_todo_list() -> todo::TodoList {
    let content = fs::read_to_string(TODO_FILE_NAME).unwrap();
    let content = content.trim();
    let todo_list: todo::TodoList = serde_json::from_str(content).unwrap();

    return todo_list;
}

pub fn write_todo_list(todo_list: &todo::TodoList) -> Result<()> {
    let json = serde_json::to_string(&todo_list)?;
    fs::write(TODO_FILE_NAME, json).expect("Failed to wirte new todos.");
    Ok(())
}
