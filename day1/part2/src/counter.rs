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
        let mut click_count:i32 = 0;
        if self.count > 99 {
            click_count = self.count / 100;
            self.zero_count += click_count;
            let excess:i32 = self.count % 100;
            self.count = excess;
        }
        println!(" - The dial is rotated R{} to point to {}, zero points {}", value, self.count, click_count);
    }

    pub fn subtract(&mut self, value: i32) {
        self.count -= value;
        let mut click_count:i32 = 0;
        if self.count < 0 {
            click_count = (self.count / -100);
            self.zero_count += click_count;
            self.count = self.count.rem_euclid(100);
        }
        println!(" - The dial is rotated L{} to point to {}, zero points {}", value, self.count, click_count);
    }
}