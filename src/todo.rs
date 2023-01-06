use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: usize,
    pub due: String,
    pub title: String,
    pub description: String,
    pub is_completed: bool,
}

impl Todo {
    pub fn completed(&mut self) {
        self.is_completed = true;
    }
}

impl PartialEq for Todo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

#[derive(Serialize, Deserialize)]
pub struct TodoList {
    pub todos: Vec<Todo>,
}

pub fn show_all_todo_list(todo_list: &TodoList) {
    for todo in todo_list.todos.to_vec() {
        println!("- [] |{}| {}", todo.id, todo.title);
    }
}

fn generate_id(current_todo_list: &TodoList) -> usize {
    let todos = current_todo_list.todos.clone();
    let mut current_id: usize = 0;
    for todo in todos {
        if todo.id > current_id {
            current_id = todo.id;
        }
    }

    return current_id + 1;
}

pub fn add_new_todo(
    due: &str,
    title: &str,
    description: &str,
    current_todo_list: &TodoList,
) -> TodoList {
    let id = generate_id(&current_todo_list);
    let todo: Todo = Todo {
        id,
        due: String::from(due),
        title: String::from(title),
        description: String::from(description),
        is_completed: false,
    };

    let mut new_todo_list = current_todo_list.todos.clone();
    new_todo_list.push(todo);
    return TodoList {
        todos: new_todo_list,
    };
}
