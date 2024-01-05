pub struct Project {
    pub id: i32,
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
    pub order: i32,
}

pub struct Status(Vec<StatusType>);

pub struct Task {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub completed: bool,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: String,
    pub project_id: i32,
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
