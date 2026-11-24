// v2b: same as v2a, but two digits per step using a "00".."99" table.
// One division by 100 produces two digits.

pub struct Format {
    buffer: [u8; 2048],
    ptr: usize,
}

// Pair i lives at DIGITS[2 * i..2 * i + 2]
const DIGITS: &[u8; 200] = b"\
0001020304050607080910111213141516171819\
2021222324252627282930313233343536373839\
4041424344454647484950515253545556575859\
6061626364656667686970717273747576777879\
8081828384858687888990919293949596979899";

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

        // Two digits at a time, from the end
        while value >= 100 {
            let p = (value % 100) as usize;
            value /= 100;
            pos -= 2;
            out[pos] = DIGITS[2 * p];
            out[pos + 1] = DIGITS[2 * p + 1];
        }
        // 1 or 2 digits left
        if value >= 10 {
            let p = value as usize;
            out[0] = DIGITS[2 * p];
            out[1] = DIGITS[2 * p + 1];
        } else {
            out[0] = value as u8 + b'0';
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
