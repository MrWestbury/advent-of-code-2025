struct Point {
    x: usize,
    y: usize,
}

pub struct Grid {
    width: usize,
    height: usize,
    cells: Vec<Point>,
    counts: Vec<Vec<i32>>,
    populated_rows: usize,
}

impl Grid {
  pub fn new(width: usize, height: usize) -> Grid {
    let cells: Vec<Point> = Vec::new();
    let mut counts: Vec<Vec<i32>> = Vec::new();
    for _ in 0..height {
      let mut count_row: Vec<i32> = Vec::new();
      for _ in 0..width {
        count_row.push(0);
      }
      counts.push(count_row);
    }
    Grid { width, height, cells, counts, populated_rows: 0 }
  }

  pub fn process_next_line(&mut self, line: &str) {
    let chars: Vec<char> = line.chars().collect();
    for (x, c) in chars.into_iter().enumerate() {
      if c == '@' {
        self.increment_count_around_cell(x as isize, self.populated_rows as isize);
        self.cells.push(Point { x, y: self.populated_rows });
      }
    }
    self.populated_rows += 1;
  }

  fn increment_count_around_cell(&mut self, x: isize, y: isize) {
    for i in -1..=1 {
      for j in -1..=1 {
        let new_x = x as isize + i;
        let new_y = y as isize + j;
        if new_x >= 0 && new_x < self.width as isize && new_y >= 0 && new_y < self.height as isize && !(i == 0 && j == 0) {
          self.counts[new_y as usize][new_x as usize] += 1;
        }
      }
    }
  }

  pub fn print_counts(&self) {
    for row in &self.counts {
      for count in row {
        print!("{} ", count);
      }
      println!();
    }
  }

  pub fn get_count_lower_than(&self, threshold: u32) -> u32 {
    let mut count: u32 = 0;
    for cell in &self.cells {
      if self.counts[cell.y][cell.x] < threshold as i32 {
        count += 1;
      }
    }
    return count;
  }
}