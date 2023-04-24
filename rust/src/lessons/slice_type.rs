fn first_word_index(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

fn first_word_slice(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    s
}

fn main() {
    {
        let mut s = String::from("hello world");
        let word = first_word_index(&s); // word will get the value 5
        s.clear(); // this empties the String, making it equal to ""

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
        dbg!(&s);
        dbg!(&word);

        // Having to worry about the index in word getting out of sync with the data in s is tedious and error prone!
        // Luckily, Rust has a solution to this problem: string slices.
    }

    {
        let s = String::from("hello world");

        // Start from first index
        let slice = &s[0..5];
        dbg!(&slice);
        let slice = &s[..5];
        dbg!(&slice);

        // Include last index
        let slice = &s[6..s.len()];
        dbg!(&slice);
        let slice = &s[6..];
        dbg!(&slice);

        // Start from first and to last
        let slice = &s[0..s.len()];
        dbg!(&slice);
        let slice = &s[..];
        dbg!(&slice);

        // Invalid slices
        // let slice = &s[12..];
        // let slice = &s[..12];
        // dbg!(&slice);

        let mut s = String::from("hello world");
        let word = first_word_slice(&s); // word will get the value 5

        // s.clear(); // this empties the String, making it equal to ""
        // error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
        //   --> src/lessons/slice_type.rs:60:5
        //    |
        // 59 |     let word = first_word_slice(&s); // word will get the value 5
        //    |                                 -- immutable borrow occurs here
        // 60 |     s.clear(); // this empties the String, making it equal to ""
        //    |     ^^^^^^^^^ mutable borrow occurs here
        // ...
        // 65 |     dbg!(&word);
        //    |          ----- immutable borrow later used here

        // word still has the value 5 here, but there's no more string that
        // we could meaningfully use the value 5 with. word is now totally invalid!
        dbg!(&s);
        dbg!(&word);

        // Not only has Rust made our API easier to use, but it has also eliminated an entire class of errors at compile time!
    }

    {
        let s: &str = "Hello, world!";
        dbg!(&s);
        // The type of s here is &str: it’s a slice pointing to that specific point of the binary.
        // This is also why string literals are immutable; &str is an immutable reference.
    }

    {
        // Defining a function to take a string slice instead of a reference to a String makes our API more general and useful without losing any functionality:
        let my_string = String::from("hello world");
        // `first_word` works on slices of `String`s, whether partial or whole
        let word = first_word(&my_string[0..6]);
        dbg!(&word);
        let word = first_word(&my_string[..]);
        dbg!(&word);
        // `first_word` also works on references to `String`s, which are equivalent
        // to whole slices of `String`s
        let word = first_word(&my_string);
        dbg!(&word);

        let my_string_literal = "hello world";
        // `first_word` works on slices of string literals, whether partial or whole
        let word = first_word(&my_string_literal[0..6]);
        dbg!(&word);
        let word = first_word(&my_string_literal[..]);
        dbg!(&word);
        // Because string literals *are* string slices already,
        // this works too, without the slice syntax!
        let word = first_word(my_string_literal);
        dbg!(&word);
    }

    {
        // Just as we might want to refer to part of a string, we might want to refer to part of an array.
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
        // This slice has the type &[i32].
        // It works the same way as string slices do, by storing a reference to the first element and a length.
    }
}

// Rust Slice type is similar to C++ std::string_view and std::span
