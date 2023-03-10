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

    pub fn format_for_out(&self) -> String {
        let list_item = if *&self.is_completed { "[x]" } else { "[ ]" };
        let pad = "          ";
        format!(
            "- {} |{}| {}\n{}{}\n{}{}",
            list_item, &self.id, &self.title, pad, &self.due, pad, &self.description
        )
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
        println!("{}", todo.format_for_out());
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

pub fn remove_todo(id: usize, current_todo_list: &TodoList) -> TodoList {
    let removed_todos = current_todo_list
        .todos
        .iter()
        .filter(|t| t.id != id)
        .cloned()
        .collect();
    return TodoList {
        todos: removed_todos,
    };
}

#[cfg(test)]
mod tests {
    use super::{add_new_todo, generate_id, remove_todo, Todo, TodoList};

    fn generate_todo() -> Todo {
        return Todo {
            id: 1,
            due: "2099/01/01".to_string(),
            title: "test".to_string(),
            description: "test".to_string(),
            is_completed: false,
        };
    }

    #[test]
    fn test_generate_id() {
        let t = generate_todo();
        let tl = TodoList { todos: vec![t] };
        assert_eq!(generate_id(&tl), 2);
        let tl = TodoList { todos: vec![] };
        assert_eq!(generate_id(&tl), 1);
    }

    #[test]
    fn test_add_new_todo() {
        let tl = TodoList { todos: vec![] };
        assert_eq!(
            add_new_todo("2099/01/01", "test", "test", &tl).todos.len(),
            1
        );

        let t = generate_todo();
        let tl = TodoList { todos: vec![t] };
        assert_eq!(
            add_new_todo("2099/01/01", "test", "test", &tl).todos.len(),
            2
        );
    }

    #[test]
    fn test_remove_todo() {
        let t = generate_todo();
        let tl = TodoList { todos: vec![t] };
        assert_eq!(remove_todo(1, &tl).todos.len(), 0);
    }

    #[test]
    fn test_completed() {
        let mut t = generate_todo();
        assert_eq!(t.is_completed, false);
        t.completed();
        assert_eq!(t.is_completed, true);
    }
}
