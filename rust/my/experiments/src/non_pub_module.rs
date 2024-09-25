// Will be warned if unused.
pub fn unused_pub_fn() {}

// Will be warned if unused.
pub(crate) fn unused_pub_crate_fn() {}

// warning: function `unused_pub_fn` is never used
//  --> my/experiments/src/non_pub_module.rs:2:8
//   |
// 2 | pub fn unused_pub_fn() {}
//   |        ^^^^^^^^^^^^^

// warning: function `unused_pub_crate_fn` is never used
//  --> my/experiments/src/non_pub_module.rs:5:15
//   |
// 5 | pub(crate) fn unused_pub_crate_fn() {}
//   |               ^^^^^^^^^^^^^^^^^^^
