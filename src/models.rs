use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fmt;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub order: u32,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
    pub project_name: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    NotFoundSettingsError,
}
impl std::fmt::Display for ToDoErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ToDoErrors {}
