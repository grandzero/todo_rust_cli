use clap::Parser;
use models::ToDoErrors;
use std::env;
mod settings;
// use std::fs::{self, File};
use std::io::{Error, Write};
// use std::path::PathBuf;
mod cli;

mod models;
mod tasks;
use cli::{Args, Commands};
mod helpers;
use crate::settings::Settings;

fn main() -> Result<(), ToDoErrors> {
    let args = cli::Args::parse();
    match args.command {
        Commands::CreateProject { name } => {
            if let Ok(settings) = Settings::read_settings("settings.json") {
                if settings.default_project == name {
                    println!("Project already exists");
                    std::process::exit(0);
                }
            } else {
                helpers::create_update_settings(&name)?;
            }
            let file_name = format!("{}.json", name);
            println!("Creating project '{}'", file_name);
            helpers::create_empty_file(&file_name)?;
            println!("Project '{}' created", file_name);
        }
        Commands::SetProject { name } => {
            let file_name = format!("{}.json", name);
            if helpers::file_exists(&file_name) {
                println!("Setting project '{}' as default", file_name);
                helpers::create_update_settings(&name)?;
            } else {
                println!("Project not found. Do you want to create a new project file (Y/n)?");
                let mut input = String::new();
                std::io::stdout().flush().unwrap();
                std::io::stdin().read_line(&mut input).unwrap();

                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => {
                        println!("Creating and setting up new project...");
                        helpers::create_empty_file(&file_name)?;
                        helpers::create_update_settings(&name)?;
                        println!("Project '{}' created and set as default", file_name);
                    }
                    _ => {
                        println!("Exiting without creating project.");
                        return Err(ToDoErrors::DatabaseError);
                    }
                }
            }
        }
        Commands::CreateTask {
            title,
            description,
            order,
        } => {
            println!(
                "Creating task '{}' with description '{}'",
                title, description
            );
            let settings = helpers::get_settings()?;
            println!(
                "Creating task with default project '{}'",
                settings.default_file
            );
            tasks::create_new_task(
                title.to_string(),
                description.to_string(),
                settings.default_project.to_string(),
                order,
                &settings.default_file,
            )?;
            println!("Task '{}' created", title);
        }
        Commands::GetTasklist => {
            let filename = helpers::get_settings()?;
            let tasklist = tasks::read_tasklist(&filename.default_file)?;
            for task in tasklist.tasks.iter() {
                println!("-----------------------------------");
                println!(
                    "Id: {},\ntitle: {},\ndescription: {},\nproject: {},\norder: {},\nstatus: {},\ncompleted: {},\ncreated_at: {},\nupdated_at: {}",
                    task.id,
                    task.name,
                    task.description,
                    task.project_name,
                    task.order,
                    task.status,
                    task.completed,
                    task.created_at,
                    task.updated_at
                );
                println!("-----------------------------------");
            }
        }
        Commands::GetTaskByOrder { order } => {
            let filename = helpers::get_settings()?;
            let tasklist = tasks::read_tasklist(&filename.default_file)?;
            for task in tasks::find_task_by_order(order, tasklist.clone())? {
                println!("-----------------------------------");
                println!(
                    "Id: {},\ntitle: {},\ndescription: {},\nproject: {},\norder: {},\nstatus: {},\ncompleted: {},\ncreated_at: {},\nupdated_at: {}",
                    task.id,
                    task.name,
                    task.description,
                    task.project_name,
                    task.order,
                    task.status,
                    task.completed,
                    task.created_at,
                    task.updated_at
                );
                println!("-----------------------------------");
            }
        }
        Commands::GetTaskByStatus { status } => {
            let filename = helpers::get_settings()?;
            let tasklist = tasks::read_tasklist(&filename.default_file)?;
            for task in tasks::find_task_by_status(status, tasklist.clone())? {
                println!("-----------------------------------");
                println!(
                    "Id: {},\ntitle: {},\ndescription: {},\nproject: {},\norder: {},\nstatus: {},\ncompleted: {},\ncreated_at: {},\nupdated_at: {}",
                    task.id,
                    task.name,
                    task.description,
                    task.project_name,
                    task.order,
                    task.status,
                    task.completed,
                    task.created_at,
                    task.updated_at
                );
                println!("-----------------------------------");
            }
        }
        Commands::GetTaskById { id } => {
            let filename = helpers::get_settings()?;
            let tasklist = tasks::read_tasklist(&filename.default_file)?;
            let task = tasks::find_task_by_id(id, tasklist.clone())?;
            println!("-----------------------------------");
            println!(
                "Id: {},\ntitle: {},\ndescription: {},\nproject: {},\norder: {},\nstatus: {},\ncompleted: {},\ncreated_at: {},\nupdated_at: {}",
                task.id,
                task.name,
                task.description,
                task.project_name,
                task.order,
                task.status,
                task.completed,
                task.created_at,
                task.updated_at
            );
            println!("-----------------------------------");
        }
        Commands::GetTask { name } => {
            let filename = helpers::get_settings()?;
            let tasklist = tasks::read_tasklist(&filename.default_file)?;
            for task in tasks::find_task_by_name(name, tasklist.clone())? {
                println!("-----------------------------------");
                println!(
                    "Id: {},\ntitle: {},\ndescription: {},\nproject: {},\norder: {},\nstatus: {},\ncompleted: {},\ncreated_at: {},\nupdated_at: {}",
                    task.id,
                    task.name,
                    task.description,
                    task.project_name,
                    task.order,
                    task.status,
                    task.completed,
                    task.created_at,
                    task.updated_at
                );
                println!("-----------------------------------");
            }
        }
        Commands::FindTask { contains } => {
            let filename = helpers::get_settings()?;
            let tasklist = tasks::read_tasklist(&filename.default_file)?;
            for task in tasks::search_task(contains, tasklist.clone())? {
                println!("-----------------------------------");
                println!(
                    "Id: {},\ntitle: {},\ndescription: {},\nproject: {},\norder: {},\nstatus: {},\ncompleted: {},\ncreated_at: {},\nupdated_at: {}",
                    task.id,
                    task.name,
                    task.description,
                    task.project_name,
                    task.order,
                    task.status,
                    task.completed,
                    task.created_at,
                    task.updated_at
                );
                println!("-----------------------------------");
            }
        }

        Commands::UpdateTask {
            id,
            title,
            description,
            completed,
            order,
            status,
        } => {
            let settings = helpers::get_settings()?;
            tasks::update_task(
                id,
                title,
                description,
                completed,
                order,
                status,
                &settings.default_file,
            )?;
        }
        Commands::UpdateTaskStatus { id, status } => {
            let settings = helpers::get_settings()?;
            tasks::update_task(
                id,
                None,
                None,
                None,
                None,
                Some(status),
                &settings.default_file,
            )?;
        }
        Commands::RemoveTask { id } => {
            let settings = helpers::get_settings()?;
            tasks::remove_task(id, &settings.default_file)?;
        }
        Commands::Complete { id } => {
            let settings = helpers::get_settings()?;
            tasks::update_task(
                id,
                None,
                None,
                Some(true),
                None,
                None,
                &settings.default_file,
            )?;
        }
        _ => {
            // Implement logic to handle other subcommands
            println!("Other subcommands");
        }
    }
    Ok(())
}
