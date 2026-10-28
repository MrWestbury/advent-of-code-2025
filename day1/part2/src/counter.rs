pub struct Counter {
    pub count: i32,
    pub zerod: i32,
}

impl Counter {
  pub fn new() -> Self {
    Counter { 
      count: 50,
      zerod: 0
    }
  }

  pub fn add(&mut self, value: i32) {
    let new_value:i32 = self.count + value;
    if new_value > 99 {
      let excess:i32 = new_value % 100;
      self.zerod += new_value / 100;
      self.count = excess;
    }
    else {
      self.count = new_value;
    }
    assert!(self.count >= 0 && self.count <= 99, "Counter value out of bounds: {}", self.count);
  }

  pub fn subtract(&mut self, value: i32) {
    let new_value:i32 = self.count - value;
    if new_value < 0 && self.count > 0 { // If the new value is negative and the current count is positive, it means we crossed zero at least once
      self.zerod += 1 + (-new_value - 1) / 100; // Calculate how many times we crossed zero
      self.count = 100 + (new_value % 100); // Wrap around to the positive side
    } else if new_value < 0 && self.count == 0 { // If the new value is negative and the current count is zero, it means we only crossed zero if the new value is less than -100
      self.zerod += (-new_value - 1) / 100; // Calculate how many times we crossed zero
      self.count = 100 + (new_value % 100); // Wrap around to the positive side
    } else {
      self.count = new_value;
    }

    if self.count == 100 {
      self.count = 0;
    }
    if self.count == 0 {
      self.zerod += 1;
    }
    assert!(self.count >= 0 && self.count <= 99, "Counter value out of bounds: {}", self.count);
  }
}