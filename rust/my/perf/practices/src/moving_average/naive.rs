// Time: O(window)
// Space: O(window)

use std::collections::VecDeque;

pub struct MovingAverage {
    limit: i32,
    values: VecDeque<i32>,
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        Self { limit: size, values: VecDeque::new() }
    }

    pub fn next(&mut self, val: i32) -> f64 {
        self.values.push_back(val);

        if self.values.len() as i32 > self.limit {
            let _ = self.values.pop_front().unwrap();
        }

        let sum: i32 = self.values.iter().sum();

        sum as f64 / self.values.len() as f64
    }
}
