// v2a: count the digits first, then write them straight into place from the end.
// No reversal pass, one bounds check per number.

pub struct Format {
    buffer: [u8; 2048],
    ptr: usize,
}

const POW10: [u32; 10] = [
    1, 10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000, 100_000_000, 1_000_000_000,
];

// Number of decimal digits of x. x must not be 0.
fn digit_count(x: u32) -> usize {
    let b = 31 - x.leading_zeros(); // floor(log2 x)
    let t = ((b + 1) * 1233 >> 12) as usize; // ~log10(2^(b+1)), at most 1 too big
    // POW10[t] is the smallest number with t + 1 digits
    t + 1 - (x < POW10[t]) as usize
}

impl Format {
    pub fn new() -> Self {
        Self { buffer: [0; 2048], ptr: 0 }
    }

    pub fn append(&mut self, c: u8) {
        self.buffer[self.ptr] = c;
        self.ptr += 1;
    }

    pub fn append_str(&mut self, data: &str) {
        for &b in data.as_bytes() {
            self.append(b);
        }
    }

    pub fn finish(&mut self) {
        self.append(0);
    }

    pub fn decimal_append_non_neg(&mut self, mut value: u32) {
        if value == 0 {
            self.append(b'0');
            return;
        }

        let d = digit_count(value);
        let out = &mut self.buffer[self.ptr..self.ptr + d];
        let mut pos = d;

        while value >= 10 {
            pos -= 1;
            out[pos] = (value % 10) as u8 + b'0';
            value /= 10;
        }
        out[0] = value as u8 + b'0';

        self.ptr += d;
    }

    pub fn reset(&mut self) {
        self.ptr = 0;
    }

    pub fn data(&self) -> &[u8] {
        &self.buffer[..self.ptr]
    }

    pub fn size(&self) -> usize {
        self.ptr
    }
}
