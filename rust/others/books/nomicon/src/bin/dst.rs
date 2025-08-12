// https://doc.rust-lang.org/nomicon/exotic-sizes.html#dynamically-sized-types-dsts
// https://users.rust-lang.org/t/solved-initialize-an-unsized-struct/88140
// https://users.rust-lang.org/t/what-does-the-second-field-of-a-fat-pointer-of-struct-dyn-trait-point-to/65653
// https://stackoverflow.com/questions/78395488/why-in-rust-struct-only-last-field-is-allowed-to-have-dynamically-sized-type
// https://stackoverflow.com/questions/30938499/why-is-the-sized-bound-necessary-in-this-trait
// https://www.reddit.com/r/rust/comments/7q3bz8/trait_object_with_clone

struct MySuperSliceable<T: ?Sized> {
    info: u32,
    data: T,
    // dummy: i32,
    // error[E0277]: the size for values of type `T` cannot be known at compilation time
    //  --> others/books/nomicon/src/bin/dst.rs:8:11
    //   |
    // 6 | struct MySuperSliceable<T: ?Sized> {
    //   |                         - this type parameter needs to be `Sized`
    // 7 |     info: u32,
    // 8 |     data: T,
    //   |           ^ doesn't have a size known at compile-time
    //   |
    //   = note: only the last field of a struct may have a dynamically sized type
    //   = help: change the field's type to have a statically known size
    // help: consider removing the `?Sized` bound to make the type parameter `Sized`
    //   |
    // 6 - struct MySuperSliceable<T: ?Sized> {
    // 6 + struct MySuperSliceable<T> {
    //   |
    // help: borrowed types always have a statically known size
    //   |
    // 8 |     data: &T,
    //   |           +
    // help: the `Box` type always has a statically known size and allocates its contents in the heap
    //   |
    // 8 |     data: Box<T>,
    //   |           ++++ +
}

fn main() {
    let sized: MySuperSliceable<[u8; 6]> = MySuperSliceable { info: 17, data: [0; 6] };

    let dynamic: &MySuperSliceable<[u8]> = &sized;

    // prints: "17 [0, 0, 0, 0, 0, 0, 0, 0]"
    println!("{} {:?}", dynamic.info, &dynamic.data);
    assert_eq!(16, size_of_val(&dynamic));
    assert_eq!(8, align_of_val(&dynamic));
    assert_eq!(12, size_of_val(dynamic));
    assert_eq!(4, align_of_val(dynamic));
}
