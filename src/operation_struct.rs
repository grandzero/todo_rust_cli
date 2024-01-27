use crate::models::FindTaskBy;
use crate::models::{Task, TaskList, ToDoErrors};
use crate::Settings;
// use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use std::path::Path;
use std::result::Result;
extern crate chrono;
use chrono::Local;
#[derive(Clone)]
pub struct OperationStruct {
    pub filename: String,
    pub project_name: String,
}

impl OperationStruct {
    pub fn change_names(&mut self, name: String) -> Result<(), ToDoErrors> {
        self.filename = format!("{}.json", name);
        self.project_name = name;
        let settings = Settings {
            default_project: self.project_name.clone(),
            default_file: self.filename.clone(),
        };
        settings
            .set_settings("settings.json")
            .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
        Ok(())
    }

    pub fn init(name: &str) -> Result<OperationStruct, ToDoErrors> {
        if Path::new("settings.json").exists() {
            let settings =
                Settings::read_settings("settings.json").map_err(|_| ToDoErrors::DatabaseError)?;

            return Ok(OperationStruct {
                filename: settings.default_file,
                project_name: settings.default_project,
            });
        }
        let settings = Settings {
            default_project: name.to_string(),
            default_file: format!("{}.json", name),
        };
        if settings.set_settings("settings.json").is_ok() {
            return Ok(OperationStruct {
                filename: settings.default_file,
                project_name: settings.default_project,
            });
        }
        // set_settings updates file if exist, creates if not
        else {
            return Err(ToDoErrors::DatabaseError);
        }
    }

