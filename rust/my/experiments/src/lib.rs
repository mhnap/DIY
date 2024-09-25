// https://www.reddit.com/r/learnrust/comments/mezhj8/pub_versus_pubcrate_in_a_private_module/
// Commented to not emit warnings all the time.
// pub mod pub_module;
// mod non_pub_module;

// https://rust-lang.github.io/rfcs/2008-non-exhaustive.html
// https://doc.rust-lang.org/reference/attributes/type_system.html#the-non_exhaustive-attribute
// https://www.reddit.com/r/rust/comments/j0ldal/non_exhaustive_got_me_worried_about_the_direction/
pub mod non_exhaustive;
