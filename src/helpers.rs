use crate::models::ToDoErrors;
use crate::settings::Settings;
use std::process::Command;
use std::{path::Path, result::Result};
use winreg::{enums::*, RegKey};
pub fn create_empty_file(file_path: &str) -> Result<(), ToDoErrors> {
    if let Ok(_) = std::fs::File::create(file_path) {
        Ok(())
    } else {
        Err(ToDoErrors::DatabaseError)
    }
}

pub fn file_exists(file_path: &str) -> bool {
    Path::new(file_path).exists()
}
pub fn create_update_settings(name: &str) -> Result<(), ToDoErrors> {
    let settings = Settings {
        default_project: name.to_string(),
        default_file: format!("{}.json", name),
    };
    if settings.set_settings("settings.json").is_ok() {
        println!("Project '{:?}' set as default", settings);
        Ok(())
    } else {
        Err(ToDoErrors::DatabaseError)
    }
}

pub fn get_settings() -> Result<Settings, ToDoErrors> {
    if let Ok(settings) = Settings::read_settings("settings.json") {
        Ok(settings)
    } else {
        Err(ToDoErrors::NotFoundSettingsError)
    }
}
