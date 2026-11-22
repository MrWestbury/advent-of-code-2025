use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Point {
  pub x: usize, // Column # from left
  pub y: usize, // Row # from top
}

impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "({}, {})", self.x, self.y)
  }
}

impl Point {
  pub fn new(x: usize, y: usize) -> Self {
    Point { x, y }
  }

  pub fn propagate(&self) -> Point {
    Point { x: self.x, y: self.y + 1 }
  }

  pub fn left_point(&self) -> Point {
    Point { 
      x: self.x-1, 
      y: self.y,
    }
  }

  pub fn right_point(&self) -> Point {
    Point {
      x: self.x + 1,
      y: self.y,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn test_new_point() {
    let pt = Point::new(20, 30);
    assert_eq!(pt.x, 20);
    assert_eq!(pt.y, 30);
  }

  #[test]
  fn test_propagate() {
    let pt = Point::new(25, 35);
    assert_eq!(pt.y, 35);
    let next = pt.propagate();
    assert_eq!(next.y, 36);
  }

  #[test]
  fn test_left_point() {
    let pt = Point::new(25, 35);
    assert_eq!(pt.x, 25);
    assert_eq!(pt.y, 35);
    let left = pt.left_point();
    assert_eq!(left.x, 24);
    assert_eq!(left.y, 35);
  }

  #[test]
  fn test_right_point() {
    let pt = Point::new(45, 65);
    assert_eq!(pt.x, 45);
    assert_eq!(pt.y, 65);
    let right = pt.right_point();
    assert_eq!(right.x, 46);
    assert_eq!(right.y, 65);
  }
}