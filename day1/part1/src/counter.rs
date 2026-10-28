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
        self.count += value;
        if self.count > 99 {
            let excess:i32 = self.count % 100;
            self.count = excess;
        }
        if self.count == 0 {
            self.zerod += 1;
        }
    }

    pub fn subtract(&mut self, value: i32) {
        self.count -= value;
        if self.count < 0 {
            self.count = 100 + (self.count % 100);
        }
        if self.count == 100 {
            self.count = 0;
        }
        if self.count == 0 {
            self.zerod += 1;
        }
    }
}