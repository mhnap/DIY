fn main() {
    {
        let x = 127_i8;
        println!("{x} = {:08b}", x); // 127 = 01111111
        let (y, o) = x.overflowing_add(1); // wrap
        println!("{y} = {:08b} {o}", y); // -128 = 10000000
        let (z, o) = y.overflowing_add(1); // no wrap
        println!("{z} = {:08b} {o}", z); // -127 = 10000001
        // ... no wraps
        let q = -1_i8;
        println!("{q} = {:08b}", q); // -1 = 11111111
        let (p, o) = q.overflowing_add(1); // no wrap
        println!("{p} = {:08b} {o}", p); // 0 = 00000000
        // ... no wraps
        let w = 127_i8;
        println!("{w} = {:08b}", w); // 127 = 01111111
        let (k, o) = w.overflowing_add(1); // wrap
        println!("{k} = {:08b} {o}", k); // -128 = 10000000
    }
    {
        let a = 250_u8;
        dbg!(a.wrapping_add(10)); // 4         wrap around
        dbg!(a.checked_add(10)); // None      Option, None on overflow
        dbg!(a.saturating_add(10)); // 255       clamp to MIN/MAX
        dbg!(a.overflowing_add(10)); // (4, true) wrapped value plus flag
    }
    {
        let x = 0b0100_0000_u8; // 64
        dbg!(x << 1); // 128
        dbg!(x << 2); // 0, bits dropped, no panic
        // x << 8 panics in debug ("shift left with overflow"); masks the amount in release
        dbg!(x.checked_shl(8)); // None
    }
    {
        /*
            The absolute value of i8::MIN cannot be represented as an i8,
            and attempting to calculate it will cause an overflow.
            This means that code in debug mode will trigger a panic on this case
            and optimized code will return i8::MIN without a panic.

            If you do not want this behavior, consider using unsigned_abs instead.
        */

        dbg!((-127 as i8).abs());
        dbg!((-128 as i8).abs());
        dbg!((-127 as i8).unsigned_abs());
        dbg!((-128 as i8).unsigned_abs());

        // unsigned_abs() returns u8, which has range 0..=255. Now 128 fits.
        // The operation is the same bit trick (invert and add 1),
        // but the result is reinterpreted as unsigned:
        dbg!((-128 as i8).abs() as u8);

        /*
           -127 = 1000_0001
           -128 = 1000_0000   (i8::MIN)
            127 = 0111_1111   (i8::MAX)

            The range is asymmetric: −128..=127.
            There is a negative value with no positive twin,
            because zero takes one of the non-negative slots.
        */
    }
}
