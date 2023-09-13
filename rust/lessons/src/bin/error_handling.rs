use std::error::Error;
use std::fs::{self, File};
use std::io::{self, Read, ErrorKind};
use std::process::{ExitCode, Termination};

// By default main return unit type ().
// fn main() {

// And main can only return types that implement `Termination` as this error states.
// fn main() -> i32 {
// error[E0277]: `main` has invalid return type `i32`
//  --> src/lessons/error_handling.rs:4:14
//   |
// 4 | fn main() -> i32 {
//   |              ^^^ `main` can only return types that implement `Termination`
//   |
//   = help: consider using `()`, or a `Result`

struct Term {
    pub exit_code: ExitCode,
}

impl Termination for Term {
    fn report(self) -> ExitCode {
        self.exit_code
    }
}

// `Term` type can be used in main return because it implement `Termination`.
// fn main() -> Term {

// Result type can also be used as main return type.
fn main() -> Result<(), Box<dyn Error>> {
    // https://doc.rust-lang.org/book/ch09-00-error-handling.html
    // Rust groups errors into two major categories: recoverable and unrecoverable errors.
    // For a recoverable error, such as a file not found error, we most likely just want to report the problem to the user and retry the operation.
    // Unrecoverable errors are always symptoms of bugs, like trying to access a location beyond the end of an array, and so we want to immediately stop the program.

    // Rust doesn’t have exceptions.
    // Instead, it has the type Result<T, E> for recoverable errors and the panic! macro that stops execution when the program encounters an unrecoverable error.

    //

    // https://doc.rust-lang.org/book/ch09-01-unrecoverable-errors-with-panic.html#unwinding-the-stack-or-aborting-in-response-to-a-panic
    // Sometimes, bad things happen in your code, and there’s nothing you can do about it.
    // In these cases, Rust has the panic! macro.
    // There are two ways to cause a panic in practice: by taking an action that causes our code to panic (such as accessing an array past the end) or by explicitly calling the panic! macro.
    // In both cases, we cause a panic in our program.
    // By default, these panics will print a failure message, unwind, clean up the stack, and quit.
    // Via an environment variable, you can also have Rust display the call stack when a panic occurs to make it easier to track down the source of the panic.
    // Default `unwind` mode when panicking can be changed to `abort` mode in profile section in Cargo.toml file.

    // Will call drop during stack unwinding.
    struct A {}
    impl Drop for A {
        fn drop(&mut self) {
            println!("Drop A!");
        }
    }
    let a = A {};

    // Just panic!
    // panic!("crash and burn");

    // Attempts to access an index in a vector beyond the range of valid indexes will cause panic!
    let v = vec![1, 2, 3];
    // v[99];

    // In order to get backtraces with this information, debug symbols must be enabled.
    // Debug symbols are enabled by default when using cargo build or cargo run without the --release flag, as we have here.

    //

    // https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html
    // Most errors aren’t serious enough to require the program to stop entirely.
    // Sometimes, when a function fails, it’s for a reason that you can easily interpret and respond to.
    // For example, if you try to open a file and that operation fails because the file doesn’t exist, you might want to create the file instead of terminating the process.
    let greeting_file_result = File::open("hello.txt");

    // Handle Result value by panicking on Error.
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // Handle Result value by creating a new file if the file doesn't exist, else still panic.
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        }
    };
    dbg!(greeting_file);

    // Using match works well enough, but it can be a bit verbose and doesn’t always communicate intent well.
    // The Result<T, E> type has many helper methods defined on it to do various, more specific tasks.

    // Can use `unwrap_or_else` method instead of `match` expressions.
    let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });

    // Can use `unwrap` method to get value or panic.
    let greeting_file = File::open("hello.txt").unwrap();

    // Similarly, the `expect` method lets us also choose the panic! error message.
    // Using `expect` instead of `unwrap` and providing good error messages can convey your intent and make tracking down the source of a panic easier.
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");

    // In production-quality code, most Rustaceans choose expect rather than unwrap and give more context about why the operation is expected to always succeed.
    // That way, if your assumptions are ever proven wrong, you have more information to use in debugging.


    // When a function’s implementation calls something that might fail, instead of handling the error within the function itself, you can return the error to the calling code so that it can decide what to do.
    // This is known as `propagating` the error and gives more control to the calling code, where there might be more information or logic that dictates how the error should be handled than what you have available in the context of your code.

    // Open file and read string from it.
    fn read_username_from_file_v1() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
    let username = read_username_from_file_v1();
    dbg!(username);

    // This pattern of propagating errors is so common in Rust that Rust provides the question mark operator `?` to make this easier.
    fn read_username_from_file_v2() -> Result<String, io::Error> {
        let mut username_file = File::open("hello.txt")?;
        let mut username = String::new();
        username_file.read_to_string(&mut username)?;
        Ok(username)
    }
    let username = read_username_from_file_v2();
    dbg!(username);

    // There is a difference between what the `match` expression does and what the `?` operator does:
    // error values that have the `?` operator called on them go through the `from` function, defined in the `From` trait in the standard library, which is used to convert values from one type into another.
    // When the `?` operator calls the `from` function, the error type received is converted into the error type defined in the return type of the current function.
    // This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.

    // More concise version.
    fn read_username_from_file_v3() -> Result<String, io::Error> {
        let mut username = String::new();
        File::open("hello.txt")?.read_to_string(&mut username)?;
        Ok(username)
    }
    let username = read_username_from_file_v3();
    dbg!(username);

    // And the shortest one using standard library `read_to_string` function.
    fn read_username_from_file_v4() -> Result<String, io::Error> {
        fs::read_to_string("hello.txt")
    }
    let username = read_username_from_file_v4();
    dbg!(username);

    // Cannot use `?` operator with incompatible Err type.
    // let greeting_file = File::open("hello.txt")?;
    // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
    //    --> src/lessons/error_handling.rs:149:48
    //     |
    // 4   | fn main() {
    //     | --------- this function should return `Result` or `Option` to accept `?`
    // ...
    // 149 |     let greeting_file = File::open("hello.txt")?;
    //     |                                                ^ cannot use the `?` operator in a function that returns `()`
    //     |
    //     = help: the trait `FromResidual<Result<Infallible, std::io::Error>>` is not implemented for `()`

    // The error message also mentioned that ? can be used with Option<T> values as well.
    // The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point.
    // If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
    fn last_char_of_first_line(text: &str) -> Option<char> {
        text.lines().next()?.chars().last()
    }
    match last_char_of_first_line("") {
        Some(ch) => println!("last char of first line is {}", ch),
        None => println!("no last char of first line"),
    }
    match last_char_of_first_line("abc\ndfg") {
        Some(ch) => println!("last char of first line is {}", ch),
        None => println!("no last char of first line"),
    }
    match last_char_of_first_line("\ndfg") {
        Some(ch) => println!("last char of first line is {}", ch),
        None => println!("no last char of first line"),
    }

    // Note that you can use the ? operator on a Result in a function that returns Result, and you can use the ? operator on an Option in a function that returns Option, but you can’t mix and match.
    // The ? operator won’t automatically convert a Result to an Option or vice versa; in those cases, you can use methods like the ok method on Result or the ok_or method on Option to do the conversion explicitly.


    // The main function is special because it’s the entry and exit point of executable programs, and there are restrictions on what its return type can be for the programs to behave as expected.
    // The main function may return any types that implement the std::process::Termination trait, which contains a function report that returns an ExitCode.

    // We can propagate `std::io::Error` here because main return trait object of type `Error` for `Err` case.
    let greeting_file = File::open("hello.txt")?;
    dbg!(greeting_file);
    Ok(())

    //

    // https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html
    // So how do you decide when you should call panic! and when you should return Result?
    // When code panics, there’s no way to recover.
    // You could call panic! for any error situation, whether there’s a possible way to recover or not, but then you’re making the decision that a situation is unrecoverable on behalf of the calling code.
    // When you choose to return a Result value, you give the calling code options.
    // The calling code could choose to attempt to recover in a way that’s appropriate for its situation, or it could decide that an Err value in this case is unrecoverable, so it can call panic! and turn your recoverable error into an unrecoverable one.
    // Therefore, returning Result is a good default choice when you’re defining a function that might fail.

    // More guidelines for error handling can be found in this chapter of The Book.
}
