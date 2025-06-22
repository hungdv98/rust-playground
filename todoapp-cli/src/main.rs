use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io::{BufReader, BufWriter};
use std::path::Path;

const FILE_PATH: &str = "task.json";

#[derive(Serialize, Deserialize, Debug)]
struct Task {
    text: String,
    done: bool,
}

#[derive(Parser)]
#[command(name = "ToDoApp CLI")]
#[command(about = "Simple ToDoApp", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { task: String },
    List,
    Done { index: usize },
    Delete { index: usize },
}

fn load_tasks() -> Vec<Task> {
    if !Path::new(FILE_PATH).exists() {
        return Vec::new();
    }

    let file = File::open(FILE_PATH).expect("Failed to open file");
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).unwrap_or_else(|_| Vec::new())
}

fn save_tasks(tasks: &Vec<Task>) {
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(FILE_PATH)
        .expect("Failed to open file");

    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, tasks).expect("Failed to write to file");
}

fn main() {
    let cli = Cli::parse();
    let mut tasks = load_tasks();

    match &cli.command {
        Commands::Add { task } => {
            tasks.push(Task {
                text: task.to_string(),
                done: false,
            });
            save_tasks(&tasks);
            println!("Task added.");
        }
        Commands::List => {
            for (i, task) in tasks.iter().enumerate() {
                let status = if task.done { "[x]" } else { "[ ]" };
                println!("{} {} - {}", i, status, task.text);
            }
        }
        Commands::Done { index } => {
            if let Some(task) = tasks.get_mut(*index) {
                task.done = true;
                save_tasks(&tasks);
                println!("Task marked as done.");
            } else {
                println!("Task not found.");
            }
        }
        Commands::Delete { index } => {
            if *index < tasks.len() {
                tasks.remove(*index);
                save_tasks(&tasks);
                println!("Task not found.");
            }
        }
    }
}
