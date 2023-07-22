use std::thread;
use std::time::Duration;

fn main() {
    // https://doc.rust-lang.org/book/ch13-00-functional-features.html

    // Rust’s design has taken inspiration from many existing languages and techniques, and one significant influence is functional programming.
    // Programming in a functional style often includes using functions as values by passing them in arguments, returning them from other functions, assigning them to variables for later execution, and so forth.
    // Mastering closures and iterators is an important part of writing idiomatic, fast Rust code.

    //

    // https://doc.rust-lang.org/book/ch13-01-closures.html

    // Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions.
    // You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context.
    // Unlike functions, closures can capture values from the scope in which they’re defined.
    // We’ll demonstrate how these closure features allow for code reuse and behavior customization.

    {
        // Regular function cannot capture the environment.
        let a = 1;
        // fn foo() {
        //     dbg!(a);
        // }
        // foo();
        // error[E0434]: can't capture dynamic environment in a fn item
        //   --> src/lessons/iterators_and_closures.rs:21:18
        //    |
        // 21 |             dbg!(a);
        //    |                  ^
        //    |
        //    = help: use the `|| { ... }` closure form instead

        // Closure can capture the environment.
        let foo = || {
            dbg!(a);
        };
        foo();
    }

    {
        // We’ll first examine how we can use closures to capture values from the environment they’re defined in for later use.

        #[derive(Debug, PartialEq, Copy, Clone)]
        enum ShirtColor {
            Red,
            Blue,
        }

        struct Inventory {
            shirts: Vec<ShirtColor>,
        }

        impl Inventory {
            fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
                user_preference.unwrap_or_else(|| self.most_stocked())
            }

            fn most_stocked(&self) -> ShirtColor {
                let mut num_red = 0;
                let mut num_blue = 0;

                for color in &self.shirts {
                    match color {
                        ShirtColor::Red => num_red += 1,
                        ShirtColor::Blue => num_blue += 1,
                    }
                }
                if num_red > num_blue {
                    ShirtColor::Red
                } else {
                    ShirtColor::Blue
                }
            }
        }

        let store = Inventory {
            shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
        };

        let user_pref1 = Some(ShirtColor::Red);
        let giveaway1 = store.giveaway(user_pref1);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref1, giveaway1
        );

        let user_pref2 = None;
        let giveaway2 = store.giveaway(user_pref2);
        println!(
            "The user with preference {:?} gets {:?}",
            user_pref2, giveaway2
        );

        // One interesting aspect here is that we’ve passed a closure that calls self.most_stocked() on the current Inventory instance.
        // The standard library didn’t need to know anything about the Inventory or ShirtColor types we defined, or the logic we want to use in this scenario.
        // The closure captures an immutable reference to the self Inventory instance and passes it with the code we specify to the unwrap_or_else method.
        // Functions, on the other hand, are not able to capture their environment in this way.
    }

    {
        // Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
        // Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
        // Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.
        // Closures, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

        // Implicit types, inferred by the compiler.
        let expensive_closure = |num| {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(num));
            num
        };
        dbg!(expensive_closure(1));

        // Explicit types, manually annotated.
        let expensive_closure = |num: u64| -> u64 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(num));
            num
        };
        dbg!(expensive_closure(1));

        // With type annotations added, the syntax of closures looks more similar to the syntax of functions.
        // This illustrates how closure syntax is similar to function syntax except for the use of pipes and the amount of syntax that is optional:
        // fn  add_one_v1   (x: u32) -> u32 { x + 1 }
        // let add_one_v2 = |x: u32| -> u32 { x + 1 };
        // let add_one_v3 = |x|             { x + 1 };
        // let add_one_v4 = |x|               x + 1  ;

        // For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value.
        // But note that this inference is done only for the first usage, it's not generics.
        let example_closure = |x| x;
        let s = example_closure(String::from("hello"));
        // let n = example_closure(5);
        // error[E0308]: mismatched types
        //    --> src/lessons/iterators_and_closures.rs:135:33
        //     |
        // 135 |         let n = example_closure(5);
        //     |                 --------------- ^- help: try using a conversion method: `.to_string()`
        //     |                 |               |
        //     |                 |               expected `String`, found integer
        //     |                 arguments to this function are incorrect
        //     |
        // note: closure parameter defined here
        //    --> src/lessons/iterators_and_closures.rs:133:32
        //     |
        // 133 |         let example_closure = |x| x;
        //     |                                ^
    }

    {
        // Note that we can coerce closure to fn with type annotation manually.
        let expensive_closure: fn(u64) -> u64 = |num: u64| -> u64 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(num));
            num
        };
        dbg!(expensive_closure(1));

        // But we cannot coerce closure to fn if it captures any variables.
        let num = 1;
        // let expensive_closure: fn() -> u64 = || -> u64 {
        //     println!("calculating slowly...");
        //     thread::sleep(Duration::from_secs(num));
        //     num
        // };
        // error[E0308]: mismatched types
        //    --> src/lessons/iterators_and_closures.rs:145:46
        //     |
        // 145 |           let expensive_closure: fn() -> u64 = || -> u64 {
        //     |  ________________________________-----------___^
        //     | |                                |
        //     | |                                expected due to this
        // 146 | |             println!("calculating slowly...");
        // 147 | |             thread::sleep(Duration::from_secs(num));
        // 148 | |             num
        // 149 | |         };
        //     | |_________^ expected fn pointer, found closure
        //     |
        //     = note: expected fn pointer `fn() -> u64`
        //                   found closure `[closure@src/lessons/iterators_and_closures.rs:145:46: 145:55]`
        // note: closures can only be coerced to `fn` types if they do not capture any variables
        //    --> src/lessons/iterators_and_closures.rs:147:47
        //     |
        // 147 |             thread::sleep(Duration::from_secs(num));
        //     |                                               ^^^ `num` captured here
        let expensive_closure = || -> u64 {
            println!("calculating slowly...");
            thread::sleep(Duration::from_secs(num));
            num
        };
        dbg!(expensive_closure());
    }

    {
        // Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership.
        // The closure will decide which of these to use based on what the body of the function does with the captured values.

        // Borrowing immutably.
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let only_borrows = || println!("From closure: {:?}", list);
        println!("Before calling closure: {:?}", list);
        only_borrows();
        println!("After calling closure: {:?}", list);
        // Because we can have multiple immutable references to list at the same time, list is still accessible from the code before the closure definition, after the closure definition but before the closure is called, and after the closure is called.

        // Borrowing mutably.
        let mut list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let mut borrows_mutably = || {
            println!("From closure: {:?}", list);
            list.push(7)
        };
        // Between the closure definition and the closure call, an immutable borrow to print isn’t allowed because no other borrows are allowed when there’s a mutable borrow.
        // println!("Before calling closure: {:?}", list);
        borrows_mutably();
        // We don’t use the closure again after the closure is called, so the mutable borrow ends.
        println!("After calling closure: {:?}", list);

        // Taking ownership.
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);
        let take_ownership = || {
            println!("From closure: {:?}", list);
            drop(list)
        };
        // Between the closure definition and the closure call, an immutable borrow to print isn’t allowed because the value is already moved into closure.
        // println!("Before calling closure: {:?}", list);
        take_ownership();
        // After the closure call, an immutable borrow to print also isn’t allowed because the value is already moved into closure.
        // println!("After calling closure: {:?}", list);
    }

    {
        // It is also possible to force the closure to take ownership of the values it uses in the environment even though the body of the closure doesn’t strictly need ownership, by using the move keyword before the parameter list.
        // This technique is mostly useful when passing a closure to a new thread to move the data so that it’s owned by the new thread.
        let list = vec![1, 2, 3];
        println!("Before defining closure: {:?}", list);

        // The compiler requires that list be moved into the closure given to the new thread so the reference will be valid.
        // thread::spawn(|| println!("From thread: {:?}", list))
        //     .join()
        //     .unwrap();
        // error[E0373]: closure may outlive the current function, but it borrows `list`, which is owned by the current function
        //    --> src/lessons/iterators_and_closures.rs:243:23
        //     |
        // 243 |         thread::spawn(|| println!("From thread: {:?}", list))
        //     |                       ^^                               ---- `list` is borrowed here
        //     |                       |
        //     |                       may outlive borrowed value `list`
        //     |
        // note: function requires argument type to outlive `'static`
        //    --> src/lessons/iterators_and_closures.rs:243:9
        //     |
        // 243 |         thread::spawn(|| println!("From thread: {:?}", list))
        //     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
        // help: to force the closure to take ownership of `list` (and any other referenced variables), use the `move` keyword
        //     |
        // 243 |         thread::spawn(move || println!("From thread: {:?}", list))
        //     |                       ++++
        thread::spawn(move || println!("From thread: {:?}", list))
            .join()
            .unwrap();

        // After the closure definition, an immutable borrow to print isn’t allowed because the value is already moved into closure.
        // println!("Before defining closure: {:?}", list);
    }

    {
        // The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use.
        // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
        //
        // 1. `FnOnce` applies to closures that can be called once.
        // All closures implement at least this trait, because all closures can be called.
        // A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
        //
        // 2. `FnMut` applies to closures that don’t move captured values out of their body, but that might mutate the captured values.
        // These closures can be called more than once.
        //
        // 3. `Fn` applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment.
        // These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

        // FnOnce case.
        let a1 = "hi".to_string();
        let a1_fnonce = || {
            dbg!(&a1);
            a1;
        };
        a1_fnonce();
        // a1_fnonce(); // error[E0382]: use of moved value: `a1_fnonce`

        // FnMut case.
        let mut a2 = "hi".to_string();
        let mut a2_fnmut = || {
            a2.push_str(" world");
            dbg!(&a2);
        };
        a2_fnmut();
        a2_fnmut();

        // Fn case.
        let a3 = "hi".to_string();
        let a3_fn = || {
            dbg!(&a3);
        };
        a3_fn();
        a3_fn();

        //

        // Note, that need to add trait bound to mark generic as function.
        // fn call_f<F>(f: F) {
        //     f();
        // }
        // error[E0618]: expected function, found `F`
        //    --> src/lessons/iterators_and_closures.rs:315:13
        //     |
        // 314 |         fn call_f<F>(f: F) {
        //     |                      - `f` has type `F`
        // 315 |             f();
        //     |             ^--
        //     |             |
        //     |             call expression requires function
        fn call_f<F>(f: F)
        where
            F: Fn(),
        {
            f();
        }

        let dummy = || {};
        call_f(dummy);

        // Cannot be called on FnMut or FnOnce.
        let mut string = "h".to_string();
        let fn_mut = || string.push('i');

        // call_f(fn_mut);
        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnMut`
        //    --> src/lessons/iterators_and_closures.rs:338:22
        //     |
        // 338 |         let fn_mut = || string.push('i');
        //     |                      ^^ ------ closure is `FnMut` because it mutates the variable `string` here
        //     |                      |
        //     |                      this closure implements `FnMut`, not `Fn`
        // 339 |         call_f(fn_mut);
        //     |         ------ ------ the requirement to implement `Fn` derives from here
        //     |         |
        //     |         required by a bound introduced by this call
        //     |
        // note: required by a bound in `call_f`
        //    --> src/lessons/iterators_and_closures.rs:328:16
        //     |
        // 326 |         fn call_f<F>(f: F)
        //     |            ------ required by a bound in this function
        // 327 |         where
        // 328 |             F: Fn(),
        //     |                ^^^^ required by this bound in `call_f`

        let fn_once = || drop(string);
        // call_f(fn_once);
        // error[E0525]: expected a closure that implements the `Fn` trait, but this closure only implements `FnOnce`
        //    --> src/lessons/iterators_and_closures.rs:362:23
        //     |
        // 362 |         let fn_once = || drop(string);
        //     |                       ^^      ------ closure is `FnOnce` because it moves the variable `string` out of its environment
        //     |                       |
        //     |                       this closure implements `FnOnce`, not `Fn`
        // 363 |         call_f(fn_once);
        //     |         ------ ------- the requirement to implement `Fn` derives from here
        //     |         |
        //     |         required by a bound introduced by this call
        //     |
        // note: required by a bound in `call_f`
        //    --> src/lessons/iterators_and_closures.rs:328:16
        //     |
        // 326 |         fn call_f<F>(f: F)
        //     |            ------ required by a bound in this function
        // 327 |         where
        // 328 |             F: Fn(),
        //     |                ^^^^ required by this bound in `call_f`
    }

    {
        fn check_fn<F: Fn()>(f: F) {}

        fn check_fnmut<F: FnMut()>(f: F) {}

        fn check_fnonce<F: FnOnce()>(f: F) {}

        struct A(i32);
        let mut a = A(1);

        check_fn(|| {
            &a;
            // &mut a; // error[E0596]: cannot borrow `a` as mutable, as it is a captured variable in a `Fn` closure
            // a; // error[E0507]: cannot move out of `a`, a captured variable in an `Fn` closure
        });
        check_fnmut(|| {
            &a;
            &mut a;
            // a; // error[E0507]: cannot move out of `a`, a captured variable in an `FnMut` closure
        });
        check_fnonce(|| {
            &a;
            &mut a;
            a;
        });
    }
}
