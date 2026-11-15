fn main() {
    {
        dbg!(0.1 + 0.2); // https://0.30000000000000004.com
        dbg!(0.1_f32 + 0.2); // 0.3

        dbg!(0.1 + 0.2 == 0.3); // false
        dbg!(0.1_f32 + 0.2 == 0.3); // true
    }
    {
        let a = 16_777_216_f32;
        println!("{}", a + 1.0 == a); // true, the +1 is lost
        println!("{}", 16_777_217_i32 as f32); // 16777216
        println!("{}", f64::from(16_777_217_i32)); // 16777217, f64 has room

        // classic bug: this loop never ends
        let mut x = 0.0_f32;
        while x < a + 2.0 {
            x += 1.0;
        } // x gets stuck at 16777216
    }
}
