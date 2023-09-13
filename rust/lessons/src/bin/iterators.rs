// https://doc.rust-lang.org/book/ch13-02-iterators.html

fn main() {
    // The iterator pattern allows you to perform some task on a sequence of items in turn.
    // An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished.
    // When you use iterators, you don’t have to reimplement that logic yourself.

    // In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.
    // For example, the code below creates an iterator over the items in the vector v1 by calling the iter method defined on Vec<T>.
    // This code by itself doesn’t do anything useful.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    dbg!(&v1, &v1_iter);

    // Once we’ve created an iterator, we can use it in a variety of ways.
    // We separate the creation of the iterator from the use of the iterator in the for loop.
    // When the for loop is called using the iterator in v1_iter, each element in the iterator is used in one iteration of the loop, which prints out each value.
    for val in v1_iter {
        println!("Got: {}", val);
    }

    // In languages that don’t have iterators provided by their standard libraries, you would likely write this same functionality by starting a variable at index 0, using that variable to index into the vector to get a value, and incrementing the variable value in a loop until it reached the total number of items in the vector.
    // Iterators handle all that logic for you, cutting down on repetitive code you could potentially mess up.
    // Iterators give you more flexibility to use the same logic with many different kinds of sequences, not just data structures you can index into, like vectors.

    //

    // All iterators implement a trait named Iterator that is defined in the standard library.
    // Implementing the Iterator trait requires to also define an Item type, and this Item type is used in the return type of the next method.
    // In other words, the Item type will be the type returned from the iterator.

    // The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.
    // We can call the next method on iterators directly.
    let mut v1_iter = v1.iter();
    dbg!(v1_iter.next());
    dbg!(v1_iter.next());
    dbg!(v1_iter.next());
    dbg!(v1_iter.next());

    // Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence.
    // In other words, this code consumes, or uses up, the iterator.
    // Each call to next eats up an item from the iterator.
    // We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.

    // Also note that the values we get from the calls to next are immutable references to the values in the vector.
    // The iter method produces an iterator over immutable references.
    // If we want to create an iterator that takes ownership of v1 and returns owned values, we can call into_iter instead of iter.
    // Similarly, if we want to iterate over mutable references, we can call iter_mut instead of iter.

    //

    // Methods that call next are called consuming adaptors, because calling them uses up the iterator.
    // One example is the sum method, which takes ownership of the iterator and iterates through the items by repeatedly calling next, thus consuming the iterator.
    // As it iterates through, it adds each item to a running total and returns the total when iteration is complete.
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();
    let total: i32 = v1_iter.sum();
    dbg!(&total);
    // dbg!(v1_iter); // error[E0382]: use of moved value: `v1_iter`
    // We aren’t allowed to use v1_iter after the call to sum because sum takes ownership of the iterator we call it on.

    //

    // Iterator adaptors are methods defined on the Iterator trait that don’t consume the iterator.
    // Instead, they produce different iterators by changing some aspect of the original iterator.
    // Below code shows an example of calling the iterator adaptor method map, which takes a closure to call on each item as the items are iterated through.
    // The map method returns a new iterator that produces the modified items.
    // The closure here creates a new iterator in which each item from the vector will be incremented by 1.
    let v1: Vec<i32> = vec![1, 2, 3];
    v1.iter().map(|x| x + 1);

    // Note warning.
    // warning: unused `Map` that must be used
    //   --> src/lessons/iterators.rs:70:5
    //    |
    // 70 |     v1.iter().map(|x| x + 1);
    //    |     ^^^^^^^^^^^^^^^^^^^^^^^^
    //    |
    //    = note: iterators are lazy and do nothing unless consumed
    //    = note: `#[warn(unused_must_use)]` on by default
    // help: use `let _ = ...` to ignore the resulting value
    //    |
    // 70 |     let _ = v1.iter().map(|x| x + 1);
    //    |     +++++++

    // The code doesn’t do anything; the closure we’ve specified never gets called.
    // The warning reminds us why: iterator adaptors are lazy, and we need to consume the iterator here.
    // To fix this warning and consume the iterator, we’ll use the collect method.
    // This method consumes the iterator and collects the resulting values into a collection data type.
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    dbg!(v2);

    // Because map takes a closure, we can specify any operation we want to perform on each item.
    // This is a great example of how closures let you customize some behavior while reusing the iteration behavior that the Iterator trait provides.
    // You can chain multiple calls to iterator adaptors to perform complex actions in a readable way.
    // But because all iterators are lazy, you have to call one of the consuming adaptor methods to get results from calls to iterator adaptors.

    //

    // Many iterator adapters take closures as arguments, and commonly the closures we’ll specify as arguments to iterator adapters will be closures that capture their environment.
    #[derive(PartialEq, Debug)]
    struct Shoe {
        size: u32,
        style: String,
    }

    fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
        shoes.into_iter().filter(|s| s.size == shoe_size).collect()
    }

    let shoes = vec![
        Shoe {
            size: 10,
            style: String::from("sneaker"),
        },
        Shoe {
            size: 13,
            style: String::from("sandal"),
        },
        Shoe {
            size: 10,
            style: String::from("boot"),
        },
    ];

    let in_my_size = shoes_in_size(shoes, 10);
    dbg!(&in_my_size);

    // The shoes_in_size function takes ownership of a vector of shoes and a shoe size as parameters.
    // It returns a vector containing only shoes of the specified size.
    // In the body of shoes_in_size, we call into_iter to create an iterator that takes ownership of the vector.
    // Then we call filter to adapt that iterator into a new iterator that only contains elements for which the closure returns true.
    // The closure captures the shoe_size parameter from the environment and compares the value with each shoe’s size, keeping only shoes of the size specified.
    // Finally, calling collect gathers the values returned by the adapted iterator into a vector that’s returned by the function.

    //

    // Most Rust programmers prefer to use the iterator style.
    // It’s a bit tougher to get the hang of at first, but once you get a feel for the various iterator adaptors and what they do, iterators can be easier to understand.
    // Instead of fiddling with the various bits of looping and building new vectors, the code focuses on the high-level objective of the loop.
    // This abstracts away some of the commonplace code so it’s easier to see the concepts that are unique to this code, such as the filtering condition each element in the iterator must pass.

    //

    // https://doc.rust-lang.org/book/ch13-04-performance.html

    // Iterators, although a high-level abstraction, get compiled down to roughly the same code as if you’d written the lower-level code yourself.
    // Iterators are one of Rust’s zero-cost abstractions, by which we mean using the abstraction imposes no additional runtime overhead.
}
