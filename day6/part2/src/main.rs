// Day 6, Part 1
#[path="../../../modules/readdata.rs"]
mod readdata;
mod calculation;
use std::env;
use crate::calculation::Calculation;

fn main() {
  println!("Day 6 - Part 2");
  let mut input_file:String = "../test.txt".to_string();
  let args: Vec<String> = env::args().collect();
  match args.len() {
    2 => {
      input_file = args[1].clone();
    },
  _ => {}
  }

  let data = readdata::read_input(&input_file);

  let mut calculations: Vec<Calculation> = Vec::new();
  let mut current_calc: Option<Calculation> = None;
  let last_line = data.last().unwrap();
  for (idx, op_char) in last_line.chars().into_iter().enumerate() {
    if op_char == '+' || op_char == '*' {
      if current_calc != None {
        let mut cc = current_calc.unwrap();
        cc.length = idx - cc.start_index - 1;
        calculations.push(cc);
      }
      current_calc = Some(Calculation::new(op_char, idx))
    }
  }

  let mut cc = current_calc.unwrap();
  cc.length = last_line.len() - cc.start_index;
  calculations.push(cc);

  for calculation in &mut calculations {
    let start = calculation.start_index;
    let end = start + calculation.length;
    for line_idx in 0..data.len()-1{
      calculation.values.push(data[line_idx][start..end].to_string());
    }
  }
  let mut total: i64 = 0;
  for calculation in &mut calculations {
    total += calculation.calculate();
  }
  println!("Total: {}", total);
}
