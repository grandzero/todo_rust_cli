use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Settings {
    pub default_project: String,
    pub default_file: String,
}

impl Settings {
    pub fn set_settings(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let serialized = serde_json::to_string(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(serialized.as_bytes())?;
        Ok(())
    }
    pub fn read_settings(filename: &str) -> Result<Settings, Box<dyn Error>> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let settings: Settings = serde_json::from_str(&contents)?;
        Ok(settings)
    }
}
