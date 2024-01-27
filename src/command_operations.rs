use chrono::format;

use crate::models::{TaskList, ToDoErrors};
pub use crate::operation_struct::OperationStruct;
use crate::Settings;
use std::io::Write;
use std::path::Path;

use std::result::Result;
extern crate chrono;

pub trait CommandOperations {
    fn create_project(&mut self, name: String) -> Result<(), ToDoErrors>;
    fn set_project_as_default(&mut self, name: String) -> Result<(), ToDoErrors>;
    fn get_task_list(&self) -> Result<(), ToDoErrors>;
    fn update_task_status(&self, id: u32, status: String) -> Result<(), ToDoErrors>;
    fn mark_task_complete(&self, id: u32) -> Result<(), ToDoErrors>;
    fn mark_task_incomplete(&self, id: u32) -> Result<(), ToDoErrors>;
}

impl CommandOperations for OperationStruct {
    fn create_project(&mut self, name: String) -> Result<(), ToDoErrors> {
        // If there is no settings.json file, create it and set the project as default

        let file_name = format!("{}.json", name);
        if Path::new(&file_name).exists() {
            println!("Project '{}' already exists", name);
            return Err(ToDoErrors::SameProjectError);
        } else {
            let tasklist = TaskList {
                tasks: Vec::new(),
                last_id: 1,
                last_order: 1,
            };
            self.change_names(name)?;
            self.write_tasks(&tasklist)?;
        }

        Ok(())
    }

    fn set_project_as_default(&mut self, name: String) -> Result<(), ToDoErrors> {
        // Check if project exists
        // If not, ask if user wants to create project file
        // If yes, create project file
        // If there is, set as default
        let file_name = format!("{}.json", name);
        if !Path::new(&file_name).exists() {
            println!("Project not found. Do you want to create a new project file (Y/n)?");
            let mut input = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut input).unwrap();

            match input.trim().to_lowercase().as_str() {
                "y" | "yes" => {
                    println!("Creating and setting up new project...");
                    let tasklist = TaskList {
                        tasks: Vec::new(),
                        last_id: 1,
                        last_order: 1,
                    };
                    self.change_names(name)?;
                    self.write_tasks(&tasklist)?;
                    println!("Project '{}' created and set as default", file_name);
                    return Ok(());
                }
                _ => {
                    println!("Exiting without creating project.");
                    return Err(ToDoErrors::DatabaseError);
                }
            }
        }
        let settings = Settings {
            default_project: name.to_string(),
            default_file: format!("{}.json", name),
        };
        settings.set_settings("settings.json")?;

        Ok(())
    }

    fn get_task_list(&self) -> Result<(), ToDoErrors> {
        let tasklist = self.read_tasks()?;
        for task in tasklist.tasks {
            self.print_task(&task);
        }
        return Ok(());
    }

    fn update_task_status(&self, id: u32, status: String) -> Result<(), ToDoErrors> {
        self.update_task(id, None, None, None, None, Some(status))?;
        Ok(())
    }

    fn mark_task_complete(&self, id: u32) -> Result<(), ToDoErrors> {
        self.update_task(id, None, None, Some(true), None, None)?;
        Ok(())
    }

    fn mark_task_incomplete(&self, id: u32) -> Result<(), ToDoErrors> {
        self.update_task(id, None, None, Some(false), None, None)?;
        Ok(())
    }
}
