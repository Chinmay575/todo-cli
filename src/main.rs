use clap::Parser;
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::{self};

#[derive(Parser, Debug)]
struct Cli {
    argument: String,
    data: Vec<String>,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
struct Task {
    name: String,
    id: u8,
    is_done: bool,
}

#[derive(Deserialize, Debug, Serialize, PartialEq, Clone)]
struct FileContents {
    data: Vec<Task>,
}

const F: &str = "src/data.json";

fn main() {
    // let args: Vec<String> = env::args().collect();

    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
    })
    .expect("Error setting Ctrl-C handler");

    let args = Cli::parse();

    let contents = fs::read_to_string(F).expect("Failed to read file");
    let mut file_contents: FileContents;
    if !contents.is_empty() {
        file_contents = serde_json::from_str(&contents).expect("JSON Formatting is not correct");

        let modfied_data = match args.argument.as_str() {
            "add" => add_task(args.data, file_contents.data),
            "display" => print_all_tasks(file_contents.data),
            "remove" => remove_task(args.data, file_contents.data),
            "markdone" => mark_done(args.data, file_contents.data), 
            _ => file_contents.data,
        };

        file_contents.data = modfied_data;

        let str = serde_json::to_string(&file_contents).expect("Failed to serialize file contents");
        fs::write(F, str).expect("Failed to write in file");
    }
}

fn add_task(data: Vec<String>, mut tasks: Vec<Task>) -> Vec<Task> {
    // let _ = data;

    if data.is_empty() {
        println!("{}", "You need to provide data to add a task".red());
        return tasks;
    }

    for i in data {
        let x: Vec<Task> = tasks
            .clone()
            .into_iter()
            .filter(|task| task.name == *i.to_lowercase())
            .collect();

        let len = match u8::try_from(tasks.len()) {
            Ok(num) => num,
            Err(_) => 255,
        };
        if x.is_empty() {
            tasks.push(Task {
                id: len + 1,
                name: i.to_string().to_lowercase(),
                is_done: false,
            });
            println!(
                "{0} {1} {2}",
                "Task".green(),
                i.green(),
                "added successfully".green()
            );
        } else {
            println!("{0} {1} {2}", "Task".red(), i.red(), "already exists".red());
            // println!("Task {} already exists",i);
            continue;
        }
    }

    return tasks;
}

fn print_all_tasks(tasks: Vec<Task>) -> Vec<Task> {
    if !tasks.is_empty() {
        for (index, data) in tasks.iter().enumerate() {
            println!(
                "{0}) {1}:{2}",
                index.to_string().magenta(),
                data.name.yellow(),
                data.is_done.to_string().green()
            );
        }
    } else {
        println!("{}", "You don't have any tasks right now !!".blue());
        println!(
            "{}",
            "You can add task by using add and specifying data as shown in example below:".blue()
        );
        println!("{}", "todo add task_name".yellow());
    }

    // for (index,d)
    return tasks;
}

fn remove_task(data: Vec<String>, mut tasks: Vec<Task>) -> Vec<Task> {
    if data.is_empty() {
        println!("{}", "You need to provide data to remove a task".red());
        return tasks;
    }

    for i in data {
        for (index, j) in tasks.clone().iter().enumerate() {
            if j.name.to_lowercase() == i.to_lowercase() {
                tasks.remove(index);
            }
        }
    }

    return tasks;
}

fn mark_done(data: Vec<String>, mut tasks: Vec<Task>) -> Vec<Task> {
    if data.is_empty() {
        println!("{}", "You need to provide data to remove a task".red());
        return tasks;
    }

    for i in data {
        for (index, j) in tasks.clone().iter().enumerate() {
            if j.name.to_lowercase() == i.to_lowercase() {
                tasks[index].is_done = true;
            }
        }
    }

    return tasks;
}
