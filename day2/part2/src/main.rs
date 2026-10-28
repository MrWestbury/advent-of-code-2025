#[path="../../../modules/readdata.rs"]
mod readdata;
use std::env;
use std::str;

fn main() {
  println!("Day 1 - Part 1");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);
  let data_as_str = data[0].as_str();
  let ranges: Vec<&str> = data_as_str.split(',').collect();
  let mut invalid_numbers: Vec<i64> = Vec::new();
  for range in ranges {
    let bounds: Vec<&str> = range.split("-").collect();
    let lower_bound:i64 = bounds[0].trim().parse().unwrap();
    let upper_bound:i64 = bounds[1].trim().parse().unwrap();
    let mut is_valid = true;
    for n in lower_bound..=upper_bound {
      if !is_number_valid(n) {
        is_valid = false;
        invalid_numbers.push(n);
      }
    }
    println!("Lower Bound: {lower_bound} Upper Bound: {upper_bound} Valid: {is_valid}");
  }
  println!("Sum of invalid numbers: {}", invalid_numbers.iter().sum::<i64>());
}

fn is_number_repeated_twice(n: i64) -> bool {
  let num_as_str = n.to_string();
  let num_len = num_as_str.len();
  if num_len == 1 {
    return false;
  }
  if num_len % 2 != 0 {
    return false;
  }
  let check_str = num_as_str.chars().take(num_len / 2).collect::<String>();
  let repeated_str = check_str.repeat(2);
  return repeated_str == num_as_str;
}

fn is_number_valid(n: i64) -> bool {
  let num_as_str = n.to_string();
  let factors = factors(num_as_str.len() as i32);

  for factor in factors {
    let check_str = num_as_str.chars().take(factor as usize).collect::<String>();
    let repeat_num = num_as_str.len() / factor as usize;
    let repeated_str = check_str.repeat(repeat_num);
    if repeated_str == num_as_str {
      return false;
    }
  }
  return true;
}

fn factors(n: i32) -> Vec<i32> {
  let mut factors: Vec<i32> = Vec::new();
  for i in 1..=n {
    if n % i == 0 {
      factors.push(i);
    }
  }
  factors.pop(); // Remove the last factor which is the number itself
  return factors;
}
