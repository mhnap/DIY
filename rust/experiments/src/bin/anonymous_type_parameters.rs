// https://www.reddit.com/r/rust/comments/8jfn7z/what_is_the_advantage_of_impl_trait_in_argument/
// https://github.com/rust-lang/rfcs/pull/1951
// https://doc.rust-lang.org/reference/types/impl-trait.html

use std::fmt::Display;

fn main() {
    // Useless syntax.
    fn foo1(d: impl Display) {
        println!("{d}");
    }

    // Normal syntax.
    fn foo2<T: Display>(d: T) {
        println!("{d}");
    }

    foo1(1);
    foo2(1);

    // Cannot use with turbofish.
    // foo1::<u8>(1);
    // error[E0107]: function takes 0 generic arguments but 1 generic argument was supplied
    //   --> src/experiments/test.rs:22:5
    //    |
    // 22 |     foo1::<u8>(1);
    //    |     ^^^^------ help: remove these generics
    //    |     |
    //    |     expected 0 generic arguments
    //    |
    // note: function defined here, with 0 generic parameters
    //   --> src/experiments/test.rs:9:8
    //    |
    // 9  |     fn foo1(d: impl Display) {
    //    |        ^^^^
    //    = note: `impl Trait` cannot be explicitly specified as a generic argument

    foo2::<u8>(1);
}
