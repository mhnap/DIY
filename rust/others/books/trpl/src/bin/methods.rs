// https://doc.rust-lang.org/book/ch05-03-method-syntax.html

fn main() {
    {
        fn area(width: u32, height: u32) -> u32 {
            width * height
        }
        let width = 30;
        let height = 50;
        println!("The area of the rectangle is {} square pixels.", area(width, height));
    }

    // With tuple
    {
        fn area(dimensions: (u32, u32)) -> u32 {
            dimensions.0 * dimensions.1
        }
        let rectangle = (30, 50);
        println!("The area of the rectangle is {} square pixels.", area(rectangle));
    }

    // With struct
    {
        struct Rectangle {
            width: u32,
            height: u32,
        }
        fn area(rectangle: &Rectangle) -> u32 {
            rectangle.width * rectangle.height
        }
        let rectangle = Rectangle { width: 30, height: 50 };
        println!("The area of the rectangle is {} square pixels.", area(&rectangle));
    }

    // Methods are similar to functions: we declare them with the fn keyword and a name, they can have parameters and a return value, and they contain some code that’s run when the method is called from somewhere else.
    // Unlike functions, methods are defined within the context of a struct (or an enum or a trait object), and their first parameter is always self, which represents the instance of the struct the method is being called on.

    // With struct and method
    {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        impl Rectangle {
            // The &self is actually short for self: &Self.
            // Within an impl block, the type Self is an alias for the type that the impl block is for.
            // Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
            // Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
            fn area(&self) -> u32 {
                self.width * self.height
            }
        }
        let rectangle = Rectangle { width: 30, height: 50 };
        println!("The area of the rectangle is {} square pixels.", rectangle.area());

        //

        // Rust doesn’t have an equivalent to the -> operator; instead, Rust has a feature called automatic referencing and dereferencing.
        // Calling methods is one of the few places in Rust that has this behavior.
        let rec_ref = &rectangle;
        println!("The area of the rectangle is {} square pixels.", rec_ref.area());
        // The same as
        println!("The area of the rectangle is {} square pixels.", (*rec_ref).area());
        // And the same as
        println!("The area of the rectangle is {} square pixels.", (&rectangle).area());
        // And as
        println!("The area of the rectangle is {} square pixels.", (*(&rectangle)).area());
        // Can be called on instance as associated function
        println!("The area of the rectangle is {} square pixels.", Rectangle::area(&rectangle));

        //

        // Can add methods separately later.
        impl Rectangle {
            // Also can use the same identifiers for fields and methods.
            fn width(&self) -> bool {
                self.width > 0
            }
        }

        impl Rectangle {
            fn can_hold(&self, other: &Self) -> bool {
                self.width > other.width && self.height > other.height
            }
        }
        let rect1 = Rectangle { width: 30, height: 50 };
        let rect2 = Rectangle { width: 10, height: 40 };
        let rect3 = Rectangle { width: 60, height: 45 };
        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        //

        // All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl.
        // We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.
        // Associated functions that aren’t methods are often used for constructors that will return a new instance of the struct.
        impl Rectangle {
            fn square(size: u32) -> Self {
                Self { width: size, height: size }
            }
        }
        let sq = Rectangle::square(3);
        dbg!(&sq);
    }

    {
        // impl i8 {}
        // error[E0390]: cannot define inherent `impl` for primitive types
        //    --> src/lessons/methods.rs:151:9
        //     |
        // 151 |         impl i8 {}
        //     |         ^^^^^^^
        //     |
        //     = help: consider using an extension trait instead

        // impl String {}
        // error[E0116]: cannot define inherent `impl` for a type outside of the crate where the type is defined
        //    --> src/lessons/methods.rs:160:9
        //     |
        // 160 |         impl String {}
        //     |         ^^^^^^^^^^^ impl for type defined outside of crate.
        //     |
        //     = note: define and implement a trait or new type instead
    }
}

// https://www.reddit.com/r/rust/comments/285rfb/what_was_the_design_rationale_for_explicit_self/
// https://internals.rust-lang.org/t/skip-typing-self-each-time-in-a-method-and-instead-use/17627
// https://stackoverflow.com/questions/72486023/rust-why-isnt-self-a-reference-when-self-self
