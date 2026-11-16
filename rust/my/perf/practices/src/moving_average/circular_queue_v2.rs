// Time: O(1)
// Space: O(window)
//
// Same as circular_queue, but use comparision to update the index value,
// instead of an expensive division.

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

        self.index += 1;
        if self.index == self.values.len() {
            self.index = 0;
        }

        if self.count < self.limit {
            self.count += 1;
        }

        self.sum as f64 / self.count as f64
    }
}
