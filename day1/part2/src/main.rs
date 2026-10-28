#[path="../../../modules/readdata.rs"]
mod readdata;
mod counter;
use std::env;
use counter::Counter;

fn main() {
  println!("Day 1 - Part 2");

  let mut input_file:String = "../test.txt".to_string();

  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut counter: Counter = Counter::new();

  for line in data {
    let firstchar:String = line.chars().take(1).collect();
    let value:i32 = line[1..].trim().parse().unwrap();
    match firstchar.as_str() {
      "R" => counter.add(value),
      "L" => counter.subtract(value),
      _ => {}
    }
    println!("Command: {line:<5}  {0:<3} {1:<3}", counter.count, counter.zerod);
  }
  println!("Final Counter Value: {}", counter.count);
  println!("Counter was zeroed {} times", counter.zerod);
}