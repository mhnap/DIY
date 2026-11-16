// Time: O(1)
// Space: O(window)

use std::collections::VecDeque;

pub struct MovingAverage {
    limit: i32,
    values: VecDeque<i32>,
    sum: i32,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        Self { limit: size, values: VecDeque::new(), sum: 0 }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.values.push_back(val);
        self.sum += val;

        if self.values.len() as i32 > self.limit {
            let front = self.values.pop_front().unwrap();
            self.sum -= front;
        }

        self.sum as f64 / self.values.len() as f64
    }
}
