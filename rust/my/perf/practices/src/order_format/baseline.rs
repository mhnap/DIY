pub struct Format {
    buffer: [u8; 2048],
    ptr: usize,
}

impl Format {
    pub fn new() -> Self {
        Self {
            buffer: [0; 2048],
            ptr: 0,
        }
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
        let start_pos = self.ptr;
        loop {
            self.append((value % 10) as u8 + b'0');
            value /= 10;
            if value == 0 {
                break;
            }
        }
        // Reverse the digits.
        let mut end = self.ptr - 1;
        let mut start = start_pos;
        while end > start {
            self.buffer.swap(start, end);
            start += 1;
            end -= 1;
        }
    }

    pub fn decimal_append(&mut self, mut value: i32) {
        if value < 0 {
            self.append(b'-');
            value = value.wrapping_neg(); // C++ overflows on INT_MIN; Rust would panic in debug
        }
        self.decimal_append_non_neg(value as u32);
    }

    // For tests and benchmarks
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

pub fn new_order(format: &mut Format, id: &str, price: i32, quantity: u32) {
    format.append_str("NEW ");
    format.append_str(id);
    format.append(b' ');
    format.decimal_append(price);
    format.append(b' ');
    format.decimal_append_non_neg(quantity);
    format.finish();
}
