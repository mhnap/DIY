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
            content: String::from("of course, as you probably already know, people"),
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
                Self {
                    x: self.x,
                    y: self.y,
                }
            }
        }

        let p1 = Point { x: 1, y: 2 };
        let p2 = p1.clone();
        dbg!(p1, p2);

        // Implement local trait `Print` for external type `String`.
        trait Print {
            fn print(&self)
            where
                Self: Debug,
            {
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
            content: String::from("of course, as you probably already know, people"),
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
            content: String::from("of course, as you probably already know, people"),
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
                    content: String::from("of course, as you probably already know, people"),
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

    //

    // https://doc.rust-lang.org/book/ch19-03-advanced-traits.html

    // Associated types connect a type placeholder with a trait such that the trait method definitions can use these placeholder types in their signatures.
    // The implementor of a trait will specify the concrete type to be used instead of the placeholder type for the particular implementation.
    // That way, we can define a trait that uses some types without needing to know exactly what those types are until the trait is implemented.

    // One example of a trait with an associated type is the Iterator trait that the standard library provides.
    // The associated type is named Item and stands in for the type of the values the type implementing the Iterator trait is iterating over.
    // The definition of the Iterator trait:
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }

    // Associated types might seem like a similar concept to generics, in that the latter allow us to define a function without specifying what types it can handle.
    // To examine the difference between the two concepts, we’ll look at an implementation of the Iterator trait on a type named Counter that specifies the Item type is u32:
    struct Counter;
    impl Iterator for Counter {
        type Item = u32;
        fn next(&mut self) -> Option<Self::Item> {
            todo!()
        }
    }

    // This syntax seems comparable to that of generics.
    pub trait IteratorV2<T> {
        fn next(&mut self) -> Option<T>;
    }

    // The difference is that when using generics, we must annotate the types in each implementation; because we can also implement Iterator<String> for Counter or any other type, we could have multiple implementations of Iterator for Counter.
    // In other words, when a trait has a generic parameter, it can be implemented for a type multiple times, changing the concrete types of the generic type parameters each time.
    // When we use the next method on Counter, we would have to provide type annotations to indicate which implementation of Iterator we want to use.

    impl IteratorV2<u32> for Counter {
        fn next(&mut self) -> Option<u32> {
            todo!()
        }
    }

    impl IteratorV2<String> for Counter {
        fn next(&mut self) -> Option<String> {
            todo!()
        }
    }

    // With associated types, we don’t need to annotate types because we can’t implement a trait on a type multiple times.
    // With the definition that uses associated types, we can only choose what the type of Item will be once, because there can only be one impl Iterator for Counter.
    // We don’t have to specify that we want an iterator of u32 values everywhere that we call next on Counter.

    // Associated types also become part of the trait’s contract: implementors of the trait must provide a type to stand in for the associated type placeholder.
    // Associated types often have a name that describes how the type will be used, and documenting the associated type in the API documentation is good practice.

    //

    // When we use generic type parameters, we can specify a default concrete type for the generic type.
    // This eliminates the need for implementors of the trait to specify a concrete type if the default type works.

    // A great example of a situation where this technique is useful is with operator overloading, in which you customize the behavior of an operator (such as +) in particular situations.
    // Rust doesn’t allow you to create your own operators or overload arbitrary operators.
    // But you can overload the operations and corresponding traits listed in std::ops by implementing the traits associated with the operator.

    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Point;

        fn add(self, other: Point) -> Self::Output {
            Point {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );

    // The default generic type in this code is within the Add trait.
    // trait Add<Rhs = Self> {
    //     type Output;
    //
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }

    // The new part is Rhs=Self: this syntax is called default type parameters.
    // The Rhs generic type parameter (short for “right hand side”) defines the type of the rhs parameter in the add method.
    // If we don’t specify a concrete type for Rhs when we implement the Add trait, the type of Rhs will default to Self, which will be the type we’re implementing Add on.

    // When we implemented Add for Point, we used the default for Rhs because we wanted to add two Point instances.
    // Let’s look at an example of implementing the Add trait where we want to customize the Rhs type rather than using the default.
    struct Millimeters(u32);
    struct Meters(u32);

    impl Add<Meters> for Millimeters {
        type Output = Millimeters;

        fn add(self, other: Meters) -> Millimeters {
            Millimeters(self.0 + (other.0 * 1000))
        }
    }

    let meters = Meters(1);
    let millimeters = Millimeters(1);

    // dbg!(meters + millimeters);
    // error[E0369]: cannot add `Millimeters` to `Meters`
    //    --> src/lessons/traits.rs:510:17
    //     |
    // 510 |     dbg!(meters + millimeters);
    //     |          ------ ^ ----------- Millimeters
    //     |          |
    //     |          Meters
    //     |
    // note: an implementation of `Add<Millimeters>` might be missing for `Meters`
    //    --> src/lessons/traits.rs:498:5
    //     |
    // 498 |     struct Meters(u32);
    //     |     ^^^^^^^^^^^^^ must implement `Add<Millimeters>`
    // note: the trait `Add` must be implemented
    //    --> /home/mhnap/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/arith.rs:76:1
    //     |
    // 76  | pub trait Add<Rhs = Self> {
    //     | ^^^^^^^^^^^^^^^^^^^^^^^^^

    dbg!((millimeters + meters).0);

    //

    // Nothing in Rust prevents a trait from having a method with the same name as another trait’s method, nor does Rust prevent you from implementing both traits on one type.
    // It’s also possible to implement a method directly on the type with the same name as methods from traits.
    // When calling methods with the same name, you’ll need to tell Rust which one you want to use.

    trait Pilot {
        fn fly(&self);
    }

    trait Wizard {
        fn fly(&self);
    }

    struct Human;

    impl Pilot for Human {
        fn fly(&self) {
            println!("This is your captain speaking.");
        }
    }

    impl Wizard for Human {
        fn fly(&self) {
            println!("Up!");
        }
    }

    impl Human {
        fn fly(&self) {
            println!("*waving arms furiously*");
        }
    }

    // When we call fly on an instance of Human, the compiler defaults to calling the method that is directly implemented on the type.
    let person = Human;
    person.fly();

    // Specifying the trait name before the method name clarifies to Rust which implementation of fly we want to call.
    Pilot::fly(&person);
    Wizard::fly(&person);
    Human::fly(&person);

    // Because the fly method takes a self parameter, if we had two types that both implement one trait, Rust could figure out which implementation of a trait to use based on the type of self.

    // However, associated functions that are not methods don’t have a self parameter.
    // When there are multiple types or traits that define non-method functions with the same function name, Rust doesn't always know which type you mean unless you use fully qualified syntax.
    trait Animal {
        fn baby_name() -> String;
    }

    struct Dog;

    impl Dog {
        fn baby_name() -> String {
            String::from("Spot")
        }
    }

    impl Animal for Dog {
        fn baby_name() -> String {
            String::from("puppy")
        }
    }

    println!("A baby dog is called a {}", Dog::baby_name());

    // Attempting to call the baby_name function from the Animal trait, but Rust doesn’t know which implementation to use.
    // println!("A baby dog is called a {}", Animal::baby_name());
    // error[E0790]: cannot call associated function on trait without specifying the corresponding `impl` type
    //    --> src/lessons/traits.rs:600:43
    //     |
    // 581 |         fn baby_name() -> String;
    //     |         ------------------------- `Animal::baby_name` defined here
    // ...
    // 600 |     println!("A baby dog is called a {}", Animal::baby_name());
    //     |                                           ^^^^^^^^^^^^^^^^^ cannot call associated function of trait
    //     |
    // help: use the fully-qualified path to the only available implementation
    //     |
    // 600 |     println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    //     |                                           +++++++       +

    // Because Animal::baby_name doesn’t have a self parameter, and there could be other types that implement the Animal trait, Rust can’t figure out which implementation of Animal::baby_name we want.

    // To disambiguate and tell Rust that we want to use the implementation of Animal for Dog as opposed to the implementation of Animal for some other type, we need to use fully qualified syntax.
    println!("A baby dog is called a {}", <Dog as Animal>::baby_name());

    // In general, fully qualified syntax is defined as follows:
    // <Type as Trait>::function(receiver_if_method, next_arg, ...);

    //

    // Sometimes, you might write a trait definition that depends on another trait: for a type to implement the first trait, you want to require that type to also implement the second trait.
    // You would do this so that your trait definition can make use of the associated items of the second trait.
    // The trait your trait definition is relying on is called a supertrait of your trait.
    use std::fmt;
    trait OutlinePrint: fmt::Display {
        fn outline_print(&self) {
            let output = self.to_string();
            let len = output.len();
            println!("{}", "*".repeat(len + 4));
            println!("*{}*", " ".repeat(len + 2));
            println!("* {} *", output);
            println!("*{}*", " ".repeat(len + 2));
            println!("{}", "*".repeat(len + 4));
        }
    }

    impl OutlinePrint for str {}
    let hello = "hello";
    hello.outline_print();

    // Let’s see what happens when we try to implement OutlinePrint on a type that doesn’t implement Display.
    // impl OutlinePrint for Point {}
    // error[E0277]: `main::Point` doesn't implement `std::fmt::Display`
    //    --> src/lessons/traits.rs:645:27
    //     |
    // 645 |     impl OutlinePrint for Point {}
    //     |                           ^^^^^ `main::Point` cannot be formatted with the default formatter
    //     |
    //     = help: the trait `std::fmt::Display` is not implemented for `main::Point`
    //     = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
    // note: required by a bound in `OutlinePrint`
    //    --> src/lessons/traits.rs:629:25
    //     |
    // 629 |     trait OutlinePrint: Display {
    //     |                         ^^^^^^^ required by this bound in `OutlinePrint`

    // To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires.
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
    impl OutlinePrint for Point {}

    let point = Point { x: 1, y: 3 };
    point.outline_print();

    //

    // Orphan rule states we’re only allowed to implement a trait on a type if either the trait or the type are local to our crate.
    // It’s possible to get around this restriction using the newtype pattern, which involves creating a new type in a tuple struct.
    // The tuple struct will have one field and be a thin wrapper around the type we want to implement a trait for.
    // Then the wrapper type is local to our crate, and we can implement the trait on the wrapper.
    // Newtype is a term that originates from the Haskell programming language.
    // There is no runtime performance penalty for using this pattern, and the wrapper type is elided at compile time.

    // As an example, let’s say we want to implement Display on Vec<T>, which the orphan rule prevents us from doing directly because the Display trait and the Vec<T> type are defined outside our crate.
    struct Wrapper(Vec<String>);
    impl fmt::Display for Wrapper {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}]", self.0.join(", "))
        }
    }

    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);

    // The downside of using this technique is that Wrapper is a new type, so it doesn’t have the methods of the value it’s holding.
    // We would have to implement all the methods of Vec<T> directly on Wrapper such that the methods delegate to self.0, which would allow us to treat Wrapper exactly like a Vec<T>.
    // If we wanted the new type to have every method the inner type has, implementing the Deref trait on the Wrapper to return the inner type would be a solution.
    // If we don’t want the Wrapper type to have all the methods of the inner type—for example, to restrict the Wrapper type’s behavior—we would have to implement just the methods we do want manually.
}
