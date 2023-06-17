fn main() {
    // In Rust, generic functions are being checked for generic type, in this case, `T`.
    // So, type `T` need to have `larger` method implemented.
    // The error, in this case, is a function definition.
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];
        for item in list {
            if item.larger(largest) {
                largest = item;
            }
        }
        largest
    }

    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
