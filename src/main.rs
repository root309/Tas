use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("使い方: cargo run command args");
        return;
    }

    let command = &args[1];

    match command.as_str() {
        "add" => add_task(&args),
        "list" => list_tasks(),
        "remove" => remove_task(&args),
        _ => println!("Unknown command: {}", command),
    }
}

fn add_task(args: &Vec<String>) {
    if args.len() < 3 {
        println!("Usage: todo_cli add [task]");
        return;
    }

    let task = &args[2];
    fs::write("tasks.txt", format!("{}\n", task)).expect("書き込み失敗");
    println!("Task added: {}", task);
}

fn list_tasks() {
    if let Ok(contents) = fs::read_to_string("tasks.txt") {
        if contents.trim().is_empty() {
            println!("タスクないよー");
        } else {
            println!("Tasks:\n{}", contents);
        }
    } else {
        println!("タスクないよー");
    }
}

fn remove_task(args: &Vec<String>) {
    if args.len() < 3 {
        println!("使い方: cargo run remove 0");
        return;
    }

    let index = args[2].parse::<usize>().expect("null index");
    let tasks = fs::read_to_string("tasks.txt").expect("ファイルの読み込みに失敗");
    let task_lines: Vec<&str> = tasks.lines().collect();

    if index >= task_lines.len() {
        println!("インデックスの入力方法が間違っています。");
    } else {
        let removed_task = task_lines[index];
        let updated_tasks = task_lines
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .map(|(_, t)| *t)
            .collect::<Vec<&str>>()
            .join("\n");
        fs::write("tasks.txt", updated_tasks).expect("書き込み失敗");
        println!("Removed task: {}", removed_task);
    }
}
