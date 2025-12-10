use std::fs::read_to_string;
use std::path::Path;

pub fn read_input(filename: &str) -> Vec<String> {
  if !Path::new(filename).exists() {
    panic!("Input file '{}' does not exist.", filename);
  }
  read_to_string(filename)
      .unwrap()
      .lines()
      .map(String::from)
      .collect()
}