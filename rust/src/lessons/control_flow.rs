// https://doc.rust-lang.org/stable/book/ch03-05-control-flow.html

fn main() {
    let number = 3;

    // The condition must be a bool
    // if number {
    // error[E0308]: mismatched types
    //  --> src/lessons/control_flow.rs:5:8
    //   |
    // 5 |     if number {
    //   |        ^^^^^^ expected `bool`, found integer
    if number == 3 {
        println!("number was three");
    }

    // Because if is an expression, we can use it on the right side of a let statement to assign the outcome to a variable.
    let three = if number == 4 { 4 } else { 3 };
    dbg!(three);

    // Must be the same types
    // let four = if number == 4 {4} else {"3"};
    // error[E0308]: `if` and `else` have incompatible types
    //   --> src/lessons/control_flow.rs:19:41
    //    |
    // 19 |     let four = if number == 4 {4} else {"3"};
    //    |                                -        ^^^ expected integer, found `&str`
    //    |                                |
    //    |                                expected because of this

    // Can return value from loop with break
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // Can optionally specify a loop label on a loop that you can then use with break or continue to specify that those keywords apply to the labeled loop instead of the innermost loop.
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;
        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");

    // Regular while loop
    let mut number = 3;
    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // However, this approach is error prone; we could cause the program to panic if the index value or test condition is incorrect.
    // It’s also slow, because the compiler adds runtime code to perform the conditional check of whether the index is within the bounds of the array on every iteration through the loop.
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;
    while index < 5 {
        println!("the value is: {}", a[index]);
        index += 1;
    }

    // As a more concise alternative, you can use a for loop.
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {element}");
    }

    // Can use a range expression
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
