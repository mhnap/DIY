use std::mem::{size_of, size_of_val};

/// Why does Rust have [`str`] and [`String`] types?
/// How do they differ?
/// When should you use them?
/// Why str slice coexists with slice?
/// What is the difference between [`String`] and [`Vec`]?

fn main() {
    let s1 = "Hello";
    let s2 = String::from("world!");

    // #1
    {
        let ptr_size = size_of::<usize>();
        assert_eq!(ptr_size, 8);
        {
            // dbg!(s1.as_ptr(), s1.len());

            assert_eq!(size_of_val(&s1), 2 * ptr_size);
            // assert_eq!(size_of::<str>(), 2 * ptr_size);
            assert_eq!(size_of::<&str>(), 2 * ptr_size);
            assert_eq!(size_of::<Box<str>>(), 2 * ptr_size);
        }
        {
            // dbg!(s2.as_ptr(), s2.len(), s2.capacity());

            assert_eq!(size_of_val(&s2), 3 * ptr_size);
            assert_eq!(size_of::<String>(), 3 * ptr_size);
            assert_eq!(size_of::<&String>(), 1 * ptr_size);
        }
        {
            assert_eq!(size_of_val(s1), 5);
            assert_eq!(s1.len(), 5);
            assert_eq!(size_of_val(&s2[..]), 6);
            assert_eq!(s2.len(), 6);
        }
    }

    // #2
    {
        fn check_s(s: &str) -> bool {
            s.contains("ll")
        }

        assert!(check_s(s1));
        assert!(!check_s(&s2));
    }

    // #3
    {
        // let mut s1: &'static mut str = "Hello";
        let mut s1 = s1.to_owned();
        let /*mut*/ s1 = s1.as_mut_str();
        let mut s2 = s2;

        {
            s1.make_ascii_uppercase();
            assert_eq!(s1, "HELLO");

            s2.make_ascii_uppercase();
            assert_eq!(s2, "WORLD!");
        }
        {
            // s1.push(' ');
            s2.push('!');
        }
    }

    // #4
    {
        // let s1 = "Hеllо";
        // s1[0];
        // s2[0];
        // dbg!(&s1[..2]);

        {
            let v1: Vec<char> = s1.chars().collect();
            assert_eq!(v1[0], 'H');
            assert_eq!(v1[3], 'l');

            let v2: Vec<u8> = s1.bytes().collect();
            assert_eq!(v2[0], b'H');
            assert_eq!(v2[3], b'l');

            // dbg!(size_of_val(&s1[..]));
            // dbg!(size_of_val(&v1[..]));
            // dbg!(size_of_val(&v2[..]));
        }
    }

    // #5
    {
        let story = "Once upon a time...";

        let ptr = story.as_ptr();
        let len = story.len();

        // story has nineteen bytes
        assert_eq!(19, len);

        // We can re-build a str out of ptr and len. This is all unsafe because
        // we are responsible for making sure the two components are valid:
        let s = unsafe {
            // First, we build a &[u8]...
            let slice = std::slice::from_raw_parts(ptr, len);

            // ... and then convert that slice into a string slice
            str::from_utf8(slice)
        };

        assert_eq!(s, Ok(story));
    }

    // #6
    {
        // https://www.reddit.com/r/rustjerk/comments/xn58rt/before_and_after/
    }
}
