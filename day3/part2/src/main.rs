// Day 3, Part 2
#[path="../../../modules/readdata.rs"]
mod readdata;
use std::env;
use std::str;

fn main() {
  println!("Day 3 - Part 2");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut total_joltage: i64 = 0;
  for bank in data {
    let joltage = get_joltage_from_bank(&bank, 12);
    println!("Bank: {bank} Joltage: {joltage}");
    total_joltage += joltage as i64;
  }
  println!("Total Joltage: {total_joltage}");
}

fn get_joltage_from_bank(bank: &str, max_digits: usize) -> i64 {
  let joltages: Vec<i32> = bank.chars().map(|s| s.to_digit(10).map(|d| d as i32).unwrap()).collect();
  let joltage_len = joltages.len();
  let mut first_digit_index: usize = 0;
  let mut digits: Vec<String> = Vec::new();
  for place in 0..max_digits {
    let mut max_digit_value: i32 = 0;
    let mut max_digit_index: usize = 0;
    for i in first_digit_index..(joltage_len - (max_digits - place - 1)) {
      if joltages[i] > max_digit_value {
        max_digit_value = joltages[i];
        max_digit_index = i;
      }
    }
    digits.push(max_digit_value.to_string());
    first_digit_index = max_digit_index + 1;
  }
  return digits.join("").parse::<i64>().unwrap();
}
