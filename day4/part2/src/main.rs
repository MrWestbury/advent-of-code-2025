// Day 4, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod grid;
use std::env;

fn main() {
  println!("Day 4 - Part 2");
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

 
  let mut cells_to_remove = grid.get_cells_lower_than(4);
  let mut total_removed_cells = 0;
  while !cells_to_remove.is_empty() {
    let remove_count = cells_to_remove.len();
    println!("Removing {} cells", remove_count);
    for cell in &cells_to_remove {
      grid.remove_cell(cell);
    }
    total_removed_cells += remove_count;
    grid.calculate_counts();
    cells_to_remove = grid.get_cells_lower_than(4);
  }
  println!("Total removed cells: {}", total_removed_cells);
}
