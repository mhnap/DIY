// Time: O(1)
// Space: O(window)
//
// Same as circular_queue_v2, but once the window is full multiply by a
// precomputed 1 / window instead of dividing. The result can differ from
// sum / window in the last bit, which is fine for this problem but not bit-exact.

pub struct MovingAverage {
    limit: i32,
    values: Vec<i32>,
    sum: i32,
    index: usize,
    count: i32,
    inv: f64, // 1.0 / window
}

impl MovingAverage {
    pub fn new(size: i32) -> Self {
        Self {
            limit: size,
            values: vec![0; size as usize],
            sum: 0,
            index: 0,
            count: 0,
            inv: 1.0 / size as f64,
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

        // Filling phase: the divisor changes every call, so keep the division
        if self.count < self.limit {
            self.count += 1;
            return self.sum as f64 / self.count as f64;
        }

        // Full window: the divisor is always `limit`
        self.sum as f64 * self.inv
    }
}
