use clap::{Command, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, about, version, name = "todo")]
pub struct Args {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Creates a new project
    CreateProject {
        #[arg(long, short = 'n')]
        name: String,
    },
    /// Sets a default project
    SetProject {
        #[arg(long)]
        name: String,
    },
    /// Creates a new task
    CreateTask {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: String,
        #[arg(long)]
        order: Option<u32>,
    },
    GetTasklist,
    GetTaskById {
        #[arg(long)]
        id: u32,
    },
    GetTaskByOrder {
        #[arg(long)]
        order: u32,
    },
    GetTaskByStatus {
        #[arg(long)]
        status: String,
    },
    GetTask {
        #[arg(long)]
        name: String,
    },
    FindTask {
        #[arg(long)]
        contains: String,
    },
    UpdateTask {
        #[arg(long)]
        id: u32,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        completed: Option<bool>,
        #[arg(long)]
        order: Option<u32>,
        #[arg(long)]
        status: Option<String>,
    },

    UpdateTaskStatus {
        #[arg(long)]
        id: u32,
        #[arg(long)]
        status: String,
    },

    RemoveTask {
        #[arg(long)]
        id: u32,
    },

    Complete {
        #[arg(long)]
        id: u32,
    },
}
