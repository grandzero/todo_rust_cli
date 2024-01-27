use std::path::Path;
use todo_cli::command_operations::CommandOperations;
use todo_cli::command_operations::OperationStruct;
// Test scenarios
// Create project
#[test]
fn test_create_project() {
    use std::path::Path;
    let mut op = OperationStruct::init("todocli").unwrap();
    assert_eq!(op.create_project("todocli2".to_string()).is_ok(), false);
    assert_eq!(Path::new("todocli2.json").exists(), true);
}
// Set project as default
#[test]
fn test_set_project_as_default() {
    let mut op = OperationStruct::init("todocli").unwrap();
    assert_eq!(
        op.set_project_as_default("todocli".to_string()).is_ok(),
        true
    );
    assert_eq!(
        op.set_project_as_default("todocli".to_string()).is_ok(),
        true
    );
}
// Create task
#[test]
fn test_create_task() {
    let op = OperationStruct::init("todocli").unwrap();
    assert_eq!(
        op.create_new_task(
            "Test task7".to_string(),
            "Test description7".to_string(),
            Some(1)
        )
        .is_ok(),
        true
    );
}

// Update task
#[test]
fn test_update_task_title() {
    let op = OperationStruct::init("todocli").unwrap();
    assert_eq!(
        op.update_task(2, Some("UpdatedNext".to_string()), None, None, None, None)
            .is_ok(),
        true
    );

    // assert_eq!(tasks.tasks[0].name, "Updated".to_string());
}
// Update task status
#[test]
fn test_update_task_status() {
    let op = OperationStruct::init("todocli").unwrap();
    assert_eq!(
        op.update_task_status(2, "In Progress".to_string()).is_ok(),
        true
    );

    // assert_eq!(
    //     op.find_task_by_id(2).unwrap().status,
    //     "In Progress".to_string()
    // );
}
// Mark task complete
#[test]
fn test_mark_task_complete() {
    let op = OperationStruct::init("todocli").unwrap();
    assert_eq!(op.mark_task_complete(2).is_ok(), true);

    // assert_eq!(tasks.tasks[0].completed, true);
}
// Mark task incomplete
#[test]
fn test_mark_task_incomplete() {
    let op = OperationStruct::init("todocli").unwrap();
    assert_eq!(op.mark_task_incomplete(2).is_ok(), true);

    // assert_eq!(tasks.tasks[0].completed, false);
}
// Remove task
#[test]
fn test_remove_task() {
    let op = OperationStruct::init("todocli").unwrap();
    assert_eq!(op.remove_task(2).is_ok(), true);

    // assert_eq!(tasks.tasks.len(), 0);
}
