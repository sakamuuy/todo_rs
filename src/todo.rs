use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub due: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

pub fn show_all_todo_list(todo_list: &TodoList) {
    for todo in todo_list.todos.to_vec() {
        println!("- [] {}", todo.title);
    }
}

pub fn add_new_todo(
    due: &str,
    title: &str,
    description: &str,
    current_todo_list: &TodoList,
) -> TodoList {
    let mut id: usize = 1;
    for todo in current_todo_list.todos.into_iter() {
        if todo.id == id {
            id = todo.id + 1;
        }
    }
    let todo: Todo = Todo {
        id: id,
        due: String::from(due),
        title: String::from(title),
        description: String::from(description),
        is_completed: false
    };
    let mut new_todo_list = current_todo_list.todos.clone();
    new_todo_list.push(todo);
    return TodoList {
        todos: new_todo_list,
    };
}
