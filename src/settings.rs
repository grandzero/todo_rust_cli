use crate::models::ToDoErrors;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::io::Write;
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub default_project: String,
    pub default_file: String,
}

impl Settings {
    pub fn set_settings(&self, filename: &str) -> Result<(), ToDoErrors> {
        let serialized = serde_json::to_string(&self).map_err(|_| ToDoErrors::DatabaseError)?;
        let mut file = File::create(filename).map_err(|_| ToDoErrors::DatabaseError)?;
        file.write_all(serialized.as_bytes())
            .map_err(|_| ToDoErrors::DatabaseError)?;
        Ok(())
    }
    pub fn read_settings(filename: &str) -> Result<Settings, ToDoErrors> {
        let mut file = File::open(filename).map_err(|_| ToDoErrors::DatabaseError)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)
            .map_err(|_| ToDoErrors::DatabaseError)?;
        let settings: Settings =
            serde_json::from_str(&contents).map_err(|_| ToDoErrors::DatabaseError)?;
        Ok(settings)
    }
}
