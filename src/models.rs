use serde::{Deserialize, Serialize};
use std::fmt;
#[derive(Serialize, Deserialize)]
pub struct StatusType {
    pub name: String,
    pub order: u8,
}
#[derive(Serialize, Deserialize)]
pub struct Status(Vec<StatusType>);
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub order: u32,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
    pub project_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TaskList {
    pub tasks: Vec<Task>,
    pub last_id: u32,
    pub last_order: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ToDoErrors {
    NotFound,
    DatabaseError,
    SameProjectError,
    SameTaskError,
    InvalidIdError,
    InvalidTaskError,
    NotFoundTaskError,
}

impl fmt::Display for ToDoErrors {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error occured!")
    }
}
