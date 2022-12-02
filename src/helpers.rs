use std::env;
use std::fs;


pub fn read_file(input_path: &str) -> String {
    let binding = env::current_dir().unwrap();
    let cur_dir = binding.as_path().to_str().unwrap();
    let file_path = cur_dir.to_owned() + input_path;
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    return contents;
}
