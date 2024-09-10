// https://users.rust-lang.org/t/use-case-for-box-str-and-string/8295
// https://www.reddit.com/r/rust/comments/174ndzi/fun_fact_size_of_optionstring
// https://users.rust-lang.org/t/does-bool-save-space-over-u8/75250
// https://stackoverflow.com/questions/73180983/why-is-the-size-of-optionbool-equal-to-1
// https://www.reddit.com/r/rust/comments/wqsxk2/is_it_better_to_pass_optiont_or_optiont/

use std::{mem::size_of_val, num::NonZero};

fn main() {
    dbg!(size_of::<i32>());
    dbg!(size_of::<Option<i32>>());
    dbg!(size_of::<&i32>());
    dbg!(size_of::<Option<&i32>>());
    dbg!(size_of::<bool>());
    dbg!(size_of::<Option<bool>>());
    dbg!(size_of::<NonZero<i32>>());
    dbg!(size_of::<Option<NonZero<i32>>>());
    dbg!(size_of::<Vec<i32>>());
    dbg!(size_of::<Option<Vec<i32>>>());
    dbg!(size_of::<Box<Vec<i32>>>());
    dbg!(size_of::<Option<Box<Vec<i32>>>>());
    dbg!(size_of::<Box<[i32]>>());
    dbg!(size_of::<Option<Box<[i32]>>>());
    dbg!(size_of::<String>());
    dbg!(size_of::<Option<String>>());
    dbg!(size_of::<Box<String>>());
    dbg!(size_of::<Option<Box<String>>>());
    dbg!(size_of::<Box<str>>());
    dbg!(size_of::<Option<Box<str>>>());
    struct A;
    dbg!(size_of::<A>());
    dbg!(size_of::<Option<A>>());
    struct B {
        byte: u8,
    }
    dbg!(size_of::<B>());
    dbg!(size_of::<Option<B>>());

    //

    dbg!(true as u8);
    dbg!(false as u8);
    dbg!(unsafe { std::mem::transmute::<_, u8>(Some(false)) });
    dbg!(unsafe { std::mem::transmute::<_, u8>(Some(true)) });
    dbg!(unsafe { std::mem::transmute::<Option<bool>, u8>(None) });
}
