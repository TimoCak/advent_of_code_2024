use std::fs;

pub fn read_input_file(path: &str) -> String {
    fs::read_to_string(path).expect("should read the input file!")
}