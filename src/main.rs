use std::{env, fs};
use serde::{Deserialize, Serialize};

const TODO_FILE_NAME: &str = "data.json";

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    due: String,
    title: String,
    description: String
}

#[derive(Serialize, Deserialize)]
struct TodoList{
    todos: Vec<Todo>
}

fn read_todo_list() -> TodoList {
  let content = fs::read_to_string(TODO_FILE_NAME).unwrap();
  let content = content.trim();
  let todo_list: TodoList  = serde_json::from_str(content).unwrap();

  return todo_list;
}

fn show_all_todo_list(todo_list: &TodoList) {
    for todo in todo_list.todos.to_vec() {
        println!("- [] {}", todo.title);
    }
}

fn print_usage() {
    const USAGE: &str = "[USAGE] todo [--help] <command>";
    println!("{}", USAGE);
}

fn match_option(option: &str) {
    match option {
        "--help" => print_usage(),
        _ => print_usage(),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let todo_list = read_todo_list();
    if args.len() < 2 {
        show_all_todo_list(&todo_list);
        return;
    }

    let option = &args[1];
    match_option(option);
}
