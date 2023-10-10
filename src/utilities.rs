use std::env;
use std::fs;
use std::path::Path;

// Define a constant for the temporary folder name
const TEMP_FOLDER: &str = ".juric_temp";

// Change the current directory to the specified folder
pub fn cd_to_folder(folder: &str) {
    set_current_dir(folder);
}

// Change the current directory to the temporary folder
pub fn cd_to_temp_folder() {
    set_current_dir(TEMP_FOLDER);
}

// Create a new folder if it doesn't already exist
pub fn create_folder(folder: &str) {
    create_directory(folder);
}

// Create the temporary folder if it doesn't already exist
pub fn create_temp_folder() {
    create_directory(TEMP_FOLDER);
}

// Set the current working directory to the specified directory
fn set_current_dir(dir: &str) {
    let path = Path::new(dir);
    env::set_current_dir(path).expect("Failed to change directory!");
}

// Create a directory if it doesn't already exist
fn create_directory(dir: &str) {
    if !Path::new(dir).is_dir() {
        fs::create_dir(dir).expect("Failed to create directory");
    }
}
