use std::env;
// use std::fs::{self, File};
// use std::io::{Error, Write};
// use std::path::PathBuf;
mod models;
mod project;
use project::create_project;

fn main() {
    // println!("Dir : {}", env::current_dir().unwrap().display());

    create_project("First project".to_string(), "Test project".to_string());

    #[cfg(target_os = "windows")]
    {
        println!("Running on Windows");
        // Windows specific code here
    }

    #[cfg(target_os = "linux")]
    {
        println!("Running on Linux");
        // Linux specific code here
    }

    #[cfg(target_os = "macos")]
    {
        println!("Running on macOS");
        // macOS specific code here
    }
    // Step 1: Access the %APPDATA% environment variable
    // let appdata = env::var("APPDATA").expect("Failed to find APPDATA");

    // // Step 2: Create a path to your application's directory
    // let mut app_dir = PathBuf::from(appdata);
    // app_dir.push("YourAppName"); // Replace with your app's name

    // // Create the directory if it does not exist
    // fs::create_dir_all(&app_dir)?;

    // // Step 3: Write to a file in your directory
    // let mut file_path = app_dir.join("todo_list.txt"); // Example file name
    // let mut file = File::create(file_path)?;
    // writeln!(file, "Your to-do list contents go here")?;

    // Ok(())
}
