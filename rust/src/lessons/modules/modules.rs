// https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html

// Features sometimes collectively referred to as the module system, include:
//
// Packages: A Cargo feature that lets you build, test, and share crates
// Crates: A tree of modules that produces a library or executable
// Modules and use: Let you control the organization, scope, and privacy of paths
// Paths: A way of naming an item, such as a struct, function, or module

// A crate is the smallest amount of code that the Rust compiler considers at a time.
// A crate can come in one of two forms: a binary crate or a library crate.

// A package is a bundle of one or more crates that provides a set of functionality.
// A package contains a Cargo.toml file that describes how to build those crates.
// A package can contain as many binary crates as you like, but at most only one library crate.
// A package must contain at least one crate, whether that’s a library or binary crate.

// Modules let us organize code within a crate for readability and easy reuse.
// Modules also allow us to control the privacy of items, because code within a module is private by default.

mod garden;

fn main() {
    // Modules can be private (by default) or public.
    // let asparagus = garden::vegetables::Asparagus;
    // error[E0603]: module `vegetables` is private
    //   --> src/lessons/modules/modules.rs:19:29
    //    |
    // 19 |     let asparagus = garden::vegetables::Asparagus;
    //    |                             ^^^^^^^^^^ private module
    //    |
    // note: the module `vegetables` is defined here
    //   --> src/lessons/modules/garden.rs:1:1
    //    |
    // 1  | mod vegetables;
    //    | ^^^^^^^^^^^^^^^

    // All items in a module are also private by default, even if a module is public
    // let asparagus = garden::vegetables::Asparagus;
    // error[E0603]: unit struct `Asparagus` is private
    //   --> src/lessons/modules/modules.rs:34:41
    //    |
    // 34 |     let asparagus = garden::vegetables::Asparagus;
    //    |                                         ^^^^^^^^^ private unit struct
    //    |
    // note: the unit struct `Asparagus` is defined here
    //   --> src/lessons/modules/garden/vegetables.rs:2:1
    //    |
    // 2  | struct Asparagus;
    //    | ^^^^^^^^^^^^^^^^^

    let asparagus = garden::vegetables::Asparagus;
    dbg!(asparagus);

    // Can create a shortcut with use.
    use garden::vegetables::Asparagus;
    let asparagus = Asparagus;
    dbg!(asparagus);

    // Cannot use private items from the parent module.
    // println!("Garden name - {}", garden::GARDEN_NAME);
    // error[E0603]: constant `GARDEN_NAME` is private
    //   --> src/lessons/modules/modules.rs:59:42
    //    |
    // 59 |     println!("Garden name - {}", garden::GARDEN_NAME);
    //    |                                          ^^^^^^^^^^^ private constant
    //    |
    // note: the constant `GARDEN_NAME` is defined here
    //   --> src/lessons/modules/garden.rs:3:1
    //    |
    // 3  | const GARDEN_NAME: &str = "Mike's garden";
    //    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

    // But the child module can use parent's items, even private ones.
    println!("Garden name - {}", garden::vegetables::get_garden_name());

    // Cannot instantiate struct with the private field.
    // let potato = garden::vegetables::Potato{weight: 10, kind: "homemade".into()};
    // error[E0451]: field `kind` of struct `Potato` is private
    //   --> src/lessons/modules/modules.rs:76:57
    //    |
    // 76 |     let potato = garden::vegetables::Potato{weight: 10, kind: "homemade".into()};
    //    |                                                         ^^^^^^^^^^^^^^^^^^^^^^^ private field

    // Need to use associated function call.
    let potato = garden::vegetables::Potato::new(10);
    dbg!(potato);

    // Can use any enum variant as a pub.
    let herb = garden::vegetables::Herb::Mint;
    dbg!(herb);

    //

    // Bringing the `get_garden_name` function into scope with use, which is unidiomatic
    {
        use garden::vegetables::get_garden_name;
        println!("Garden name - {}", get_garden_name());
    }

    // Bringing the `vegetables` module into scope with use and specifying the parent module when calling the function, which is idiomatic
    // Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing repetition of the full path.
    {
        use garden::vegetables;
        println!("Garden name - {}", vegetables::get_garden_name());
    }

    // On the other hand, when bringing in structs, enums, and other items with use, it’s idiomatic to specify the full path.
    // There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and writing Rust code this way.
    {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 2);

        // The exception to this idiom is if we’re bringing two items with the same name into scope with use statements, because Rust doesn’t allow that.
        {
            use std::fmt;
            use std::io;
            fn function1() -> fmt::Result {
                Ok(())
            }
            fn function2() -> io::Result<()> {
                Ok(())
            }
        }

        // There’s another solution to the problem of bringing two types of the same name into the same scope with use: after the path, we can specify as and a new local name, or alias, for the type.
        {
            use std::fmt::Result;
            use std::io::Result as IoResult;
            fn function1() -> Result {
                Ok(())
            }
            fn function2() -> IoResult<()> {
                Ok(())
            }
        }
    }

    // Use re-exported enum from `vegetables`.
    {
        let herb = garden::Herb::Basil;
        dbg!(herb);
    }

    // To use any external package (even std), need to bring crate definition into the scope.
    {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        map.insert(1, 2);
    }

    //

    // We can use nested paths to bring the same items into scope in one line.
    // We do this by specifying the common part of the path, followed by two colons, and then curly brackets around a list of the parts of the paths that differ.
    {
        use std::{cmp::Ordering, io};
    }

    // Two use statements where one is a subpath of the other.
    {
        use std::io;
        use std::io::Write;
    }

    // To merge these two paths into one use statement, we can use self in the nested path.
    {
        use std::io::{self, Write};
    }

    // Some crazy user can implement its own "std" module.
    // In such case we can fix ambiguity by using "::" for true std module.
    // Such technique is often used in macros.
    {
        mod std {}
        // use std::io::{self, Write};
        // error[E0659]: `std` is ambiguous
        //    --> src/lessons/modules/modules.rs:176:13
        //     |
        // 176 |         use std::io::{self, Write};
        //     |             ^^^ ambiguous name
        //     |
        //     = note: ambiguous because of multiple potential import sources
        //     = note: `std` could refer to a built-in crate
        //     = help: use `::std` to refer to this crate unambiguously
        // note: `std` could also refer to the module defined here
        //    --> src/lessons/modules/modules.rs:175:9
        //     |
        // 175 |         mod std {}
        //     |         ^^^^^^^^^^
        use ::std::io::{self, Write};
    }

    // If we want to bring all public items defined in a path into scope, we can specify that path followed by the * glob operator.
    {
        use std::collections::*;
        // Be careful when using the glob operator!
        // Glob can make it harder to tell what names are in scope and where a name used in your program was defined.
    }
}
