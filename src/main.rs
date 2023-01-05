use std::env;
mod io;
mod todo;

enum Command {
    List,
    Add,
    Complete,
    Noop,
}

struct Input {
    command: Command,
}

fn print_usage() {
    println!(
        "{}{}{}{}{}",
        "[USAGE] todo <command>\n",
        "<command>\n",
        "add - Add a new todo.\n",
        "list - List all todos.\n",
        "complete - Complete a todo.\n"
    );
}

fn input_prompt(msg: &str) -> String {
    println!("{}", msg);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Input error");
    input.trim().to_string()
}

fn match_command(command: Command, todo_list: &todo::TodoList) {
    match command {
        Command::List => todo::show_all_todo_list(&todo_list),
        Command::Add => {
            let due = input_prompt("Input due date for new Todo.(YYYY/MM/DD)");
            let title = input_prompt("Input title for new Todo.");
            let description = input_prompt("Input description for new Todo.");
            let new_todo_list = todo::add_new_todo(&due, &title, &description, todo_list);
        }
        Command::Complete => println!("do complete"),
        _ => print_usage(),
    }
}

// Todo:
fn _match_option(option: &str) {
    match option {
        "--help" => print_usage(),
        _ => print_usage(),
    }
}

fn parse_args(args: &Vec<String>) -> Input {
    // Todo: implement parse options
    // let reg = Regex::new(r"--.*").unwrap();
    // let options = reg.captures(&args[1]).unwrap();
    // let option = options.get(1).unwrap().as_str();
    // println!("opt: {}", option);
    // let options: Vec<&str> = args[1].matches("--").collect();
    // if !options.is_empty() {
    //     println!("opt: {}", options[0]);
    // }

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
    } else {
        return Input {
            command: Command::Noop,
        };
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let todo_list = io::read_todo_list();
    if args.len() < 2 {
        print_usage();
        return;
    }

    let input: Input = parse_args(&args);
    match_command(input.command, &todo_list);
}
