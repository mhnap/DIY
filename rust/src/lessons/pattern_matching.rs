// https://doc.rust-lang.org/stable/book/ch06-02-match.html
// https://doc.rust-lang.org/stable/book/ch06-03-if-let.html

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

        // Or we could use an if let and else expression
        let config_max: Option<u8> = None;
        if let Some(max) = config_max {
            println!("The maximum is configured to be {}", max);
        } else {
            println!("No configured maximum");
        }

        // Can use the same syntax with regular variable initialization
        let Some(value) = Some(3.5) else {panic!()};
        dbg!(value);
    }
}
