// Day 4, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
use std::env;

fn main() {
  println!("Day 4 - Part 1");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);
  println!("Grid size: {} x {}", data[0].len(), data.len());

  let mut grid = grid::Grid::new(data[0].len(), data.len());
  for line in data {
    grid.process_next_line(&line);
  }

  println!("Answer: {}", grid.get_count_lower_than(4));
}
