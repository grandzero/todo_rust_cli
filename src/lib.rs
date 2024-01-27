use command_operations::CommandOperations;
pub use models::ToDoErrors;
pub mod cli;
pub mod models;
pub mod settings;
pub use cli::{Args, Commands};
pub use settings::Settings;
pub mod command_operations;
pub mod operation_struct;
pub use command_operations::OperationStruct;

pub fn task_operations_func(commands: cli::Commands) -> Result<(), ToDoErrors> {
    let mut op: OperationStruct = OperationStruct::init("default")?;
    match commands {
        Commands::CreateProject { name } => {
            op.create_project(name)?;
        }
        Commands::SetProject { name } => {
            println!("Creating project");
            op.set_project_as_default(name)?;
        }
        Commands::CreateTask {
            title,
            description,
            order,
        } => {
            op.create_new_task(title, description, order)?;
        }
        Commands::GetTasklist => {
            op.get_task_list()?;
        }
        Commands::GetTaskByOrder { order } => {
            op.find_task(models::FindTaskBy::Order(order))?;
        }
        Commands::GetTaskByStatus { status } => {
            op.find_task(models::FindTaskBy::Status(status))?;
        }
        Commands::GetTaskById { id } => {
            op.find_task_by_id(id)?;
        }
        Commands::GetTask { name } => {
            op.find_task(models::FindTaskBy::Name(name))?;
        }
        Commands::FindTask { contains } => {
            op.find_task(models::FindTaskBy::Contains(contains))?;
        }

        Commands::UpdateTask {
            id,
            title,
            description,
            completed,
            order,
            status,
        } => {
            op.update_task(id, title, description, completed, order, status)?;
        }
        Commands::UpdateTaskStatus { id, status } => {
            op.update_task_status(id, status)?;
        }
        Commands::RemoveTask { id } => {
            op.remove_task(id)?;
        }
        Commands::Complete { id } => {
            op.mark_task_complete(id)?;
        }
        Commands::Uncomplete { id } => {
            op.mark_task_incomplete(id)?;
        }
    }
    return Ok(());
}
