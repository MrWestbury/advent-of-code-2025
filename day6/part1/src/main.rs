// Day 6, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod calc;
use std::env;
use std::str;

fn main() {
  println!("Day 6 - Part 1");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut calculations: Vec<calc::Calculation> = Vec::new();
  let last_line = data.len() - 1;
  let mut line_num = 0;
  for line in data {
    let parts: Vec<&str> = line.split_whitespace().collect();
    for (idx, part) in parts.iter().enumerate() {
      if line_num == 0 {
        let initial_value: i64 = part.parse().unwrap();
        calculations.push(calc::Calculation::new(initial_value));
      } else if line_num == last_line {
        let operator: char = part.chars().next().unwrap();
        calculations[idx].operator = operator;
      } else {
        let value: i64 = part.parse().unwrap();
        calculations[idx].values.push(value);
      }
    }
    line_num += 1;
  }
  let mut total: i64 = 0;
  for calculation in calculations {
    total += calculation.calculate();
  }
  println!("Total: {}", total);
}
