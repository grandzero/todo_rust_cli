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
    },
    GetTasklist,

    GetTaskById {
        #[arg(long)]
        id: u32,
    },
    GetTask {
        #[arg(long)]
        name: String,
    },
    FindTask {
        #[arg(long)]
        contains: String,
    },
    UpdateTaskById {
        #[arg(long)]
        id: u32,
        #[arg(long)]
        title: Option<String>,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        completed: Option<bool>,
    },
    UpdateTask {
        #[arg(long)]
        title: String,
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        completed: Option<bool>,
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
