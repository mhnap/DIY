use std::mem::size_of;

fn main() {
    {
        // Need to explicitly create reference
        let a: String = "42".to_string();
        // let b: &String = a;
        // error[E0308]: mismatched types
        //  --> src/cpp_comparison/references.rs:5:26
        //   |
        // 5 |         let b: &String = a;
        //   |                -------   ^
        //   |                |         |
        //   |                |         expected `&String`, found struct `String`
        //   |                |         help: consider borrowing here: `&a`
        //   |                expected due to this
        let b: &String = &a;
        println!("a:{a}; b:{b}");
    }

    {
        // It IS possible to change what a reference refers to after initialization
        let a: String = "42".to_string();
        let mut b: &String = &a;
        let c: String = "41".to_string();
        b = &c;
        println!("a:{a}; b:{b}; c:{c}");
    }

    {
        // Can create immutable reference what refer to mutable data
        let mut a: i32 = 42;
        let b: &i32 = &a;
        let c: i32 = 41;
        // *b = c;
        // error[E0594]: cannot assign to `*b`, which is behind a `&` reference
        //   --> src/cpp_comparison/references.rs:33:9
        //    |
        // 33 |         *b = c;
        //    |         ^^^^^^ `b` is a `&` reference, so the data it refers to cannot be written
        //    |
        // help: consider changing this to be a mutable reference
        //    |
        // 31 |         let b: &i32 = &mut a;
        //    |                       ~~~~~~
        println!("a:{a}; b:{b}; c:{c}");
    }

    {
        // Cannot create mutable reference what refer to immutable data
        let a: i32 = 42;
        // let b: &mut i32 = &mut a;
        // error[E0596]: cannot borrow `a` as mutable, as it is not declared as mutable
        //   --> src/cpp_comparison/references.rs:50:27
        //    |
        // 50 |         let b: &mut i32 = &mut a;
        //    |                           ^^^^^^ cannot borrow as mutable
        //    |
        // help: consider changing this to be mutable
        //    |
        // 49 |         let mut a: i32 = 42;
        //    |             +++
        println!("a:{a};");
    }

    {
        // Can have vector with references
        let a: i32 = 42;
        let mut vec: Vec<&i32> = Vec::new();
        vec.push(&a);
        println!("a:{};", vec[0]);
    }

    {
        // Need to dereference a reference to change referred value
        let mut a: i32 = 42;
        {
            let b: &mut i32 = &mut a;
            *b = 41;
        }
        println!("a:{a};");
    }

    {
        // Can assign struct with reference member
        #[derive(Copy, Clone)]
        struct S<'a> {
            r: &'a i32,
        }

        let a: i32 = 42;
        let b: i32 = 41;
        let mut sa: S = S { r: &a };
        let sb: S = S { r: &b };
        sa = sb;
        println!("sa:{}; sb:{}", sa.r, sb.r);
    }

    {
        println!(
            "sizeof bool: {}; sizeof &bool: {}",
            size_of::<bool>(),
            size_of::<&bool>()
        );
    }
}

// Differences:
// - can change what reference refers
// - can have vector with references
// - need to dereference a reference to change referred value
// - can assign struct with reference member
//
// Similarities:
// - can create immutable reference what refer to mutable data
// - cannot create mutable reference what refer to immutable data
//
// Pros:
// - need to explicitly create reference
// - reference has own distinct type
//
// Notes:
// - Rust references are more like std::reference_wrapper
