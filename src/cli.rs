use clap::{Parser, Subcommand};
use crate::models::Todo;
use crate::storage::{Storage, JsonStorage};

#[derive(Parser)]
#[command(name = "Todo CLI", version, about = "A simple todo app")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about="Add a new todo")]
    Add { 
        #[arg(help = "The title of the todo item")]
        title: String,
        #[arg(help = "The deadline of the todo item")]
        deadline: Option<String>,
    },

    #[command(about="List all todos")]
    List{
        #[arg(help="Show all todos")]
        all: bool,
    },

    #[command(about="Mark a todo as done")]
    Done { 
        #[arg(help="The ID of a item that is done")]
        id: usize 
    },
}

pub fn run() {
    let cli = Cli::parse();
    let storage = JsonStorage { path: "todos.json".into() };

    let mut todos = storage.load().unwrap_or_else(|_| vec![]);
    
    use Commands::*;
    match cli.command {
        Add { title, deadline } => {
            let todo = Todo {
                id: todos.len() + 1,
                title,
                done: false,
                deadline,
            };
            todos.push(todo);
            storage.save(&todos).unwrap();
            println!("✅ Todo added.");
        }
        List { all }    => {
            for todo in &todos {
                if all || !todo.done {
                    let status = if todo.done { "[V]" } else { "[X]" };
                    let deadline = todo.deadline.as_deref().unwrap_or("-");
                    println!("{} {} - {} (due: {})", status, todo.id, todo.title, deadline);
                }
            }
        
        }
        Done { id } => {
            if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                if todo.done {
                    println!("❌ Todo '{}' is already marked as done.", todo.title.clone());
                    return;
                }

                let title = todo.title.clone();
                todo.done = true;
                match storage.save(&todos) {
                    Ok(_) => println!("✅ Marked '{}' as done.", title),
                    Err(e) => println!("❌ Failed to save: {}", e)
                }
            } else {
                println!("❌ Todo with ID {} not found.", id);
            }
        }
    }
}

