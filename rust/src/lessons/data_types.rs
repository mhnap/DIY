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
    let mut f = true;
    dbg!(f, size_of_val(&f));

    // Bool don't have operations support
    // f += 1;
    // error[E0368]: binary assignment operation `+=` cannot be applied to type `bool`
    //   --> src/lessons/data_types.rs:37:5
    //    |
    // 37 |     f += 1;
    //    |     -^^^^^
    //    |     |
    //    |     cannot use `+=` on type `bool`

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

    // Print addresses
    dbg!(&tup as *const (i32, f64, i32));
    dbg!(&tup.0 as *const i32);
    dbg!(&tup.1 as *const f64);
    dbg!(&tup.2 as *const i32);
    dbg!(&i as *const i32);
    dbg!(&j as *const f64);

    // Unit type (empty type). Expressions implicitly return the unit value if they don’t return any other value.
    let unit = ();
    dbg!(unit, size_of_val(&unit));

    // Because the unit type has a size of 0 bytes, its memory address is not particularly useful or meaningful. (according to ChatGPT)
    println!("unit address = {:p}", &unit);

    // Array
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    dbg!(arr, size_of_val(&arr));

    // Array slice
    dbg!(&arr[1..=3]);

    // 2D array of tuples
    let arr: [[(char, f64); 2]; 2] = [[('1', 2.0), ('3', 4.0)], [('5', 6.0), ('7', 8.0)]];
    dbg!(arr);

    // You can also initialize an array to contain the same value for each element by specifying the initial value, followed by a semicolon, and then the length of the array in square brackets
    let arr = [3; 2];
    dbg!(arr, size_of_val(&arr));

    let first = arr[0];
    let second = arr[1];
    // Compiler error if index is past the end of the array and is known at compile time
    // let third = arr[2];
    // error: this operation will panic at runtime
    //   --> src/lessons/data_types.rs:94:17
    //    |
    // 94 |     let third = arr[2];
    //    |                 ^^^^^^ index out of bounds: the length is 2 but the index is 2
    //    |
    //    = note: `#[deny(unconditional_panic)]` on by default
    dbg!(first, second);

    {
        // Panic if index is past the end of the array and is not known at compile time
        println!("Please enter an array index.");
        let mut index = String::new();
        std::io::stdin()
            .read_line(&mut index)
            .expect("Failed to read line");
        let index: usize = index
            .trim()
            .parse()
            .expect("Index entered was not a number");
        let element = arr[index];
        println!("The value of the element at index {index} is: {element}");
    }

    // Floating-point types cannot be overflowed, as their range is from -inf to +inf
    let mut d: f32 = f32::MAX;
    dbg!(d);
    d += f32::MAX;
    dbg!(d);

    // Overflow will cause panic in debug and two’s complement wrapping in release
    b += 1;
    dbg!(b);

    // Essentially, "overflow-checks" flag can be set in the profile to disable checks in debug, or enable in release
    // https://doc.rust-lang.org/cargo/reference/profiles.html

    // Note, that integer overflow does not consider unsafe, it is a defined behavior
    // https://doc.rust-lang.org/reference/behavior-not-considered-unsafe.html#integer-overflow
}
