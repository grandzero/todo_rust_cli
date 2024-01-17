use crate::models::{Project, Status, StatusType, Task, ToDoErrors};

pub fn create_project(name: String, description: String) -> Result<Project, ToDoErrors> {
    let project = Project {
        id: 1,
        name: name,
        description: description,
        created_at: String::from(""),
        updated_at: String::from(""),
        deleted_at: String::from(""),
        is_active: true,
        tasks: Vec::new(),
    };
    Ok(project)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_project() {}

    // TODO add tests
}
