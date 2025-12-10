pub struct Counter {
    pub count: i32,
    pub zero_count: i32,
}

impl Counter {
    pub fn new() -> Self {
        Counter { count: 50, zero_count: 0 }
    }

    pub fn add(&mut self, value: i32) {
        self.count += value;
        if self.count > 99 {
            let excess:i32 = self.count % 100;
            self.count = excess;
        }
        if self.count == 0 {
            self.zero_count += 1;
        }
        println!(" - The dial is rotated R{} to point to {}", value, self.count);
    }

    pub fn subtract(&mut self, value: i32) {
        self.count -= value;
        
        if self.count < 0 {
            self.count = self.count.rem_euclid(100);
        }
        if self.count == 0 {
            self.zero_count += 1;
        }
        println!(" - The dial is rotated L{} to point to {}", value, self.count);
    }
}