use crate::point::Point;

#[derive(Debug)]
pub struct Manifold {
  pub split_count: i32,
  splitters: Vec<Point>,
  start: Point,
  tachyons: Vec<Point>,
  rows_processed: usize,
  col_count: usize,
}

impl Manifold {

  pub fn new() -> Manifold {
    Manifold {
      split_count: 0,
      splitters: Vec::new(),
      start: Point {x: 0, y:0},
      tachyons: Vec::new(),
      rows_processed: 0,
      col_count: 0,
    }
  }

  pub fn process_line(&mut self, line: String) {
    for (idx, c) in line.chars().into_iter().enumerate() {
      if line.len() > self.col_count {
        self.col_count = line.len();
      }
      match c {
        'S' => self.start = Point { x: idx, y: self.rows_processed },
        '^' => self.splitters.push(Point {x: idx, y: self.rows_processed}),
        '.' => {}, 
        _ => panic!("Unsupported char"),
      }
    }
    self.rows_processed += 1;
  }

  pub fn run_frame(&mut self) -> bool {
    if self.tachyons.len() == 0 {
      let next_tachyon = self.start.propagate();
      self.add_tachyon(next_tachyon);
    }

    for row in 0..self.rows_processed {
      for col in 0..self.col_count {
        let test_pt = Point::new(col, row);
        if self.tachyons.contains(&test_pt) {
          let next_tachyon = test_pt.propagate();
          self.add_tachyon(next_tachyon);
        }
      }
    }
    true
  }

  fn add_tachyon(&mut self, pt: Point) {
    if self.splitters.contains(&pt) {
      self.tachyons.push(Point { x: pt.x - 1, y: pt.y});
      self.tachyons.push(Point { x: pt.x + 1, y: pt.y});
      self.split_count += 1;
    } else {
      self.tachyons.push(pt);
    }
  }

  pub fn print(&self) {
    for row in 0..self.rows_processed {
      for col in 0..self.col_count {
        let test_pt = Point::new(col, row);
        if test_pt == self.start {
          print!("S");
        } else if self.splitters.contains(&test_pt) {
          print!("^");
        } else if self.tachyons.contains(&test_pt) {
          print!("|");
        } else {
          print!(".");
        }
      }
      println!("");
    }
  }
}