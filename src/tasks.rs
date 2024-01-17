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

pub fn read_tasklist(project_name: &str) -> Result<TaskList, ToDoErrors> {
    if let Ok(project) = read_from_file(project_name) {
        Ok(project)
    } else {
        Err(ToDoErrors::NotFound)
    }
}

pub fn find_task(task_name: String, task_list: TaskList) -> Result<Task, ToDoErrors> {
    task_list
        .tasks
        .iter()
        .find(|task| task.name == task_name)
        .cloned()
        .ok_or(ToDoErrors::NotFoundTaskError)
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

    // TODO add tests
}
