pub struct Calculation {
  pub values: Vec<i64>,
  pub operator: char,
}

impl Calculation {
  pub fn new(initial_value: i64) -> Self {
    Self {
      values: vec![initial_value],
      operator: '#',
    }
  }

  pub fn calculate(&self) -> i64 {
    match self.operator {
      '+' => self.values.iter().sum(),
      '*' => self.values.iter().product(),
      _ => panic!("Unsupported operator"),
    }
  }
}