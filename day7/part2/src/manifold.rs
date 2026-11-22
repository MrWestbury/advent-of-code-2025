use crate::point::Point;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Manifold {
  splitters: HashMap<String, Point>,
  cache: HashMap<String, usize>,
  pub start: Point,
  rows_processed: usize,
  col_count: usize,
}

impl Manifold {

  pub fn new() -> Manifold {
    Manifold {
      splitters: HashMap::new(),
      cache: HashMap::new(),
      start: Point::new(0, 0),
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
        'S' => self.start = Point::new(idx, self.rows_processed),
        '^' => {
          let pt = Point::new(idx, self.rows_processed);
          self.splitters.insert(pt.to_string(), pt);
        }
        '.' => {}, 
        _ => panic!("Unsupported char"),
      }
    }
    self.rows_processed += 1;
  }

  pub fn start_run(&mut self) -> usize {
    return self.spawn_universe(self.start);
  }

  fn spawn_universe(&mut self, pt: Point) -> usize {
    let mut max_loops = 1000; // Safety
    let mut next_point = pt.propagate();
    loop {
      if next_point.y > self.rows_processed {
        return 1;
      }

      let next_point_key = &next_point.to_string();
      if self.splitters.contains_key(next_point_key) {
        if self.cache.contains_key(next_point_key) {
          return self.cache.get(next_point_key).unwrap().clone();
        }
        
        let left = next_point.left_point();
        let left_point_size = self.spawn_universe(left);

        let right = next_point.right_point();
        let right_point_size = self.spawn_universe(right);

        let total = left_point_size + right_point_size;
        self.cache.insert(next_point_key.to_string(), total);
        return total;
      }

      max_loops -= 1;
      if max_loops <= 0 {
        assert!(false, "safety hit");
        break;
      }

      // Move down a row
      next_point = next_point.propagate();
    }
    println!("do we hit here?");
    return 0;
  }

  #[allow(dead_code)]
  pub fn print(&self) {
    for row in 0..self.rows_processed {
      for col in 0..self.col_count {
        let test_pt = Point::new(col, row);
        if test_pt == self.start {
          print!("S");
        } else if self.splitters.contains_key(&test_pt.to_string()) {
          print!("^");
        } else {
          print!(".");
        }
      }
      println!("");
    }
  }

  #[allow(dead_code)]
  pub fn print_splitters(&self) {
    for splitter in self.splitters.keys().into_iter() {
      print!("{}, ", splitter);
    }
    println!("");
  }
}