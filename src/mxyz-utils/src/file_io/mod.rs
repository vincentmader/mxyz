use std::fs;

pub fn load_file_to_str(path_to_file: &str) -> String {
    let error_msg = format!("ERROR while reading the file \"{}\"", path_to_file);
    fs::read_to_string(path_to_file).expect(&error_msg)
}
