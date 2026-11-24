// v3: count the digits, then jump to a routine that writes exactly that many
// digits. No loop, no reversal. Each arm slices the buffer into a fixed-size
// array, so the bounds check happens once per number.

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

// Digit k (0 = last digit) of v
fn digit(v: u32, k: u32) -> u8 {
    (v / 10u32.pow(k) % 10) as u8 + b'0'
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

    pub fn decimal_append_non_neg(&mut self, value: u32) {
        if value == 0 {
            self.append(b'0');
            return;
        }

        let d = digit_count(value);
        let out = &mut self.buffer[self.ptr..self.ptr + d];

        match d {
            1 => {
                out[0] = digit(value, 0);
            }
            2 => {
                out[0] = digit(value, 1);
                out[1] = digit(value, 0);
            }
            3 => {
                out[0] = digit(value, 2);
                out[1] = digit(value, 1);
                out[2] = digit(value, 0);
            }
            4 => {
                out[0] = digit(value, 3);
                out[1] = digit(value, 2);
                out[2] = digit(value, 1);
                out[3] = digit(value, 0);
            }
            5 => {
                out[0] = digit(value, 4);
                out[1] = digit(value, 3);
                out[2] = digit(value, 2);
                out[3] = digit(value, 1);
                out[4] = digit(value, 0);
            }
            6 => {
                out[0] = digit(value, 5);
                out[1] = digit(value, 4);
                out[2] = digit(value, 3);
                out[3] = digit(value, 2);
                out[4] = digit(value, 1);
                out[5] = digit(value, 0);
            }
            7 => {
                out[0] = digit(value, 6);
                out[1] = digit(value, 5);
                out[2] = digit(value, 4);
                out[3] = digit(value, 3);
                out[4] = digit(value, 2);
                out[5] = digit(value, 1);
                out[6] = digit(value, 0);
            }
            8 => {
                out[0] = digit(value, 7);
                out[1] = digit(value, 6);
                out[2] = digit(value, 5);
                out[3] = digit(value, 4);
                out[4] = digit(value, 3);
                out[5] = digit(value, 2);
                out[6] = digit(value, 1);
                out[7] = digit(value, 0);
            }
            9 => {
                out[0] = digit(value, 8);
                out[1] = digit(value, 7);
                out[2] = digit(value, 6);
                out[3] = digit(value, 5);
                out[4] = digit(value, 4);
                out[5] = digit(value, 3);
                out[6] = digit(value, 2);
                out[7] = digit(value, 1);
                out[8] = digit(value, 0);
            }
            10 => {
                out[0] = digit(value, 9);
                out[1] = digit(value, 8);
                out[2] = digit(value, 7);
                out[3] = digit(value, 6);
                out[4] = digit(value, 5);
                out[5] = digit(value, 4);
                out[6] = digit(value, 3);
                out[7] = digit(value, 2);
                out[8] = digit(value, 1);
                out[9] = digit(value, 0);
            }
            _ => unreachable!(), // a u32 has at most 10 digits
        }

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
