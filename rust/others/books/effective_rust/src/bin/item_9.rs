// https://www.lurklurk.org/effective-rust/iterators.html
// Item 9: Consider using iterator transforms instead of explicit loops

fn main() {
    // The humble loop has had a long journey of increasing convenience and increasing abstraction.
    // The B language (the precursor to C) had only while (condition) { ... }, but with the arrival of C, the common scenario of iterating through indexes of an array became more convenient with the addition of the for loop:

    // C code
    // int i;
    // for (i = 0; i < len; i++) {
    //   Item item = collection[i];
    //   // body
    // }

    // The early versions of C++ further improved convenience and scoping by allowing the loop variable declaration to be embedded in the for statement (this was also adopted by C in C99):

    // C++98 code
    // for (int i = 0; i < len; i++) {
    //     Item item = collection[i];
    //     // ...
    // }

    // Most modern languages abstract the idea of the loop further: the core function of a loop is often to move to the next item of some container.
    // Tracking the logistics that are required to reach that item (index++ or ++it) is mostly an irrelevant detail.
    // This realization produced two core concepts:
    // - Iterators: A type whose purpose is to repeatedly emit the next item of a container, until exhausted;
    // - For-each loops: A compact loop expression for iterating over all of the items in a container, binding a loop variable to the item rather than to the details of reaching that item;

    // These concepts allow for loop code that's shorter and (more importantly) clearer about what's intended:

    // C++11 code
    // for (Item& item : collection) {
    //     // ...
    // }

    // Once these concepts were available, they were so obviously powerful that they were quickly retrofitted to those languages that didn't already have them (e.g., for-each loops were added to Java 1.5 and C++11).

    // Rust includes iterators and for-each–style loops, but it also includes the next step in abstraction: allowing the whole loop to be expressed as an iterator transform (sometimes also referred to as an iterator adaptor).
    // As with Item 3's discussion of Option and Result, this Item will attempt to show how these iterator transforms can be used instead of explicit loops, and will give guidance as to when it's a good idea.
    // In particular, iterator transforms can be more efficient than an explicit loop, because the compiler can skip the bounds checks it might otherwise need to perform.

    // By the end of this Item, a C-like explicit loop to sum the squares of the first five even items of a vector:

    let values: Vec<u64> = vec![1, 1, 2, 3, 5 /* ... */];

    let mut even_sum_squares = 0;
    let mut even_count = 0;
    for i in 0..values.len() {
        if values[i] % 2 != 0 {
            continue;
        }
        even_sum_squares += values[i] * values[i];
        even_count += 1;
        if even_count == 5 {
            break;
        }
    }

    dbg!(even_sum_squares);

    // should start to feel more natural expressed as a functional-style expression:

    let even_sum_squares: u64 = values.iter().filter(|x| *x % 2 == 0).take(5).map(|x| x * x).sum();

    dbg!(even_sum_squares);

    // Iterator transformation expressions like this can roughly be broken down into three parts:
    // - An initial source iterator, from an instance of a type that implements one of Rust's iterator traits;
    // - A sequence of iterator transforms;
    // - A final consumer method to combine the results of the iteration into a final value;

    // The first two of these parts effectively move functionality out of the loop body and into the for expression; the last removes the need for the for statement altogether.
}
