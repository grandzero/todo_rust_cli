use clap::Parser;

fn main() -> Result<(), todo_cli::ToDoErrors> {
    let args = todo_cli::cli::Args::parse();
    return todo_cli::task_operations_func(args.command);
}
