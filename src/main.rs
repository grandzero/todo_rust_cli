use clap::Parser;
use std::env;
// use std::fs::{self, File};
// use std::io::{Error, Write};
// use std::path::PathBuf;

mod cli;
mod models;
mod tasks;
use cli::{Args, Commands};
fn main() {
    let args = cli::Args::parse();
    match args.command {
        Commands::CreateProject { name } => {
            // Implement logic to create a project
            println!("Creating project '{}'", name);
        }
        Commands::SetProject { name } => {
            // Implement logic to set a project
            println!("Setting project '{}'", name);
        }
        Commands::CreateTask { title, description } => {
            // Implement logic to create a task
            println!(
                "Creating task '{}' with description '{}'",
                title, description
            );
        }
        Commands::GetTasklist => {
            // Implement logic to create a task
            println!("Get task list worked",);
        }
        Commands::UpdateTaskById {
            id,
            title,
            description,
            completed,
        } => {
            // Implement logic to update a task
            println!(
                "Updating task with id '{}' title: '{:?}' with description '{:?}' completed: '{:?}'",
                id, title, description, completed
            );
        }
        _ => {
            // Implement logic to handle other subcommands
            println!("Other subcommands");
        }
    }
}
