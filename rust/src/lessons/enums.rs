// https://doc.rust-lang.org/stable/book/ch06-01-defining-an-enum.html

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
        dbg!(&ip);
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
        dbg!(&home);
        let loopback = IpAddr {
            kind: IpAddrKind::V6,
            address: String::from("::1"),
        };
        dbg!(&loopback);
    }

    // Add address using enum
    {
        #[derive(Debug)]
        enum IpAddr {
            V4(String),
            V6(String),
        }
        let home = IpAddr::V4(String::from("127.0.0.1"));
        dbg!(&home);
        let loopback = IpAddr::V6(String::from("::1"));
        dbg!(&loopback);
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
        dbg!(&home);
        let loopback = IpAddr::V6(String::from("::1"));
        dbg!(&loopback);
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
                dbg!(&self);
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
}

// Rust enums are implemented as C tagged unions
// https://patshaughnessy.net/2018/3/15/how-rust-implements-tagged-unions
