// https://www.reddit.com/r/learnrust/comments/mezhj8/pub_versus_pubcrate_in_a_private_module/

// Won't be warned if unused.
pub fn unused_pub() {}

// Will be warned if unused.
pub(crate) fn unused_pub_crate() {}

// mhnap@hp:~/projects/DIY/rust$ cargo check -p lessons --lib
// warning: function `unused_pub_crate` is never used
// --> lessons/src/lib.rs:7:15
//  |
// 7 | pub(crate) fn unused_pub_crate() {}
//  |               ^^^^^^^^^^^^^^^^
//  |
//  = note: `#[warn(dead_code)]` on by default
//
// warning: `lessons` (lib) generated 1 warning
