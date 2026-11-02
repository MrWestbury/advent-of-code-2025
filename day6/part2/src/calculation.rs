#[derive(Debug, PartialEq)]
pub struct Calculation {
  pub start_index: usize,
  pub length: usize,
  pub values: Vec<String>,
  pub operator: char,
}

impl Calculation {
  pub fn new(operator: char, start:usize) -> Self {
    Self {
      start_index: start,
      length: 0,
      values: Vec::new(),
      operator,
    }
  }

  pub fn calculate(&self) -> i64 {
    let mut num_vals: Vec<i64> = Vec::new();
    for i in 0..self.length {
      let mut num_as_chars:Vec<char> = Vec::new();
      for val in &self.values {
        let c = val.chars().nth(i).unwrap();
        if c != ' ' {
          num_as_chars.push(c);
        }
      }
      let num_as_str = num_as_chars.iter().cloned().collect::<String>();
      num_vals.push(str::parse(&num_as_str).expect("REASON"));
    }
    
    match self.operator {
      '+' => num_vals.iter().sum(),
      '*' => num_vals.iter().product(),
      _ => panic!("Unsupported operator"),
    }
  }
}