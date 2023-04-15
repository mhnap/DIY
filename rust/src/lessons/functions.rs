fn main() {
    // Rust code uses snake case as the conventional style for function and variable names, in which all letters are lowercase and underscores separate words.
    fn some_func() {}
    some_func();

    // Rust doesn’t care where you define your functions, only that they’re defined somewhere in a scope that can be seen by the caller.
    another_func();

    // In function signatures, you must declare the type of each parameter.
    // This is a deliberate decision in Rust’s design: requiring type annotations in function definitions means the compiler almost never needs you to use them elsewhere in the code to figure out what type you mean.
    // The compiler is also able to give more helpful error messages if it knows what types the function expects.
    print_labeled_measurement(5, 'h');

    // Because Rust is an expression-based language, this is an important distinction to understand.

    // Statements are instructions that perform some action and do not return a value.
    let mut y = 6;

    // Statements do not return values. Therefore, you can’t assign a let statement to another variable.
    // Function definitions are also statements
    // let x = (let y = 6);
    // error: expected expression, found `let` statement
    //   --> src/lessons/functions.rs:19:14
    //    |
    // 19 |     let x = (let y = 6);
    //    |              ^^^

    // Assignment do not returns the value of the assignment (it's a statement)
    let mut x = 0;
    // x = y = 0;
    // error[E0308]: mismatched types
    //   --> src/lessons/functions.rs:27:9
    //    |
    // 26 |     let mut x = 0;
    //    |                 - expected due to this value
    // 27 |     x = y = 0;
    //    |         ^^^^^ expected integer, found `()`

    // Calling a function is an expression.
    // Calling a macro is an expression.
    // A new scope block created with curly brackets is an expression.
    let y = {
        let x = 3;
        // Expressions do not include ending semicolons.
        // If you add a semicolon to the end of an expression, you turn it into a statement, and it will then not return a value.
        x + 1
    };

    println!("The value of y is: {y}");

    println!("five = {}", five());
}

fn another_func() {}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> u8 {
    // 5;
    // error[E0308]: mismatched types
    //   --> src/lessons/functions.rs:59:14
    //    |
    // 59 | fn five() -> u8 {
    //    |    ----      ^^ expected `u8`, found `()`
    //    |    |
    //    |    implicitly returns `()` as its body has no tail or `return` expression
    // ...
    // 62 |     5;
    //    |      - help: remove this semicolon to return this value

    // In Rust, the return value of the function is synonymous with the value of the final expression in the block of the body of a function.
    // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.
    5
}
