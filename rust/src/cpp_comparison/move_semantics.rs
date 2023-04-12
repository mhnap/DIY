fn get_vec_with_string(str: String) -> Vec<String> {
    let mut vec: Vec<String> = vec![];
    vec.push(str);
    vec
}

fn main() {
    {
        let a: i32 = 42;
        // Copy
        let b: i32 = a.clone();
        println!("a:{a}; b:{b}");
    }

    {
        let a: i32 = 42;
        // Move here is a copy because i32 implement Copy trait
        let b: i32 = a;
        println!("a:{a}; b:{b}");
    }

    {
        let a: String = "42".to_string();
        // Copy (deep copy)
        let b: String = a.clone();
        println!("a:{a}; b:{b}");
    }

    {
        let a: String = "42".to_string();
        // Move (memcpy)
        let b: String = a;
        // println!("a:{a}; b:{b}");
        // error[E0382]: borrow of moved value: `a`
        //   --> src/cpp_comparison/move_semantics.rs:27:22
        //    |
        // 24 |         let a: String = 42.to_string();
        //    |             - move occurs because `a` has type `String`, which does not implement the `Copy` trait
        // 25 |         // Move
        // 26 |         let b: String = a;
        //    |                         - value moved here
        // 27 |         println!("a:{a}; b:{b}");
        //    |                      ^ value borrowed here after move
        //    |
        //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        // help: consider cloning the value if the performance cost is acceptable
        //    |
        // 26 |         let b: String = a.clone();
        //    |                          ++++++++
        println!("a:moved; b:{b}");
    }

    {
        // Move works also for immutable objects
        let str: String = "42".to_string();
        let vec: Vec<String> = get_vec_with_string(str);
        println!("vec:{}", vec[0]);
        // println!("str:{}", str);
        // error[E0382]: borrow of moved value: `str`
        //   --> src/cpp_comparison/move_semantics.rs:57:28
        //    |
        // 54 |         let str: String = "42".to_string();
        //    |             --- move occurs because `str` has type `String`, which does not implement the `Copy` trait
        // 55 |         let vec: Vec<String> = get_vec_with_string(str);
        //    |                                                    --- value moved here
        // 56 |         println!("vec:{}", vec[0]);
        // 57 |         println!("str:{}", str);
        //    |                            ^^^ value borrowed here after move
        //    |
        // note: consider changing this parameter type in function `get_vec_with_string` to borrow instead if owning the value isn't necessary
        //   --> src/cpp_comparison/move_semantics.rs:1:29
        //    |
        // 1  | fn get_vec_with_string(str: String) -> Vec<String> {
        //    |    -------------------      ^^^^^^ this parameter takes ownership of the value
        //    |    |
        //    |    in this function
        //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        // help: consider cloning the value if the performance cost is acceptable
        //    |
        // 55 |         let vec: Vec<String> = get_vec_with_string(str.clone());
        //    |                                                       ++++++++
        println!("str:moved");
    }

    {
        // Object cannot be moved using reference
        let str: String = "42".to_string();
        let str_ref: &String = &str;
        // let new_str: String = *str_ref;
        // error[E0507]: cannot move out of `*str_ref` which is behind a shared reference
        //   --> src/cpp_comparison/move_semantics.rs:89:31
        //    |
        // 89 |         let new_str: String = *str_ref;
        //    |                               ^^^^^^^^ move occurs because `*str_ref` has type `String`, which does not implement the `Copy` trait
        //    |
        // help: consider removing the dereference here
        //    |
        // 89 -         let new_str: String = *str_ref;
        // 89 +         let new_str: String = str_ref;
        //    |
        let new_str: String = str_ref.clone();
        println!("str:{str}");
        println!("str_ref:{str_ref}");
        println!("new_str:{new_str}");
    }

    {
        let a: Box<String> = Box::new(String::from("42"));
        // Copy (deep copy)
        // Rust use move by default, so it was save to implement Box as copyable because explicit clone call is needed
        let b: Box<String> = a.clone();
        println!("a:{a}; b:{b}");
    }

    {
        let a: Box<String> = Box::new(String::from("42"));
        // Move (memcpy)
        let b: Box<String> = a;
        // println!("a:{a}; b:{b}");
        // error[E0382]: borrow of moved value: `a`
        //   --> src/cpp_comparison/move_semantics.rs:96:22
        //    |
        // 93 |         let a: Box<String> = Box::new(String::from("42"));
        //    |             - move occurs because `a` has type `Box<String>`, which does not implement the `Copy` trait
        // 94 |         // Move (memcpy)
        // 95 |         let b: Box<String> = a;
        //    |                              - value moved here
        // 96 |         println!("a:{a}; b:{b}");
        //    |                      ^ value borrowed here after move
        //    |
        //    = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
        // help: consider cloning the value if the performance cost is acceptable
        //    |
        // 95 |         let b: Box<String> = a.clone();
        //    |                               ++++++++
        println!("a:moved; b:{b}");
    }
}

// Pros:
// - move works as simple memcpy, thus no need for customs functions (move ctor, assigment, etc.)
//                                thus simpler logic (no need to use rvalue cast)
// - implicit move by default, thus no need to write moves manually everywhere
//                             thus better performance is also by default
//                             thus more cleaner parameter passing
// - destructive move, thus no destructor calls
//                     thus no need to handle empty but valid states for types
//                     thus cannot use moved-from object
// - move works also for immutable objects
// - object cannot be moved using reference
// - explicit "clone" call needed to make deep copies, thus more cleaner intentions
