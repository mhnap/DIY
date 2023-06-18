// https://doc.rust-lang.org/book/ch10-02-traits.html

use std::fmt::{Debug, Display};

fn main() {
    // A trait defines functionality a particular type has and can share with other types.
    // We can use traits to define shared behavior in an abstract way.
    // We can use trait bounds to specify that a generic type can be any type that has certain behavior.
    // Note: Traits are similar to a feature often called interfaces in other languages, although with some differences.

    {
        // A type’s behavior consists of the methods we can call on that type.
        // Different types share the same behavior if we can call the same methods on all of those types.
        // Trait definitions are a way to group method signatures together to define a set of behaviors necessary to accomplish some purpose.

        trait Summary {
            fn summarize(&self) -> String;
        }

        // Each type implementing this trait must provide its own custom behavior for the body of the method.
        // The compiler will enforce that any type that has the `Summary` trait will have the method `summarize` defined with this signature exactly.

        // Now that we’ve defined the desired signatures of the `Summary` trait’s methods, we can implement it on the types in our media aggregator.
        struct NewsArticle {
            headline: String,
            location: String,
            author: String,
            content: String,
        }

        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }

        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        // One restriction to note is that we can implement a trait on a type only if at least one of the trait or the type is local to our crate.
        // For example, we can implement standard library traits like `Display` on a custom type like `Tweet` as part of our aggregator crate functionality, because the type `Tweet` is local to our aggregator crate.
        // We can also implement `Summary` on `Vec<T>` in our aggregator crate, because the trait `Summary` is local to our aggregator crate.

        // Implement external trait `Clone` for local type `Point`.
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T: Copy> Clone for Point<T> {
            fn clone(&self) -> Self {
                Self { x: self.x, y: self.y }
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = p1.clone();
        dbg!(p1, p2);

        // Implement local trait `Print` for external type `String`.
        trait Print {
            fn print(&self) where Self: Debug {
                println!("{:?}", self);
            }
        }

        impl Print for String {}

        let str = "42".to_string();
        str.print();

        // Cannot implement external trait `Clone` for external type `String`.
        // impl Clone for String {}
        // error[E0117]: only traits defined in the current crate can be implemented for types defined outside of the crate
        //   --> src/lessons/traits.rs:91:9
        //    |
        // 91 |         impl Clone for String {}
        //    |         ^^^^^^^^^^^^^^^------
        //    |         |              |
        //    |         |              `String` is not defined in the current crate
        //    |         impl doesn't use only types from inside the current crate
        //    |
        //    = note: define and implement a trait or new type instead

        // This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present.
        // This rule ensures that other people’s code can’t break your code and vice versa.
        // Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.
    }

    {
        // Sometimes it’s useful to have default behavior for some or all of the methods in a trait instead of requiring implementations for all methods on every type.
        // Then, as we implement the trait on a particular type, we can keep or override each method’s default behavior.

        trait Summary {
            fn summarize(&self) -> String {
                String::from("(Read more...)")
            }
        }

        struct NewsArticle {
            headline: String,
            location: String,
            author: String,
            content: String,
        }

        // Use a default trait implementation.
        impl Summary for NewsArticle {}

        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
            ),
        };

        println!("New article available! {}", article.summarize());

        // Creating a default implementation doesn’t require us to change anything about the implementation of `Summary` on `Tweet`.

        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());
    }

    {
        // Default implementations can call other methods in the same trait, even if those other methods don’t have a default implementation.
        // In this way, a trait can provide a lot of useful functionality and only require implementors to specify a small part of it.

        trait Summary {
            fn summarize_author(&self) -> String;

            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }

        #[derive(Debug)]
        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }

        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }

        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        };

        println!("1 new tweet: {}", tweet.summarize());

        struct NewsArticle {
            headline: String,
            location: String,
            author: String,
            content: String,
        }

        // Use a default trait implementation.
        impl Summary for NewsArticle {
            fn summarize_author(&self) -> String {
                format!("@{}", self.headline)
            }
        }

        // Note that it isn’t possible to call the default implementation from an overriding implementation of that same method.

        //

        {
            // We can define function that accept different types that implement certain trait.
            pub fn notify(item: &impl Summary) {
                println!("Breaking news! {}", item.summarize());
            }

            notify(&tweet);
        }

        {
            // The `impl Trait` syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound.
            pub fn notify<T: Summary>(item: &T) {
                println!("Breaking news! {}", item.summarize());
            }

            notify(&tweet);
        }

        // In such case this function will allow to have different types.
        // pub fn notify(item1: &impl Summary, item2: &impl Summary) {

        // If we want to force both parameters to have the same type, however, we must use a trait bound.
        // pub fn notify<T: Summary>(item1: &T, item2: &T) {


        // We can also specify more than one trait bound.

        // Version with impl Trait.
        // pub fn notify(item: &(impl Summary + Display)) {

        // Version wit trait bound.
        // pub fn notify<T: Summary + Display>(item: &T) {


        // Using too many trait bounds has its downsides.
        // Each generic has its own trait bounds, so functions with multiple generic type parameters can contain lots of trait bound information between the function’s name and its parameter list, making the function signature hard to read.
        // For this reason, Rust has alternate syntax for specifying trait bounds inside a where clause after the function signature.

        // Version with impl Trait.
        // fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {

        // Version wit trait bound.
        // fn some_function<T, U>(t: &T, u: &U) -> i32
        // where
        //     T: Display + Clone,
        //     U: Clone + Debug,
        // {

        //

        {
            // We can also use the impl Trait syntax in the return position to return a value of some type that implements a trait.
            fn returns_summarizable() -> impl Summary {
                Tweet {
                    username: String::from("horse_ebooks"),
                    content: String::from(
                        "of course, as you probably already know, people",
                    ),
                    reply: false,
                    retweet: false,
                }
            }

            let summary = returns_summarizable();
        }

        // However, you can only use impl Trait if you’re returning a single type.
        {
            // fn returns_summarizable(switch: bool) -> impl Summary {
            //     if switch {
            //         NewsArticle {
            //             headline: String::from(
            //                 "Penguins win the Stanley Cup Championship!",
            //             ),
            //             location: String::from("Pittsburgh, PA, USA"),
            //             author: String::from("Iceburgh"),
            //             content: String::from(
            //                 "The Pittsburgh Penguins once again are the best \
            //      hockey team in the NHL.",
            //             ),
            //         }
            //     } else {
            //         Tweet {
            //             username: String::from("horse_ebooks"),
            //             content: String::from(
            //                 "of course, as you probably already know, people",
            //             ),
            //             reply: false,
            //             retweet: false,
            //         }
            //     }
            // }

            // error[E0308]: `if` and `else` have incompatible types
            //    --> src/lessons/traits.rs:308:21
            //     |
            // 295 | /                   if switch {
            // 296 | | /                     NewsArticle {
            // 297 | | |                         headline: String::from(
            // 298 | | |                             "Penguins win the Stanley Cup Championship!",
            // 299 | | |                         ),
            // ...   | |
            // 305 | | |                         ),
            // 306 | | |                     }
            //     | | |_____________________- expected because of this
            // 307 | |                   } else {
            // 308 | | /                     Tweet {
            // 309 | | |                         username: String::from("horse_ebooks"),
            // 310 | | |                         content: String::from(
            // 311 | | |                             "of course, as you probably already know, people",
            // ...   | |
            // 314 | | |                         retweet: false,
            // 315 | | |                     }
            //     | | |_____________________^ expected `NewsArticle`, found `Tweet`
            // 316 | |                   }
            //     | |___________________- `if` and `else` have incompatible types
            //     |
            // help: you could change the return type to be a boxed trait object
            //     |
            // 294 |             fn returns_summarizable(switch: bool) -> Box<dyn Summary> {
            //     |                                                      ~~~~~~~        +
            // help: if you change the return type to expect trait objects, box the returned expressions
            //     |
            // 296 ~                     Box::new(NewsArticle {
            // 297 |                         headline: String::from(
            //   ...
            // 305 |                         ),
            // 306 ~                     })
            // 307 |                 } else {
            // 308 ~                     Box::new(Tweet {
            // 309 |                         username: String::from("horse_ebooks"),
            //   ...
            // 314 |                         retweet: false,
            // 315 ~                     })
            //     |
        }

        //

        // By using a trait bound with an impl block that uses generic type parameters, we can implement methods conditionally for types that implement the specified traits.
        struct Pair<T> {
            x: T,
            y: T,
        }

        impl<T> Pair<T> {
            fn new(x: T, y: T) -> Self {
                Self { x, y }
            }
        }

        impl<T: Display + PartialOrd> Pair<T> {
            fn cmp_display(&self) {
                if self.x >= self.y {
                    println!("The largest member is x = {}", self.x);
                } else {
                    println!("The largest member is y = {}", self.y);
                }
            }
        }

        let p = Pair { x: 42.0, y: 25.4 };
        p.cmp_display();

        // We can also conditionally implement a trait for any type that implements another trait.
        // Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.
        // impl<T: Display> ToString for T {
        //     // --snip--
        // }
        // Because the standard library has this blanket implementation, we can call the `to_string` method defined by the `ToString` trait on any type that implements the `Display` trait.
        // For example, we can turn integers into their corresponding `String` values like this because integers implement `Display`.
        let s = 3.to_string();
        dbg!(s);
    }
}
