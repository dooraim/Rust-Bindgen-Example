use std::fs::read_to_string;

pub trait MyLibrary<T> {
    fn read_file(path_file: &str) -> Vec<String> {

        let row: Vec<String> = read_to_string(path_file) 
        .unwrap_or_else(|err| {
            panic!("Failed to open file in {}", path_file);
        })  // panic on possible file-reading errors
        .lines()  // split the string into an iterator of string slices
        .map(String::from)  // make each slice into a string
        .collect(); // gather them together into a vector

        row
    }

    fn create_struct(path_file: &str) -> Vec<T>;
}
