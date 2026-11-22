#[derive(Debug, PartialEq, Clone)]
pub struct Point {
  pub x: usize, // Column # from left
  pub y: usize, // Row # from top
}

impl Point {
  pub fn new(x: usize, y: usize) -> Point {
    Point { x, y }
  }

  pub fn propagate(&self) -> Point {
    Point { x: self.x, y: self.y + 1 }
  }
}