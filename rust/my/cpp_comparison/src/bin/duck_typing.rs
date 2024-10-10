fn main() {
    // In Rust, generic functions are being checked for generic type, in this case, `T`.
    // So, type `T` need to have `larger` method implemented.
    // The error, in this case, is a function definition.
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            // if item.larger(largest) {
            //     largest = item;
            // }
            //
            // error[E0599]: no method named `larger` found for reference `&T` in the current scope
            //  --> my/cpp_comparison/src/bin/duck_typing.rs:8:21
            //   |
            // 8 |             if item.larger(largest) {
            //   |                     ^^^^^^ method not found in `&T`
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    //

    // NOTE: Generic type does not mean ANY type.

    // Doesn't work.
    // fn push_one(vec: &mut Vec<_>) {
    //     vec.push(1)
    // }

    // Doesn't work.
    // fn push_one<T>(vec: &mut Vec<T>) {
    //     vec.push(1)
    // }

    // Work.
    fn push_one(vec: &mut Vec<i32>) {
        vec.push(1)
    }

    let mut vec = vec![0];
    push_one(&mut vec);
    dbg!(vec);
}
