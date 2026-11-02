// Day 5, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod ingredients_db;
use std::env;
use std::str;
use crate::ingredients_db::IngredientsDB;

fn main() {
  println!("Day 5 - Part 1");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut ingredients_db = IngredientsDB::new();
  let mut is_range_section = true;

  let mut spoiled_count = 0;
  let mut total_count = 0;
  for line in data {
    if line.trim().is_empty() {
      is_range_section = false;
      continue;
    }

    if is_range_section {
      let parts: Vec<&str> = line.split('-').collect();
      let start: i64 = parts[0].trim().parse().unwrap();
      let end: i64 = parts[1].trim().parse().unwrap();
      ingredients_db.add_ingredient_id_range(start, end);
    } else {
      let id: i64 = line.trim().parse().unwrap();
      total_count += 1;
      if !ingredients_db.is_ingredient_id_valid(id) {
        println!("Invalid ingredient ID found: {}", id);
        spoiled_count += 1;
      }
    }
  }
  println!("Range count: {}", ingredients_db.get_range_count());
  println!("Total count: {}", total_count);
  println!("Final count: {}", total_count - spoiled_count);
}
