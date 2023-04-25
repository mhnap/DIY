fn main() {
    // Structs are similar to tuples, in that both hold multiple related values.
    // Like tuples, the pieces of a struct can be different types. Unlike with tuples, in a struct you’ll name each piece of data so it’s clear what the values mean.
    // Adding these names means that structs are more flexible than tuples: you don’t have to rely on the order of the data to specify or access the values of an instance.
    let mut user1 = (
        true,
        String::from("someusername123"),
        String::from("someone@example.com"),
        1,
    );
    user1.2 = String::from("anotheremail@example.com");
    dbg!(&user1);

    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    // To change a value, the entire instance must be mutable.
    // Rust doesn’t allow us to mark only certain fields as mutable.
    user1.email = String::from("anotheremail@example.com");
    dbg!(&user1);

    // As with any expression, we can construct a new instance of the struct as the last expression in the function body to implicitly return that new instance.
    fn build_user_v1(email: String, username: String) -> User {
        User {
            active: true,
            username: username,
            email: email,
            sign_in_count: 1,
        }
    }
    let user1 = build_user_v1(
        "someusername123".to_string(),
        "someone@example.com".to_string(),
    );

    //

    // Because the parameter names and the struct field names are exactly the same, we can use the field init shorthand syntax to rewrite build_user so it behaves exactly the same but doesn’t have the repetition of username and email.
    fn build_user_v2(email: String, username: String) -> User {
        User {
            active: true,
            username,
            email,
            sign_in_count: 1,
        }
    }
    let user1 = build_user_v1(
        "someusername123".to_string(),
        "someone@example.com".to_string(),
    );

    //

    // It’s often useful to create a new instance of a struct that includes most of the values from another instance, but changes some.
    // You can do this using struct update syntax.

    // Regular syntax
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    // Using struct update syntax, we can achieve the same effect with less code.
    // The syntax .. specifies that the remaining fields not explicitly set should have the same value as the fields in the given instance.
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };

    // Struct also can be destructured
    let User {
        active,
        username,
        email,
        sign_in_count,
    } = user3;
    dbg!(active, username, email, sign_in_count);

    //

    // Rust also supports structs that look similar to tuples, called tuple structs.
    // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields; rather, they just have the types of the fields.
    // Tuple structs are useful when you want to give the whole tuple a name and make the tuple a different type from other tuples, and when naming each field as in a regular struct would be verbose or redundant.
    #[derive(Debug)]
    struct Color(i32, i32, i32);
    #[derive(Debug)]
    struct Point(i32, i32, i32);
    let black = Color(0, 0, 0);
    let mut origin = Point(0, 0, 0);
    dbg!(&black);
    dbg!(&origin);

    // Tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
    origin.2 += 1;
    // let (a, b, c) = origin;
    // error[E0308]: mismatched types
    //   --> src/lessons/structs.rs:94:9
    //    |
    // 94 |     let (a, b, c) = origin;
    //    |         ^^^^^^^^^   ------ this expression has type `Point`
    //    |         |
    //    |         expected `Point`, found `(_, _, _)`
    //    |
    //    = note: expected struct `Point`
    //                found tuple `(_, _, _)`

    // Constructor syntax is designed to be symmetric with pattern syntax. So, tuple struct destructuring looks like this.
    let Point(a, b, c) = origin;
    dbg!(c);

    //

    // You can also define structs that don’t have any fields!
    // These are called unit-like structs because they behave similarly to (), the unit type.
    // Unit-like structs can be useful when you need to implement a trait on some type but don’t have any data that you want to store in the type itself.
    #[derive(Debug)]
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    dbg!(&subject);

    //

    // Rust struct syntax avoid C struct fields reordering issue.
    // struct X {
    //     a: i8,
    //     b: i8,
    //     c: i8
    // }
    // behave the same as
    struct X {
        b: i8,
        a: i8,
        c: i8,
    }
    let x = X { a: 1, b: 2, c: 3 };
    println!("{}{}{}", x.a, x.b, x.c);

    // Even better when there are already variables with values, because we can use field init shorthand syntax.
    let (a, b, c) = (1, 2, 3);
    let x = X { a, b, c };
    println!("{}{}{}", x.a, x.b, x.c);
    // Order doesn't matter
    let x = X { a, c, b };
    println!("{}{}{}", x.a, x.b, x.c);
}

// https://users.rust-lang.org/t/noob-question-about-data-struct-ure-syntax/16140/12
// https://users.rust-lang.org/t/how-to-destructure-a-tuple-struct/45296
// https://users.rust-lang.org/t/the-struct-update-syntax/16519/5