    pub fn print_task(&self, task: &Task) {
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

    fn write_to_file(&self, tasklist: &TaskList) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(tasklist)?;
        let mut file = File::create(&self.filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }

    fn read_from_file(&self) -> Result<TaskList, Box<dyn Error>> {
        let mut file = File::open(&self.filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let tasks: TaskList = serde_json::from_str(&contents)?;
        Ok(tasks)
    }

    pub fn write_tasks(&self, tasklist: &TaskList) -> Result<(), ToDoErrors> {
        self.write_to_file(tasklist)
            .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
        Ok(())
    }

    pub fn read_tasks(&self) -> Result<TaskList, ToDoErrors> {
        if let Ok(tasklist) = self.read_from_file() {
            Ok(tasklist)
        } else {
            Err(ToDoErrors::NotFound)
        }
    }

    pub fn find_task_by_id(&self, id: u32) -> Result<Task, ToDoErrors> {
        let task_list = self.read_tasks()?;
        if let Some(task) = task_list.tasks.iter().find(|task| task.id == id).cloned() {
            self.print_task(&task);
            return Ok(task);
        }

        Err(ToDoErrors::NotFound)
    }

    pub fn find_task(&self, contains: FindTaskBy) -> Result<Vec<Task>, ToDoErrors> {
        let tasklist = self.read_tasks()?;
        let found_tasks: Vec<Task> = tasklist
            .tasks
            .into_iter()
            .filter(|task| match &contains {
                FindTaskBy::Order(order) => task.order == *order,
                FindTaskBy::Status(status) => {
                    task.status.to_lowercase() == status.to_lowercase()
                        || task
                            .status
                            .to_lowercase()
                            .contains(status.to_lowercase().as_str())
                }
                FindTaskBy::Name(name) => {
                    task.name.to_lowercase() == name.to_lowercase()
                        || task
                            .name
                            .to_lowercase()
                            .contains(name.to_lowercase().as_str())
                }
                FindTaskBy::Contains(contains) => {
                    task.name.to_lowercase() == contains.to_lowercase()
                        || task
                            .name
                            .to_lowercase()
                            .contains(contains.to_lowercase().as_str())
                        || task.description.to_lowercase() == contains.to_lowercase()
                        || task
                            .description
                            .to_lowercase()
                            .contains(contains.to_lowercase().as_str())
                }
            })
            .collect();

        if found_tasks.is_empty() {
            Err(ToDoErrors::NotFound)
        } else {
            for task in &found_tasks {
                self.print_task(task);
            }
            Ok(found_tasks)
        }
    }

    pub fn remove_task(&self, id: u32) -> Result<(), ToDoErrors> {
        self.read_from_file()
            .and_then(|mut new_tasklist| {
                let index = new_tasklist.tasks.iter().position(|x| x.id == id).unwrap();
                new_tasklist.tasks.remove(index);
                self.write_to_file(&new_tasklist)
                    .or_else(|_| return Err(ToDoErrors::NotFoundTaskError))?;
                Ok(())
            })
            .or_else(|_| Err(ToDoErrors::NotFound))
    }

    pub fn update_task(
        &self,
        id: u32,
        new_name: Option<String>,
        new_description: Option<String>,
        new_completed: Option<bool>,
        new_order: Option<u32>,
        new_status: Option<String>,
    ) -> Result<Task, ToDoErrors> {
        self.read_from_file()
            .and_then(|mut new_tasklist| {
                let task = self.find_task_by_id(id)?;
                let now = Local::now();
                let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
                let new_name = new_name.unwrap_or_else(|| task.name.clone());
                let new_description = new_description.unwrap_or_else(|| task.description.clone());
                let new_order = new_order.unwrap_or_else(|| task.order.clone());
                let new_completed = new_completed.unwrap_or_else(|| task.completed.clone());
                let new_status = new_status.unwrap_or_else(|| task.status.clone());
                let new_task = Task {
                    id: task.id,
                    name: new_name,
                    status: new_status,
                    description: new_description,
                    completed: new_completed,
                    order: new_order,
                    created_at: task.created_at,
                    updated_at: formatted_time,
                    deleted_at: task.deleted_at,
                    project_name: task.project_name,
                };
                let index = new_tasklist.tasks.iter().position(|x| x.id == id).unwrap();
                new_tasklist.tasks.remove(index);
                new_tasklist.tasks.push(new_task.clone());
                self.print_task(&new_task);
                self.write_tasks(&new_tasklist)
                    .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
                Ok(new_task)
            })
            .or_else(|_| Err(ToDoErrors::NotFound))
    }

    pub fn create_new_task(
        &self,
        name: String,
        description: String,
        order: Option<u32>,
    ) -> Result<(), ToDoErrors> {
        let now = Local::now();
        let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
        let mut task = Task {
            id: 1,
            name: name,
            description: description,
            completed: false,
            order: 1,
            status: String::from("Created"),
            created_at: formatted_time.clone(),
            updated_at: formatted_time,
            deleted_at: String::from(""),
            project_name: self.project_name.clone(),
        };
        self.read_tasks()
            .and_then(|mut new_tasklist| {
                if let Some(order_value) = order {
                    if new_tasklist
                        .tasks
                        .iter()
                        .any(|task| task.order == order_value)
                    {
                        return Err(ToDoErrors::SameTaskError);
                    }
                }
                let id = new_tasklist.last_id + 1;
                let order = order.unwrap_or_else(|| new_tasklist.last_order + 1);
                task.order = order;
                task.id = id as u32;
                new_tasklist.last_id = id;
                new_tasklist.last_order = new_tasklist.last_order + 1;
                new_tasklist.tasks.push(task.clone());
                println!(
                    "Title: {} desciption: {} project: {} order: {}",
                    task.name, task.description, task.project_name, task.order
                );
                self.write_tasks(&new_tasklist)
                    .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
                self.print_task(&task);
                Ok(())
            })
            .or_else(|_| {
                let mut new_tasklist = TaskList {
                    tasks: Vec::new(),
                    last_id: 1,
                    last_order: 1,
                };
                new_tasklist.tasks.push(task.clone());
                self.print_task(&task);
                self.write_tasks(&new_tasklist)
                    .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
                Ok(())
            })
    }
}
