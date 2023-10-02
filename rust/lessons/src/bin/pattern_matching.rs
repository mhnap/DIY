// https://doc.rust-lang.org/book/ch06-02-match.html
// https://doc.rust-lang.org/book/ch06-03-if-let.html

fn main() {
    // Rust has an extremely powerful control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.
    // The power of match comes from the expressiveness of the patterns and the fact that the compiler confirms that all possible cases are handled.
    {
        #[derive(Debug)]
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter,
        }
        fn value_in_cents(coin: &Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter => 25,
            }
        }
        let coin = Coin::Penny;
        let value = value_in_cents(&coin);
        dbg!(&coin, &value);

        // If you want to run multiple lines of code in a match arm, you must use curly brackets, and the comma following the arm is then optional.
        {
            fn value_in_cents(coin: &Coin) -> u8 {
                match coin {
                    Coin::Penny => {
                        println!("Lucky penny!");
                        1
                    }
                    Coin::Nickel => 5,
                    Coin::Dime => 10,
                    Coin::Quarter => 25,
                }
            }
            let coin = Coin::Penny;
            let value = value_in_cents(&coin);
            dbg!(&coin, &value);
        }
    }

    //

    {
        // Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
        // This is how we can extract values out of enum variants.
        #[derive(Debug)]
        enum UsState {
            Alabama,
            Alaska,
            // --snip--
        }
        enum Coin {
            Penny,
            Nickel,
            Dime,
            Quarter(UsState),
        }
        fn value_in_cents(coin: &Coin) -> u8 {
            match coin {
                Coin::Penny => 1,
                Coin::Nickel => 5,
                Coin::Dime => 10,
                Coin::Quarter(state) => {
                    println!("State quarter from {:?}!", state);
                    25
                }
            }
        }
        let coin = Coin::Quarter(UsState::Alaska);
        value_in_cents(&coin);
    }

    //

    {
        // Instead of comparing coins, we’ll compare the variants of Option<T>, but the way the match expression works remains the same.
        fn plus_one(x: Option<i32>) -> Option<i32> {
            match x {
                None => None,
                Some(i) => Some(i + 1),
            }
        }
        let five = Some(5);
        let six = plus_one(five);
        dbg!(&six);
        let none = plus_one(None);
        dbg!(&none);
    }

    //

    {
        // There’s one other aspect of match we need to discuss: the arms’ patterns must cover all possibilities.
        // fn plus_one(x: Option<i32>) -> Option<i32> {
        //     match x {
        //         Some(i) => Some(i + 1),
        //     }
        // }
        // error[E0004]: non-exhaustive patterns: `None` not covered
        //    --> src/lessons/pattern_matching.rs:99:19
        //     |
        // 99  |             match x {
        //     |                   ^ pattern `None` not covered
        //     |
        // note: `Option<i32>` defined here
        //    --> /home/mhnap/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:567:5
        //     |
        // 563 | pub enum Option<T> {
        //     | ------------------
        // ...
        // 567 |     None,
        //     |     ^^^^ not covered
        //     = note: the matched value is of type `Option<i32>`
        // help: ensure that all possible cases are being handled by adding a match arm with a wildcard pattern or an explicit pattern as shown
        //     |
        // 100 ~                 Some(i) => Some(i + 1),
        // 101 ~                 None => todo!(),
        //     |

        // Matches in Rust are exhaustive: we must exhaust every last possibility in order for the code to be valid.
    }

    //

    {
        // Note that we have to put the catch-all arm last because the patterns are evaluated in order.
        // If we put the catch-all arm earlier, the other arms would never run, so Rust will warn us if we add arms after a catch-all!
        let dice_roll = 9;
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            other => move_player(other),
        }
        fn add_fancy_hat() {
            println!("add_fancy_hat")
        }
        fn remove_fancy_hat() {
            println!("remove_fancy_hat")
        }
        fn move_player(num_spaces: u8) {
            println!("move_player by {num_spaces}");
        }

        // Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern: _ is a special pattern that matches any value and does not bind to that value.
        // This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => reroll(),
        }
        fn reroll() {
            println!("reroll")
        }

        // Here, we’re telling Rust explicitly that we aren’t going to use any other value that doesn’t match a pattern in an earlier arm, and we don’t want to run any code in this case.
        match dice_roll {
            3 => add_fancy_hat(),
            7 => remove_fancy_hat(),
            _ => (),
        }
    }

    //

    {
        // We don’t want to do anything with the None value.
        // To satisfy the match expression, we have to add _ => () after processing just one variant, which is annoying boilerplate code to add.
        let config_max = Some(3u8);
        match config_max {
            Some(max) => println!("The maximum is configured to be {}", max),
            _ => (),
        }

        // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern while ignoring the rest.
        let config_max = Some(3u8);
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        }

        // Note, don't work in the opposite direction.
        let config_max = Some(3u8);
        // if let config_max = Some(max) {
        //     println!("The maximum is configured to be {}", max);
        // }
        // error[E0425]: cannot find value `max` in this scope
        //    --> src/lessons/pattern_matching.rs:187:34
        //     |
        // 187 |         if let config_max = Some(max) {
        //     |                                  ^^^ not found in this scope
        //     |
        // help: consider importing one of these items
        //     |
        // 4   | use core::cmp::max;
        //     |
        // 4   | use std::cmp::max;
        //     |

        // Or we could use an if let and else expression
        let config_max: Option<u8> = None;
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        } else {
            println!("No configured maximum");
        }

        // Can use the same syntax with regular variable initialization
        let Some(value) = Some(3.5) else { panic!() };
        dbg!(value);
    }

    //

    // https://doc.rust-lang.org/book/ch18-00-patterns.html

    // To use a pattern, we compare it to some value.
    // If the pattern matches the value, we use the value parts in our code.
    // If the value fits the shape of the pattern, we can use the named pieces.
    // If it doesn’t, the code associated with the pattern won’t run.

    //

    // https://doc.rust-lang.org/book/ch18-01-all-the-places-for-patterns.html

    // match Arms
    let x = Some(1);
    let y = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    dbg!(x, y);

    // One requirement for match expressions is that they need to be exhaustive in the sense that all possibilities for the value in the match expression must be accounted for.

    //

    // Conditional if let Expressions
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // The downside of using if let expressions is that the compiler doesn’t check for exhaustiveness, whereas with match expressions it does.
    // If we omitted the last else block and therefore missed handling some cases, the compiler would not alert us to the possible logic bug.

    //

    // while let Conditional Loops
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    //

    // for Loops
    let v = vec!['a', 'b', 'c'];
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    //

    // let Statements
    let (x, y, z) = (1, 2, 3);

    // More formally, a let statement looks like this: let PATTERN = EXPRESSION;

    //

    // Function Parameters
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }
    let point = (3, 5);
    print_coordinates(&point);

    // We can also use patterns in closure parameter lists in the same way as in function parameter lists, because closures are similar to functions.

    //

    // https://doc.rust-lang.org/book/ch18-02-refutability.html

    // Patterns come in two forms: refutable and irrefutable.

    // Patterns that will match for any possible value passed are irrefutable.
    // An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match.

    // Patterns that can fail to match for some possible value are refutable.
    // An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

    // Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match.
    // The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.

    // Let’s look at an example of what happens when we try to use a refutable pattern where Rust requires an irrefutable pattern and vice versa.

    // let Some(x) = Some(1);
    // error[E0005]: refutable pattern in local binding
    //    --> src/lessons/pattern_matching.rs:314:9
    //     |
    // 314 |     let Some(x) = Some(1);
    //     |         ^^^^^^^ pattern `None` not covered
    //     |
    //     = note: `let` bindings require an "irrefutable pattern", like a `struct` or an `enum` with only one variant
    //     = note: for more information, visit https://doc.rust-lang.org/book/ch18-02-refutability.html
    //     = note: the matched value is of type `Option<i32>`
    // help: you might want to use `let else` to handle the variant that isn't matched
    //     |
    // 314 |     let Some(x) = Some(1) else { todo!() };
    //     |                           ++++++++++++++++

    // Note that a refutable pattern in local binding can be handled with an `else` branch.
    let opt_x = Some(1);
    let Some(x) = opt_x else { panic!() };
    dbg!(x);

    // This is the same logic but longer code as above.
    let x = if let Some(x) = opt_x { x } else { panic!() };
    dbg!(x);

    if let x = 5 {
        println!("{}", x);
    };
    // warning: irrefutable `if let` pattern
    //    --> src/lessons/pattern_matching.rs:331:8
    //     |
    // 331 |     if let x = 5 {
    //     |        ^^^^^^^^^
    //     |
    //     = note: this pattern will always match, so the `if let` is useless
    //     = help: consider replacing the `if let` with a `let`

    // For this reason, match arms must use refutable patterns, except for the last arm, which should match any remaining values with an irrefutable pattern.
    // Rust allows us to use an irrefutable pattern in a match with only one arm, but this syntax isn’t particularly useful and could be replaced with a simpler let statement.

    //

    // https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html

    // Matching Literals
    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // This syntax is useful when you want your code to take an action if it gets a particular concrete value.

    //

    // Matching Named Variables
    let x = Some(5);
    let y = 10;
    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }
    println!("at the end: x = {:?}, y = {y}", x);

    //

    // Multiple Patterns
    let x = 1;
    match x {
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    //

    // Matching Ranges of Values with ..=
    let x = 'c';
    match x {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    // The compiler checks that the range isn’t empty at compile time, and because the only types for which Rust can tell if a range is empty or not are char and numeric values, ranges are only allowed with numeric or char values.

    //

    // Destructuring Structs
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    let Point { x: a, y: b } = p;
    dbg!(a, b);

    // Won't compile without all fields.
    // let Point { x: a } = p;
    // error[E0027]: pattern does not mention field `y`
    //    --> src/lessons/pattern_matching.rs:407:9
    //     |
    // 407 |     let Point { x: a } = p;
    //     |         ^^^^^^^^^^^^^^ missing field `y`
    //     |
    // help: include the missing field in the pattern
    //     |
    // 407 |     let Point { x: a, y } = p;
    //     |                     ~~~~~
    // help: if you don't care about this missing field, you can explicitly ignore it
    //     |
    // 407 |     let Point { x: a, .. } = p;
    //     |                     ~~~~~~

    // Rust has a shorthand for patterns that match struct fields: you only need to list the name of the struct field, and the variables created from the pattern will have the same names.
    let Point { x, y } = p;
    dbg!(x, y);

    // We can also destructure with literal values as part of the struct pattern rather than creating variables for all the fields.
    // Doing so allows us to test some of the fields for particular values while creating variables to destructure the other fields.
    let p = Point { x: 0, y: 7 };
    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }

    //

    // Destructuring Enums
    {
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        let msg = Message::ChangeColor(0, 160, 255);
        match msg {
            Message::Quit => {
                println!("The Quit variant has no data to destructure.");
            }
            Message::Move { x, y } => {
                println!("Move in the x direction {x} and in the y direction {y}");
            }
            Message::Write(text) => {
                println!("Text message: {text}");
            }
            Message::ChangeColor(r, g, b) => {
                println!("Change the color to red {r}, green {g}, and blue {b}",)
            }
        }
    }

    //

    // Destructuring Nested Structs and Enums
    {
        enum Color {
            Rgb(i32, i32, i32),
            Hsv(i32, i32, i32),
        }
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(Color),
        }
        let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));

        match msg {
            Message::ChangeColor(Color::Rgb(r, g, b)) => {
                println!("Change color to red {r}, green {g}, and blue {b}");
            }
            Message::ChangeColor(Color::Hsv(h, s, v)) => {
                println!("Change color to hue {h}, saturation {s}, value {v}")
            }
            _ => (),
        }
    }

    //

    // Destructuring Structs and Tuples
    let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });

    // Destructuring with patterns is a convenient way to use pieces of values, such as the value from each field in a struct, separately from each other.

    //

    // Ignoring an Entire Value with _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    // In most cases when you no longer need a particular function parameter, you would change the signature so it doesn’t include the unused parameter.
    // Ignoring a function parameter can be especially useful in cases when, for example, you're implementing a trait when you need a certain type signature but the function body in your implementation doesn’t need one of the parameters.
    // You then avoid getting a compiler warning about unused function parameters, as you would if you used a name instead.

    //

    // Ignoring Parts of a Value with a Nested _
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    println!("setting is {:?}", setting_value);

    // We can also use underscores in multiple places within one pattern to ignore particular values.
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    //

    // Ignoring an Unused Variable by Starting Its Name with _
    let _x = 5;
    let y = 10;
    // Here we get a warning about not using the variable y, but we don’t get a warning about not using _x.

    // Note that there is a subtle difference between using only _ and using a name that starts with an underscore.
    // The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
    let s = Some(String::from("Hello!"));
    // if let Some(_s) = s {
    //     println!("found a string");
    // }
    // println!("{:?}", s);
    // error[E0382]: borrow of partially moved value: `s`
    //    --> src/lessons/pattern_matching.rs:547:22
    //     |
    // 544 |     if let Some(_s) = s {
    //     |                 -- value partially moved here
    // ...
    // 547 |     println!("{:?}", s);
    //     |                      ^ value borrowed here after partial move
    //     |
    //     = note: partial move occurs because value has type `String`, which does not implement the `Copy` trait
    //     = note: this error originates in the macro `$crate::format_args_nl` which comes from the expansion of the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
    // help: borrow this binding in the pattern to avoid moving the value
    //     |
    // 544 |     if let Some(ref _s) = s {
    //     |                 +++

    // This code works just fine because we never bind s to anything; it isn’t moved.
    if let Some(_) = s {
        println!("found a string");
    }
    println!("{:?}", s);

    //

    // Ignoring Remaining Parts of a Value with ..
    let origin = Point { x: 0, y: 0 };
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    // In this code, the first and last value are matched with first and last.
    // The .. will match and ignore everything in the middle.
    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }

    // An attempt to use .. in an ambiguous way.
    let numbers = (2, 4, 8, 16, 32);
    // match numbers {
    //     (.., second, ..) => {
    //         println!("Some numbers: {}", second)
    //     },
    // }
    // error: `..` can only be used once per tuple pattern
    //    --> src/lessons/pattern_matching.rs:590:22
    //     |
    // 590 |         (.., second, ..) => {
    //     |          --          ^^ can only be used once per tuple pattern
    //     |          |
    //     |          previously used here

    //

    // Extra Conditionals with Match Guards
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    // A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen.
    // Match guards are useful for expressing more complex ideas than a pattern alone allows.

    //

    // @ Bindings
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }

    // The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
}
