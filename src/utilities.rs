use std::env;
use std::fs;
use std::path::Path;

const PATH: &str = ".juric_temp";

pub fn cd_to_temp_folder() {
    let _path = Path::new(PATH);
    assert!(env::set_current_dir(&_path).is_ok())
}

pub fn create_temp_folder() {
    if !Path::new(PATH).is_dir() {
        fs::create_dir(PATH).expect("Failed to create directory")
    }
}
