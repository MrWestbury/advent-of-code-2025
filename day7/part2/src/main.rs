// Day 7, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod manifold;
mod point;

use std::env;
use crate::manifold::Manifold;

fn main() {
  println!("Day 7 - Part 2");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);
  let mut man = Manifold::new();
  for line in data {
    man.process_line(line);
  }
  
  let unis = man.start_run();
  println!("Universes: {}", unis);
}
