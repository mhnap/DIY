use std::mem::size_of_val;

fn main() {
    let vec: Vec<i32> = vec![1, 2, 3];
    dbg!(size_of_val(&vec));

    let boxed_vec: Box<Vec<i32>> = Box::new(vec.clone());
    dbg!(size_of_val(&boxed_vec));

    let boxed_vec_slice: Box<[i32]> = vec.clone().into_boxed_slice();
    dbg!(size_of_val(&boxed_vec_slice));

    let string: String = "hi".to_string();
    dbg!(size_of_val(&string));

    let boxed_string: Box<String> = Box::new(string.clone());
    dbg!(size_of_val(&boxed_string));

    let boxed_string_slice: Box<str> = string.clone().into_boxed_str();
    dbg!(size_of_val(&boxed_string_slice));
}
