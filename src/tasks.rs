use crate::models::{Task, TaskList, ToDoErrors};
// use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
extern crate chrono;
use chrono::Local;
// Write all tasks into tasklist file
fn write_to_file(filename: &str, tasklist: &TaskList) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string(tasklist)?;
    let mut file = File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}
// Read all tasks from tasklist file
fn read_from_file(filename: &str) -> Result<TaskList, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tasks: TaskList = serde_json::from_str(&contents)?;
    Ok(tasks)
}
// Check if there is a file [filename].json if not create and add task if there is, read tasks and append new task
pub fn create_new_task(
    name: String,
    description: String,
    project_name: String,
    order: Option<u32>,
    filename: &str,
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
        project_name: project_name,
    };
    println!("Filename is: {}", &filename);
    read_tasklist(filename)
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
            write_to_file(filename, &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
            Ok(())
        })
        .or_else(|_| {
            let mut new_tasklist = TaskList {
                tasks: Vec::new(),
                last_id: 1,
                last_order: 1,
            };
            new_tasklist.tasks.push(task.clone());
            write_to_file(filename, &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
            Ok(())
        })
}

pub fn read_tasklist(filename: &str) -> Result<TaskList, ToDoErrors> {
    if let Ok(tasklist) = read_from_file(filename) {
        Ok(tasklist)
    } else {
        Err(ToDoErrors::NotFound)
    }
}

pub fn update_task(
    id: u32,
    new_name: Option<String>,
    new_description: Option<String>,
    new_completed: Option<bool>,
    new_order: Option<u32>,
    new_status: Option<String>,
    filename: &str,
) -> Result<Task, ToDoErrors> {
    read_tasklist(filename)
        .and_then(|mut new_tasklist| {
            //let task = find_task(task_name.clone(), new_tasklist.clone())?;
            let task = find_task_by_id(id, new_tasklist.clone())?;
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
            write_to_file(filename, &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
            Ok(new_task)
        })
        .or_else(|_| Err(ToDoErrors::NotFound))
}

pub fn remove_task(id: u32, filename: &str) -> Result<(), ToDoErrors> {
    read_tasklist(filename)
        .and_then(|mut new_tasklist| {
            let index = new_tasklist.tasks.iter().position(|x| x.id == id).unwrap();
            new_tasklist.tasks.remove(index);
            write_to_file(filename, &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::NotFoundTaskError))?;
            Ok(())
        })
        .or_else(|_| Err(ToDoErrors::NotFound))
}

pub fn search_task(contains: String, task_list: TaskList) -> Result<Vec<Task>, ToDoErrors> {
    let tasks_with_order: Vec<Task> = task_list
        .tasks
        .into_iter()
        .filter(|task| {
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
        })
        .collect();

    if tasks_with_order.is_empty() {
        Err(ToDoErrors::NotFound)
    } else {
        Ok(tasks_with_order)
    }
}

pub fn find_task_by_name(name: String, task_list: TaskList) -> Result<Vec<Task>, ToDoErrors> {
    let tasks_with_order: Vec<Task> = task_list
        .tasks
        .into_iter()
        .filter(|task| {
            task.name.to_lowercase() == name.to_lowercase()
                || task
                    .name
                    .to_lowercase()
                    .contains(name.to_lowercase().as_str())
        })
        .collect();

    if tasks_with_order.is_empty() {
        Err(ToDoErrors::NotFound)
    } else {
        Ok(tasks_with_order)
    }
}

pub fn find_task(task_name: String, task_list: TaskList) -> Result<Task, ToDoErrors> {
    if let Some(task) = task_list
        .tasks
        .iter()
        .find(|task| task.name.to_lowercase().contains(&task_name.to_lowercase()))
        .cloned()
    {
        return Ok(task);
    }

    Err(ToDoErrors::NotFound)
}

pub fn find_task_by_order(order: u32, task_list: TaskList) -> Result<Vec<Task>, ToDoErrors> {
    let tasks_with_order: Vec<Task> = task_list
        .tasks
        .into_iter()
        .filter(|task| task.order == order)
        .collect();

    if tasks_with_order.is_empty() {
        Err(ToDoErrors::NotFound)
    } else {
        Ok(tasks_with_order)
    }
}

pub fn find_task_by_status(status: String, task_list: TaskList) -> Result<Vec<Task>, ToDoErrors> {
    let tasks_with_order: Vec<Task> = task_list
        .tasks
        .into_iter()
        .filter(|task| {
            task.status.to_lowercase() == status.to_lowercase()
                || task
                    .status
                    .to_lowercase()
                    .contains(status.to_lowercase().as_str())
        })
        .collect();

    if tasks_with_order.is_empty() {
        Err(ToDoErrors::NotFound)
    } else {
        Ok(tasks_with_order)
    }
}

pub fn find_task_by_id(id: u32, task_list: TaskList) -> Result<Task, ToDoErrors> {
    if let Some(task) = task_list.tasks.iter().find(|task| task.id == id).cloned() {
        return Ok(task);
    }

    Err(ToDoErrors::NotFound)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let task = create_new_task(
            String::from("Test task"),
            String::from("Test task description"),
            String::from("Test project"),
            Some(1),
            "tasks.json",
        );
        assert_eq!(task.is_ok(), true);
    }

    #[test]
    fn test_read_tasklist() {
        let tasklist = read_tasklist("tasks.json");
        assert_eq!(tasklist.is_ok(), true);
    }

    // TODO add tests
}
