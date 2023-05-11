// https://doc.rust-lang.org/stable/book/ch04-02-references-and-borrowing.html

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
        println!("a:{a}");
    }

    {
        // Can have vector with references
        let a: i32 = 42;
        let mut vec: Vec<&i32> = Vec::new();
        vec.push(&a);
        println!("a:{}", vec[0]);
    }

    {
        // Need to dereference a reference to change referred value
        let mut a: i32 = 42;
        {
            let b: &mut i32 = &mut a;
            *b = 41;
        }
        println!("a:{a}");
    }

    {
        // No need to explicitly dereference a reference to use object methods
        let s: String = "42".to_string();
        let r: &String = &s;
        println!(
            "s:{}; r:{}",
            s.chars().next().unwrap(),
            r.chars().next().unwrap()
        );
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
        // Reference can be uninitialized
        let b: &i32;
        let a: i32 = 42;
        // But cannot be read if its uninitialized
        // println!("a:{a}; b:{b}");
        // error[E0381]: used binding `b` isn't initialized
        //    --> src/cpp_comparison/references.rs:115:29
        //     |
        // 112 |         let b: &i32;
        //     |             - binding declared here but left uninitialized
        // ...
        // 115 |         println!("a:{a}; b:{b}");
        //     |                             ^ `b` used here but it isn't initialized
        //     |
        //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        // help: consider assigning a value
        //     |
        // 112 |         let b: &i32 = todo!();
        //     |                     +++++++++
        b = &a;
        println!("a:{a}; b:{b}");
    }

    {
        // Borrowing rules
        let mut s = String::from("hello");
        {
            // We call the action of creating a reference borrowing.
            let r1 = &s;
            // Can have two immutable references
            let r2 = &s;
            // Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.
            // let r3 = &mut s;
            // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
            //    --> src/cpp_comparison/references.rs:143:22
            //     |
            // 138 |             let r1 = &s;
            //     |                      -- immutable borrow occurs here
            // ...
            // 143 |             let r3 = &mut s;
            //     |                      ^^^^^^ mutable borrow occurs here
            // ...
            // 155 |             println!("r1:{r1}; r2:{r2}; r3:{r3}");
            //     |                           -- immutable borrow later used here
            // println!("r1:{r1}; r2:{r2}; r3:{r3}");
        }
        // Can borrow as mutable because no other references
        let r1 = &mut s;
        // let r2 = &mut s;
        // error[E0499]: cannot borrow `s` as mutable more than once at a time
        //    --> src/cpp_comparison/references.rs:158:18
        //     |
        // 157 |         let r1 = &mut s;
        //     |                  ------ first mutable borrow occurs here
        // 158 |         let r2 = &mut s;
        //     |                  ^^^^^^ second mutable borrow occurs here
        // 159 |         println!("r1:{r1}; r2:{r2}");
        //     |                       -- first borrow later used here
        // println!("r1:{r1}; r2:{r2}");

        // Note that a reference’s scope starts from where it is introduced and continues through the last time that reference is used.
        let r1 = &s; // no problem
        let r2 = &s; // no problem
        println!("r1:{r1}; r2:{r2}");
        // variables r1 and r2 will not be used after this point

        let r3 = &mut s; // no problem
        println!("r3:{r3}");

        // Even though borrowing errors may be frustrating at times, remember that it’s the Rust compiler pointing out a potential bug early (at compile time rather than at runtime) and showing you exactly where the problem is.
        // Then you don’t have to track down why your data isn’t what you thought it was.
    }

    {
        // Cannot create dangling reference from value that does not live long enough
        let s1 = String::from("hello");
        let mut r1 = &s1;
        {
            let s2 = String::from("hello");
            // r1 = &s2;
            // error[E0597]: `s2` does not live long enough
            //    --> src/cpp_comparison/references.rs:188:18
            //     |
            // 187 |             let s2 = String::from("hello");
            //     |                 -- binding `s2` declared here
            // 188 |             r1 = &s2;
            //     |                  ^^^ borrowed value does not live long enough
            // 189 |         }
            //     |         - `s2` dropped here while still borrowed
            // 190 |         println!("r1:{r1}");
            //     |                       -- borrow later used here
        }
        println!("r1:{r1}");

        // Cannot create dangling reference from function local data
        // let reference_to_nothing = dangle();
        // fn dangle() -> &String {
        //     let s = String::from("hello");
        //     &s
        // }
        // error[E0106]: missing lifetime specifier
        //    --> src/cpp_comparison/references.rs:206:24
        //     |
        // 206 |         fn dangle() -> &String {
        //     |                        ^ expected named lifetime parameter
        //     |
        //     = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
        // help: consider using the `'static` lifetime
        //     |
        // 206 |         fn dangle() -> &'static String {
        //     |                         +++++++

        // Move ownership out from a function to s
        let s = no_dangle();
        fn no_dangle() -> String {
            let s = String::from("hello");
            s
        }
    }

    {
        // Cannot change the owned value if there is still a valid reference to it
        // https://rust-lang.github.io/rfcs/2094-nll.html
        let mut x = 42;
        let r = &x;
        // x = 0;
        // error[E0506]: cannot assign to `x` because it is borrowed
        //    --> src/cpp_comparison/references.rs:238:9
        //     |
        // 237 |         let r = &x;
        //     |                 -- `x` is borrowed here
        // 238 |         x = 0;
        //     |         ^^^^^ `x` is assigned to here but it was already borrowed
        // 239 |         println!("r:{r}");
        //     |                      - borrow later used here
        println!("r:{r}");

        // This example makes even more sense
        let mut v = vec![1, 2];
        let r = &v[0];
        // v = vec![1, 2, 3, 4];
        // error[E0506]: cannot assign to `v` because it is borrowed
        //    --> src/cpp_comparison/references.rs:253:9
        //     |
        // 252 |         let r = &v[0];
        //     |                  - `v` is borrowed here
        // 253 |         v = vec![1, 2, 3, 4];
        //     |         ^ `v` is assigned to here but it was already borrowed
        // 254 |         println!("r:{r}");
        //     |                      - borrow later used here
        println!("r:{r}");
    }

    {
        println!(
            "sizeof bool:{}; sizeof &bool:{}",
            size_of::<bool>(),
            size_of::<&bool>()
        );
    }
}

// Differences:
// - references can be declared uninitialized, but cannot be used uninitialized
// - can change what reference refers
// - can have vector with references
// - need to dereference a reference to change referred value
// - can assign struct with reference member
// - guarantees at compile time that no data race (by checking borrowing rules)
// - guarantees at compile time that no dangling references
//
// Similarities:
// - references are non-nullable
// - can create immutable reference what refer to mutable data
// - cannot create mutable reference what refer to immutable data
// - references can be implicitly dereferenced
//
// Pros:
// - need to explicitly create reference
// - reference has own distinct type
// - there is a borrow checker, thus references cannot be dangling
//
// Notes:
// - Rust references are more like std::reference_wrapper
