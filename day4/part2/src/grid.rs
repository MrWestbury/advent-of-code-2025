#[derive(Clone, Copy, PartialEq)]
pub struct Point {
  x: usize,
  y: usize,
}

pub struct Grid {
  width: usize,
  height: usize,
  cells: Vec<Point>,
  counts: CountGrid,
  populated_rows: usize,
}

struct CountGrid {
  width: usize,
  height: usize,
  counts: Vec<Vec<i32>>,
}

impl CountGrid {
  pub fn new(width: usize, height: usize) -> CountGrid {
    let mut counts: Vec<Vec<i32>> = Vec::new();
    for _ in 0..height {
      let mut count_row: Vec<i32> = Vec::new();
      for _ in 0..width {
        count_row.push(0);
      }
      counts.push(count_row);
    }
    CountGrid { width, height, counts }
  }

  pub fn reset_counts(&mut self) {
    for row in &mut self.counts {
      for count in row {
        *count = 0;
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

  pub fn increment_count_around_cell(&mut self, pt: Point) {
    for i in -1..=1 {
      for j in -1..=1 {
        let new_x = pt.x as isize + i;
        let new_y = pt.y as isize + j;
        if new_x >= 0 && new_x < self.width as isize && new_y >= 0 && new_y < self.height as isize && !(i == 0 && j == 0) {
          self.counts[new_y as usize][new_x as usize] += 1;
        }
      }
    }
  }

  pub fn get_cells_lower_than(&self, cell_list: Vec<Point>, threshold: u32) -> Vec<Point> {
    let mut result: Vec<Point> = Vec::new();
    for cell in &cell_list {
      if self.counts[cell.y][cell.x] < threshold as i32 {
        result.push(*cell);
      }
    }
    return result;
  }
}

impl Grid {
  pub fn new(width: usize, height: usize) -> Grid {
    let cells: Vec<Point> = Vec::new();
    let counts = CountGrid::new(width, height);
    Grid { width, height, cells, counts, populated_rows: 0 }
  }

  pub fn reset_counts(&mut self) {
    self.counts.reset_counts();
  }
  
  pub fn process_next_line(&mut self, line: &str) {
    let chars: Vec<char> = line.chars().collect();
    for (x, c) in chars.into_iter().enumerate() {
      if c == '@' {
        let pt = Point { x, y: self.populated_rows };
        self.counts.increment_count_around_cell(pt);
        self.cells.push(pt);
      }
    }
    self.populated_rows += 1;
  }

  pub fn calculate_counts(&mut self) {
    self.counts.reset_counts();
    for cell in &self.cells {
      self.counts.increment_count_around_cell(*cell);
    }
  }

  pub fn remove_cell(&mut self, cell: &Point) {
    self.cells.retain(|c| c.x != cell.x || c.y != cell.y);
  }

  pub fn print_counts(&self) {
    self.counts.print_counts();
  }

  pub fn get_cells_lower_than(&self,threshold: u32) -> Vec<Point> {
    let cell_list = self.cells.clone();
    self.counts.get_cells_lower_than(cell_list, threshold)
  }
}