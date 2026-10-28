// Day 3, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
use std::env;
use std::str;

fn main() {
  println!("Day 3 - Part 1");
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
    let joltage = get_joltage_from_bank(&bank);
    println!("Bank: {bank} Joltage: {joltage}");
    total_joltage += joltage as i64;
  }
  println!("Total Joltage: {total_joltage}");
}

fn get_joltage_from_bank(bank: &str) -> i32 {
  let joltages: Vec<i32> = bank.chars().map(|s| s.to_digit(10).map(|d| d as i32).unwrap()).collect();
  let mut first_digit_index = 0;
  let mut first_digit_value = 0;
  let mut second_digit_value = 0;
  for i in 0..joltages.len()-1 {
    if joltages[i] > first_digit_value {
      first_digit_value = joltages[i];
      first_digit_index = i;
    }
  }
  for i in first_digit_index+1..joltages.len() {
    if joltages[i] > second_digit_value {
      second_digit_value = joltages[i];
    }
  }
  return format!("{}{}", first_digit_value, second_digit_value).parse::<i32>().unwrap();
}
