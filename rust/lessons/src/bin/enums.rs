// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html

use std::mem::size_of_val;

fn main() {
    {
        // Where structs give you a way of grouping together related fields and data, like a Rectangle with its width and height,
        // enums give you a way of saying a value is one of a possible set of values.
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
        // Should be explicitly named
        // let ip = V4;
        // error[E0425]: cannot find value `V4` in this scope
        //   --> src/lessons/enums.rs:11:14
        //    |
        // 11 |     let ip = V4;
        //    |              ^^ not found in this scope
        //    |
        let ip = IpAddrKind::V4;
        dbg!(&ip, size_of_val(&ip));
    }

    //

    // Add address using separate struct
    {
        #[derive(Debug)]
        enum IpAddrKind {
            V4,
            V6,
        }
        #[derive(Debug)]
        struct IpAddr {
            kind: IpAddrKind,
            address: String,
        }
        let home = IpAddr {
            kind: IpAddrKind::V4,
            address: String::from("127.0.0.1"),
        };
        dbg!(&home, size_of_val(&home));
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        dbg!(&loopback, size_of_val(&loopback));
    }

    // Add address using enum
    {
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let home = IpAddr::V4(String::from("127.0.0.1"));
        dbg!(&home, size_of_val(&home));
        let loopback = IpAddr::V6(String::from("::1"));
        dbg!(&loopback, size_of_val(&loopback));
        // Here, it’s also easier to see another detail of how enums work:
        // the name of each enum variant that we define also becomes a function that constructs an instance of the enum.
    }

    //

    {
        // There’s another advantage to using an enum rather than a struct:
        // each variant can have different types and amounts of associated data.
        #[derive(Debug)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        let home = IpAddr::V4(127, 0, 0, 1);
        dbg!(&home, size_of_val(&home));
        let loopback = IpAddr::V6(String::from("::1"));
        dbg!(&loopback, size_of_val(&loopback));
    }

    //

    {
        // Can implement methods on enum as well.
        #[derive(Debug)]
        enum Message {
            Quit,
            Move { x: i32, y: i32 },
            Write(String),
            ChangeColor(i32, i32, i32),
        }
        impl Message {
            fn call(&self) {
                dbg!(self, size_of_val(self));
            }
        }
        // Can have an array of enum variants even if they store different data types inside.
        let arr = [
            Message::Quit,
            Message::Move { x: 1, y: 2 },
            Message::Write(String::from("Hi")),
            Message::ChangeColor(0, 1, 2),
        ];
        for msg in arr {
            msg.call();
        }
    }

    //

    {
        // The Option type encodes the very common scenario in which a value could be something or it could be nothing.
        let some_number = Some(5);
        dbg!(&some_number, size_of_val(&some_number));
        let absent_string: Option<String> = None;
        dbg!(&absent_string, size_of_val(&absent_string));
        // Cannot add Option<integer> to integer
        // let number = 3 + some_number;
        // error[E0277]: cannot add `Option<{integer}>` to `{integer}`
        //    --> src/lessons/enums.rs:120:24
        //     |
        // 120 |         let number = 3 + some_number;
        //     |                        ^ no implementation for `{integer} + Option<{integer}>`
        //     |
        //     = help: the trait `Add<Option<{integer}>>` is not implemented for `{integer}`
        //     = help: the following other types implement trait `Add<Rhs>`:
        //               <&'a f32 as Add<f32>>
        //               <&'a f64 as Add<f64>>
        //               <&'a i128 as Add<i128>>
        //               <&'a i16 as Add<i16>>
        //               <&'a i32 as Add<i32>>
        //               <&'a i64 as Add<i64>>
        //               <&'a i8 as Add<i8>>
        //               <&'a isize as Add<isize>>
        //             and 48 others

        // In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
        // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null when it actually is.

        // This will panic as Option is None
        println!("{}", absent_string.unwrap());
    }
}

// Rust enums are implemented as C tagged unions
// https://patshaughnessy.net/2018/3/15/how-rust-implements-tagged-unions
