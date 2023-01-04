use std::env;
mod todo;

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

fn parseInput(args: &Vec<String>) {
    let options: Vec<&str> = args[1].matches("--").collect();
    if !options.is_empty() {
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let todo_list = todo::read_todo_list();
    if args.len() < 2 {
        todo::show_all_todo_list(&todo_list);
        return;
    }

    // match_option(option);

}
