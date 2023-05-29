// https://doc.rust-lang.org/book/ch08-02-strings.html
// https://doc.rust-lang.org/std/string/struct.String.html

fn main() {
    // We’ll first define what we mean by the term string.
    // Rust has only one string type in the core language, which is the string slice str that is usually seen in its borrowed form &str.

    // The String type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

    // Both types are used heavily in Rust’s standard library, and both String and string slices are UTF-8 encoded.

    // Create empty string.
    let s = String::new();
    dbg!(&s);

    // Often, we’ll have some initial data that we want to start the string with.
    // For that, we use the to_string method, which is available on any type that implements the Display trait, as string literals do.
    let s = "initial contents".to_string();
    dbg!(&s);

    // We can also use the function String::from to create a String from a string literal.
    let s = String::from("initial contents");
    dbg!(&s);

    // Because strings are used for so many things, we can use many different generic APIs for strings, providing us with a lot of options.
    // Some of them can seem redundant, but they all have their place!
    // In this case, String::from and to_string do the same thing, so which you choose is a matter of style and readability.

    //

    // A String can grow in size and its contents can change, just like the contents of a Vec<T>, if you push more data into it.
    // In addition, you can conveniently use the + operator or the format! macro to concatenate String values.

    // We can grow a String by using the push_str method to append a string slice.
    let mut s = String::from("foo");
    s.push_str("bar");
    dbg!(&s);
    // The push_str method takes a string slice because we don’t necessarily want to take ownership of the parameter.

    // The push method takes a single character as a parameter and adds it to the String.
    let mut s = String::from("lo");
    s.push('l');
    dbg!(&s);

    //

    // Often, you’ll want to combine two existing strings.
    // One way to do so is to use the + operator.
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    // dbg!(&s1);
    // error[E0382]: borrow of moved value: `s1`
    //   --> src/lessons/collections/strings.rs:51:10
    //    |
    // 48 |     let s1 = String::from("Hello, ");
    //    |         -- move occurs because `s1` has type `String`, which does not implement the `Copy` trait
    // 49 |     let s2 = String::from("world!");
    // 50 |     let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    //    |              -- value moved here
    // 51 |     dbg!(&s1);
    //    |          ^^^ value borrowed here after move
    //    |
    // help: consider cloning the value if the performance cost is acceptable
    //    |
    // 50 |     let s3 = s1.clone() + &s2; // note s1 has been moved here and can no longer be used
    //    |                ++++++++
    dbg!(&s2);
    dbg!(&s3);

    // The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator.
    // fn add(self, s: &str) -> String

    // First, s2 has an &, meaning that we’re adding a reference of the second string to the first string.
    // This is because of the s parameter in the add function: we can only add a &str to a String; we can’t add two String values together.
    // But wait—the type of &s2 is &String, not &str, as specified in the second parameter to add.
    // The reason we’re able to use &s2 in the call to add is that the compiler can coerce the &String argument into a &str.
    // When we call the add method, Rust uses a deref coercion, which here turns &s2 into &s2[..].
    // Because add does not take ownership of the s parameter, s2 will still be a valid String after this operation.

    // Second, we can see in the signature that add takes ownership of self, because self does not have an &.
    // This means s1 will be moved into the add call and will no longer be valid after that.
    // So although let s3 = s1 + &s2; looks like it will copy both strings and create a new one, this statement actually takes ownership of s1, appends a copy of the contents of s2, and then returns ownership of the result.
    // In other words, it looks like it’s making a lot of copies but isn’t; the implementation is more efficient than copying.

    // If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy.
    // For more complicated string combining, we can instead use the format! macro.
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // The format! macro works like println!, but instead of printing the output to the screen, it returns a String with the contents.
    // The version of the code using format! is much easier to read, and the code generated by the format! macro uses references so that this call doesn’t take ownership of any of its parameters.
    let s = format!("{s1}-{s2}-{s3}");
    dbg!(&s);
    dbg!(&s1);
    dbg!(&s2);
    dbg!(&s3);

    //

    // In many other programming languages, accessing individual characters in a string by referencing them by index is a valid and common operation.
    // However, if you try to access parts of a String using indexing syntax in Rust, you’ll get an error.
    let s1 = String::from("hello");
    // let h = s1[0];
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    //    --> src/lessons/collections/strings.rs:104:13
    //     |
    // 104 |     let h = s1[0];
    //     |             ^^^^^ `String` cannot be indexed by `{integer}`
    //     |
    //     = help: the trait `Index<{integer}>` is not implemented for `String`
    //     = help: the following other types implement trait `Index<Idx>`:
    //               <String as Index<RangeFrom<usize>>>
    //               <String as Index<RangeFull>>
    //               <String as Index<RangeInclusive<usize>>>
    //               <String as Index<RangeTo<usize>>>
    //               <String as Index<RangeToInclusive<usize>>>
    //               <String as Index<std::ops::Range<usize>>>

    // The error and the note tell the story: Rust strings don’t support indexing.
    // But why not? To answer that question, we need to discuss how Rust stores strings in memory.

    let hello = String::from("hello");
    println!("hello len {}", hello.len());
    dbg!(&hello);
    // Different UTF-8 strings have different lengths.
    let хелоу = String::from("хелоу");
    println!("хелоу len {}", хелоу.len());
    dbg!(&хелоу);

    // An index into the string’s bytes will not always correlate to a valid Unicode scalar value.
    // let answer = &хелоу[0];
    // Users generally don’t want the byte value returned, even if the string contains only Latin letters.
    // The answer, then, is that to avoid returning an unexpected value and causing bugs that might not be discovered immediately.
    // Rust doesn’t compile this code at all and prevents misunderstandings early in the development process.

    // Indexing into a string is often a bad idea because it’s not clear what the return type of the string-indexing operation should be: a byte value, a character, a grapheme cluster, or a string slice.
    // If you really need to use indices to create string slices, therefore, Rust asks you to be more specific.
    let s = &хелоу[0..4];
    dbg!(&s);

    // Will panic in runtime, because byte index 1 is not a char boundary.
    // let s = &хелоу[0..1];

    //

    // The best way to operate on pieces of strings is to be explicit about whether you want characters or bytes.

    // For individual Unicode scalar values, use the chars method.
    for c in "хе".chars() {
        println!("{c}");
    }

    // Alternatively, the bytes method returns each raw byte.
    for b in "хе".bytes() {
        println!("{b}");
    }
    // But be sure to remember that valid Unicode scalar values may be made up of more than 1 byte.
}
