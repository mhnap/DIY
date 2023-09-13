// https://doc.rust-lang.org/book/ch19-04-advanced-types.html

fn main() {
    // The newtype pattern is useful for statically enforcing that values are never confused and indicating the units of a value.
    // If we wrote a function with a parameter of type Millimeters, we couldn’t compile a program that accidentally tried to call that function with a value of type Meters or a plain u32.

    // We can also use the newtype pattern to abstract away some implementation details of a type: the new type can expose a public API that is different from the API of the private inner type.
    // The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details.

    //

    // Rust provides the ability to declare a type alias to give an existing type another name.
    type Kilometers = i32;

    // Now, the alias Kilometers is a synonym for i32; unlike the newtypes, Kilometers is not a separate, new type.
    // Values that have the type Kilometers will be treated the same as values of type i32:
    let x: i32 = 5;
    let y: Kilometers = 5;
    println!("x + y = {}", x + y);

    // The main use case for type synonyms is to reduce repetition.
    type Thunk = Box<dyn Fn() + Send + 'static>;
    let f: Thunk = Box::new(|| println!("hi"));
    fn takes_long_type(f: Thunk) {
        // --snip--
    }
    fn returns_long_type() -> Thunk {
        todo!()
    }

    // The type alias helps in two ways: it makes code easier to write and it gives us a consistent interface across all usages.

    //

    // Rust has a special type named ! that’s known in type theory lingo as the empty type because it has no values.
    // We prefer to call it the never type because it stands in the place of the return type when a function will never return.
    fn bar() -> ! {
        todo!()
    }

    // This code is read as “the function bar returns never.”
    // Functions that return never are called diverging functions.
    // We can’t create values of the type ! so bar can never possibly return.

    loop {
        let guess = "42";
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        break;
    }

    // Continue has a ! value.
    // That is, when Rust computes the type of guess, it looks at both match arms, the former with a value of u32 and the latter with a ! value.
    // Because ! can never have a value, Rust decides that the type of guess is u32.

    // The formal way of describing this behavior is that expressions of type ! can be coerced into any other type.
    // We’re allowed to end this match arm with continue because continue doesn’t return a value; instead, it moves control back to the top of the loop, so in the Err case, we never assign a value to guess.

    // The never type is useful with the panic! macro as well.
    // impl<T> Option<T> {
    //     pub fn unwrap(self) -> T {
    //         match self {
    //             Some(val) => val,
    //             None => panic!("called `Option::unwrap()` on a `None` value"),
    //         }
    //     }
    // }

    // Rust sees that val has the type T and panic! has the type !, so the result of the overall match expression is T.
    // This code works because panic! doesn’t produce a value; it ends the program.
    // In the None case, we won’t be returning a value from unwrap, so this code is valid.

    // One final expression that has the type ! is a loop.
    print!("forever ");
    let f = loop {
        print!("and ever ");
        // However, this wouldn’t be true if we included a break, because the loop would terminate when it got to the break.
        break;
    };

    //

    // Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type.
    // This leaves one corner of its type system a little confusing at first: the concept of dynamically sized types.
    // Sometimes referred to as DSTs or unsized types, these types let us write code using values whose size we can know only at runtime.

    // Let’s dig into the details of a dynamically sized type called str, which we’ve been using throughout the book.
    // That’s right, not &str, but str on its own, is a DST.
    // We can’t know how long the string is until runtime, meaning we can’t create a variable of type str, nor can we take an argument of type str.

    // let s1: str = "Hello there!";
    // let s2: str = "How's it going?";
    // error[E0308]: mismatched types
    //   --> src/lessons/types.rs:92:19
    //    |
    // 92 |     let s1: str = "Hello there!";
    //    |             ---   ^^^^^^^^^^^^^^ expected `str`, found `&str`
    //    |             |
    //    |             expected due to this

    // Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must use the same amount of memory.
    // If Rust allowed us to write this code, these two str values would need to take up the same amount of space.
    // But they have different lengths: s1 needs 12 bytes of storage and s2 needs 15.
    // This is why it’s not possible to create a variable holding a dynamically sized type.

    let s1: &str = "Hello there!";
    dbg!(s1);

    // So although a &T is a single value that stores the memory address of where the T is located, a &str is two values: the address of the str and its length.
    // As such, we can know the size of a &str value at compile time: it’s twice the length of a usize.
    // That is, we always know the size of a &str, no matter how long the string it refers to is.
    // In general, this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information.
    // The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind.

    // Every trait is a dynamically sized type we can refer to by using the name of the trait.
    // To use traits as trait objects, we must put them behind a pointer, such as &dyn Trait or Box<dyn Trait> (Rc<dyn Trait> would work too).

    // To work with DSTs, Rust provides the Sized trait to determine whether or not a type’s size is known at compile time.
    // This trait is automatically implemented for everything whose size is known at compile time.
    // In addition, Rust implicitly adds a bound on Sized to every generic function.

    // That is, a generic function definition like this:
    fn generic_v1<T>(t: T) {
        // --snip--
    }
    // is actually treated as though we had written this:
    fn generic_v2<T: Sized>(t: T) {
        // --snip--
    }

    // By default, generic functions will work only on types that have a known size at compile time.
    // However, we can use the following special syntax to relax this restriction:

    // fn generic_v3<T: ?Sized>(t: T) {
    //     // --snip--
    // }
    // error[E0277]: the size for values of type `T` cannot be known at compilation time
    //    --> src/lessons/types.rs:135:30
    //     |
    // 135 |     fn generic_v3<T: ?Sized>(t: T) {
    //     |                   -          ^ doesn't have a size known at compile-time
    //     |                   |
    //     |                   this type parameter needs to be `std::marker::Sized`
    //     |
    // help: consider removing the `?Sized` bound to make the type parameter `Sized`
    //     |
    // 135 -     fn generic_v3<T: ?Sized>(t: T) {
    // 135 +     fn generic_v3<T>(t: T) {
    //     |
    // help: function arguments must have a statically known size, borrowed types always have a known size
    //     |
    // 135 |     fn generic_v3<T: ?Sized>(t: &T) {
    //     |                                 +

    fn generic_v3<T: ?Sized>(t: &T) {
        // --snip--
    }

    // A trait bound on ?Sized means “T may or may not be Sized” and this notation overrides the default that generic types must have a known size at compile time.
    // The ?Trait syntax with this meaning is only available for Sized, not any other traits.

    // Also note that we switched the type of the t parameter from T to &T. Because the type might not be Sized, we need to use it behind some kind of pointer.
    // In this case, we’ve chosen a reference.
}
