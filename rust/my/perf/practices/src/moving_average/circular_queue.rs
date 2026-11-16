// Time: O(1)
// Space: O(window)

pub struct MovingAverage {
    limit: i32,
    values: Vec<i32>,
    sum: i32,
    index: usize,
    count: i32,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        Self {
            limit: size,
            values: vec![0; size as usize],
            sum: 0,
            index: 0,
            count: 0,
        }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.sum -= self.values[self.index];
        self.values[self.index] = val;
        self.sum += val;
        self.index = (self.index + 1) % self.limit as usize;
        if self.count < self.limit {
            self.count += 1;
        }
        self.sum as f64 / self.count as f64
    }
}
