// https://doc.rust-lang.org/book/ch08-01-vectors.html
// https://doc.rust-lang.org/std/vec/struct.Vec.html

fn main() {
    // Rust’s standard library includes a number of very useful data structures called collections.
    // Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

    //

    // Creating empty vector.
    let v: Vec<i32> = Vec::new();
    dbg!(&v);

    // Creating vector with vec macro.
    let v = vec![1, 2, 3];
    dbg!(&v);

    // Updating mutable vector.
    let mut v = Vec::new();
    v.push(4);
    v.push(5);
    // The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.

    // Get value via indexing.
    let second: &i32 = &v[1];
    println!("The second element is {second}");

    // Get value via `get` method.
    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // Attempting to add an element to a vector while holding a reference to an item.
    let first = &v[0];
    // v.push(6);
    // error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
    //   --> src/lessons/collections.rs:34:5
    //    |
    // 33 |     let first = &v[0];
    //    |                  - immutable borrow occurs here
    // 34 |     v.push(6);
    //    |     ^^^^^^^^^ mutable borrow occurs here
    // 35 |     println!("The first element is: {first}");
    //    |                                      ----- immutable borrow later used here
    println!("The first element is: {first}");

    // Ownership is moved into vector.
    let mut v = Vec::new();
    let s = "hello".to_owned();
    v.push(s);
    dbg!(&v);
    // dbg!(&s);
    // error[E0382]: borrow of moved value: `s`
    //   --> src/lessons/collections/vectors.rs:53:10
    //    |
    // 50 |     let s = "hello".to_owned();
    //    |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
    // 51 |     v.push(s);
    //    |            - value moved here
    // 52 |     dbg!(&v);
    // 53 |     dbg!(&s);
    //    |          ^^ value borrowed here after move
    //    |
    // help: consider cloning the value if the performance cost is acceptable
    //    |
    // 51 |     v.push(s.clone());
    //    |             ++++++++

    // Printing each element in a vector by iterating over the elements using a for loop.
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    // Iterating over mutable references to elements in a vector.
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }

    // Iterating over a vector, whether immutably or mutably, is safe because of the borrow checker's rules.
    // The reference to the vector that the for loop holds prevents simultaneous modification of the whole vector.
    for i in &mut v {
        // v.push(1);
        // error[E0499]: cannot borrow `v` as mutable more than once at a time
        //   --> src/lessons/collections/vectors.rs:63:9
        //    |
        // 62 |     for i in &mut v {
        //    |              ------
        //    |              |
        //    |              first mutable borrow occurs here
        //    |              first borrow later used here
        // 63 |         v.push(1);
        //    |         ^^^^^^^^^ second mutable borrow occurs here
    }

    //

    // Vectors can only store values that are the same type.
    // This can be inconvenient; there are definitely use cases for needing to store a list of items of different types.
    // Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    dbg!(&row);

    // Like any other struct, a vector is freed when it goes out of scope.
    // When the vector gets dropped, all of its contents are also dropped, meaning the integers it holds will be cleaned up.
    // The borrow checker ensures that any references to contents of a vector are only used while the vector itself is valid.
    {
        let v = vec![1, 2, 3, 4];
        // do stuff with v
    } // <- v goes out of scope and is freed here
}
