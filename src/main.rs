use std::env;
mod io;
mod todo;

enum Command {
    List,
    Add,
    Complete,
    Patch,
    Delete,
    Noop,
}

struct Input {
    command: Command,
}

fn print_usage() {
    println!(
        "{}{}{}{}{}{}{}",
        "[USAGE] cargo run -- <command>\n",
        "<command>\n",
        "add      - Add a new todo.\n",
        "list     - List all todos.\n",
        "complete - Complete a todo.\n",
        "patch    - Edit a todo.\n",
        "delete   - Delete a todo.\n"
    );
}

fn input_prompt(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input error");
    input.trim().to_string()
}

fn match_command(command: Command, todo_list: &mut todo::TodoList) {
    match command {
        Command::List => todo::show_all_todo_list(&todo_list),
        Command::Add => {
            let due = input_prompt("Input due date for new Todo.(YYYY/MM/DD)");
            let title = input_prompt("Input title for new Todo.");
            let description = input_prompt("Input description for new Todo.");
            let new_todo_list = todo::add_new_todo(&due, &title, &description, todo_list);
            io::write_todo_list(&new_todo_list).expect("Failed to write new todos.");
        }
        Command::Complete => {
            let id = input_prompt("Input id of todo you have been completed.");
            for todo in todo_list.todos.iter_mut() {
                if todo.id.to_string() == id {
                    todo.completed();
                }
            }
            io::write_todo_list(todo_list).expect("Failed to write.");
        }
        Command::Patch => {
            let id = input_prompt("Input id of todo you edit.");
            let due = input_prompt("Input due date.(YYYY/MM/DD)");
            let title = input_prompt("Input title.");
            let description = input_prompt("Input description.");
            for todo in todo_list.todos.iter_mut() {
                if todo.id.to_string() == id {
                    todo.due = due.to_owned();
                    todo.title = title.to_owned();
                    todo.description = description.to_owned();
                }
            }
            io::write_todo_list(todo_list).expect("Failed to write.");
        }
        Command::Delete => {
            let id = input_prompt("Input id of todo you delete.");
            let id = id.parse().unwrap();
            let removed_todo_list = todo::remove_todo(id, todo_list);
            io::write_todo_list(&removed_todo_list).expect("Failed to remove.");
        }
        _ => print_usage(),
    }
}

fn parse_args(args: &Vec<String>) -> Input {
    if &args[1] == "list" {
        return Input {
            command: Command::List,
        };
    } else if &args[1] == "add" {
        return Input {
            command: Command::Add,
        };
    } else if &args[1] == "complete" {
        return Input {
            command: Command::Complete,
        };
    } else if &args[1] == "patch" {
        return Input {
            command: Command::Patch,
        };
    } else if &args[1] == "delete" {
        return Input {
            command: Command::Delete,
        };
    } else {
        return Input {
            command: Command::Noop,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut todo_list = io::read_todo_list();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let input: Input = parse_args(&args);
    match_command(input.command, &mut todo_list);
}
