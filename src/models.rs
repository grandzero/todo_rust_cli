use std::{error::Error, fmt};
pub struct Project {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
    pub is_active: bool,
    pub tasks: Vec<Task>,
}

pub struct StatusType {
    pub name: String,
    pub order: u8,
}

pub struct Status(Vec<StatusType>);

pub struct Task {
    pub id: u8,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub order: u8,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
    pub project_id: u8,
}

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
