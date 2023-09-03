use std::env;
use std::fs;
use std::path::Path;

const PATH: &str = ".juric_temp";

pub fn cd_to_folder(folder: &String) {
    let _path = Path::new(folder);
    env::set_current_dir(&_path).expect("Failed to change directory!");
}

pub fn cd_to_temp_folder() {
    let _path = Path::new(PATH);
    env::set_current_dir(&_path).expect("Failed to change directory!");
}

pub fn create_folder(folder: &String) {
    if !Path::new(folder).is_dir() {
        fs::create_dir(folder).expect("Failed to create directory")
    }
}

pub fn create_temp_folder() {
    if !Path::new(PATH).is_dir() {
        fs::create_dir(PATH).expect("Failed to create directory")
    }
}
