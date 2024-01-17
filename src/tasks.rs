// use crate::models::{Status, StatusType, Task, TaskList, ToDoErrors};
use crate::models::{Task, TaskList, ToDoErrors};
// use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
extern crate chrono;
use chrono::Local;

fn write_to_file(filename: &str, tasklist: &TaskList) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string(tasklist)?;
    let mut file = File::create(filename)?;
    file.write_all(serialized.as_bytes())?;
    Ok(())
}

fn read_from_file(filename: &str) -> Result<TaskList, Box<dyn Error>> {
    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let my_struct: TaskList = serde_json::from_str(&contents)?;
    Ok(my_struct)
}

pub fn create_new_task(
    name: String,
    description: String,
    project_name: String,
    order: u32,
) -> Result<Task, ToDoErrors> {
    let now = Local::now();
    let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
    let task = Task {
        id: 1,
        name: name,
        description: description,
        completed: false,
        order: order,
        created_at: formatted_time.clone(),
        updated_at: formatted_time,
        deleted_at: String::from(""),
        project_name: project_name,
    };
    read_tasklist("tasks.json")
        .and_then(|mut new_tasklist| {
            new_tasklist.tasks.push(task.clone());
            write_to_file("tasks.json", &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
            Ok(task.clone())
        })
        .or_else(|_| {
            let mut new_tasklist = TaskList {
                tasks: Vec::new(),
                last_id: 0,
                last_order: 0,
            };
            new_tasklist.tasks.push(task.clone());
            write_to_file("tasks.json", &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
            Ok(task)
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
    task_name: String,
    new_name: String,
    new_description: String,
    new_project_name: String,
    new_order: u32,
) -> Result<Task, ToDoErrors> {
    read_tasklist("tasks.json")
        .and_then(|mut new_tasklist| {
            let task = find_task(task_name.clone(), new_tasklist.clone())?;
            let now = Local::now();
            let formatted_time = now.format("%Y-%m-%d %H:%M:%S").to_string();
            let new_task = Task {
                id: task.id,
                name: new_name,
                description: new_description,
                completed: task.completed,
                order: new_order,
                created_at: task.created_at,
                updated_at: formatted_time,
                deleted_at: task.deleted_at,
                project_name: new_project_name,
            };
            let index = new_tasklist
                .tasks
                .iter()
                .position(|x| x.name == task_name)
                .unwrap();
            new_tasklist.tasks.remove(index);
            new_tasklist.tasks.push(new_task.clone());
            write_to_file("tasks.json", &new_tasklist)
                .or_else(|_| return Err(ToDoErrors::DatabaseError))?;
            Ok(new_task)
        })
        .or_else(|_| Err(ToDoErrors::NotFound))
}

pub fn find_task(task_name: String, task_list: TaskList) -> Result<Task, ToDoErrors> {
    println!("{:?}", task_list);
    println!("Entered find task");
    if let Some(task) = task_list
        .tasks
        .iter()
        .find(|task| task.name.contains(&task_name))
        .cloned()
    {
        println!("Found task");
        return Ok(task);
    }
    println!("Task not found");
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
            1,
        );
        assert_eq!(task.is_ok(), true);
    }

    #[test]
    fn test_read_tasklist() {
        let tasklist = read_tasklist("tasks.json");
        assert_eq!(tasklist.is_ok(), true);
    }

    #[test]
    fn test_update_task() {
        let task = create_new_task(
            String::from("Test task"),
            String::from("Test task description"),
            String::from("Test project"),
            1,
        );
        let task = update_task(
            String::from("Test task"),
            String::from("Changed"),
            String::from("Changed"),
            String::from("Changed"),
            1,
        );
        let tasklist = read_tasklist("tasks.json");
        println!("{:?}", tasklist);
        assert_eq!(task.is_ok(), true);
    }

    // TODO add tests
}
