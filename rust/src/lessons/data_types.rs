use std::mem::size_of_val;

fn main() {
    let a: u8 = 255;
    dbg!(a, size_of_val(&a));

    // let b: i8 = -255;
    // error: literal out of range for `i8`
    //  --> src/lessons/data_types.rs:6:18
    //   |
    // 6 |     let b: i8 = -255;
    //   |                  ^^^
    //   |
    //   = note: the literal `255` does not fit into the type `i8` whose range is `-128..=127`
    //   = help: consider using the type `i16` instead
    //   = note: `#[deny(overflowing_literals)]` on by default
    let mut b: i8 = 127;
    dbg!(b, size_of_val(&b));

    // Can use "_" as a visual separator
    let c: u16 = 2_5_6;
    dbg!(c, size_of_val(&c));

    // Is arch dependent (64 bits if you’re on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture)
    let d: isize = 0;
    dbg!(d, size_of_val(&d));

    // Default is f64 for float
    let e = 0.0;
    dbg!(e, size_of_val(&e));

    // Just bool
    let f = true;
    dbg!(f, size_of_val(&f));

    // Char represents a Unicode Scalar Value
    let g = '😻';
    dbg!(g, size_of_val(&g));

    // &str type
    let h = "h";
    dbg!(h, size_of_val(&h));

    // Tuple
    let tup = (500, 6.4, 1);
    dbg!(tup, size_of_val(&tup));

    // Tuple destructuring
    let (i, j, _) = tup;
    dbg!(i, size_of_val(&i));
    dbg!(j, size_of_val(&j));
    dbg!(tup.2, size_of_val(&tup.2));
    // dbg!(size_of_val(&tup.3));
    // error[E0609]: no field `3` on type `({integer}, {float}, {integer})`
    //   --> src/lessons/data_types.rs:53:27
    //    |
    // 53 |     dbg!(size_of_val(&tup.3));
    //    |                           ^

    // Unit type (empty type). Expressions implicitly return the unit value if they don’t return any other value.
    let unit = ();
    dbg!(unit, size_of_val(&unit));

    // Overflow will cause panic in debug and two’s complement wrapping in release
    b += 1;
    dbg!(b);
}
