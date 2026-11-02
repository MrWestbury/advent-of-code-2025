// Day 5, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod ingredients_db;
mod ingredient_range;

use std::env;
use std::str;
use crate::ingredients_db::IngredientsDB;

fn main() {
  println!("Day 5 - Part 2");
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

  for line in data {
    if line.trim().is_empty() {
      break;
    }

    let parts: Vec<&str> = line.split('-').collect();
    let start: i64 = parts[0].trim().parse().unwrap();
    let end: i64 = parts[1].trim().parse().unwrap();
    ingredients_db.add_ingredient_id_range(start, end);
  }
  let mut did_merge = true;
  let mut merges = 0;
  while did_merge {
    did_merge = ingredients_db.merge_ranges();
    if did_merge {
      merges += 1;
    }
    // ingredients_db.print_ranges();
  }
  // ingredients_db.print_ranges();
  println!("Range count: {}\nNums In Ranges: {}\nMerges: {merges}",
    ingredients_db.get_range_count(), 
    ingredients_db.total_nums_in_ranges());
  
}
