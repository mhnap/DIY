// https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

fn main() {
    let shadowed_binding = 1.5;

    // Save shadowed to reference
    let shadowed_ref = &shadowed_binding;
    {
        println!("before being shadowed: {shadowed_binding}");

        // This binding *shadows* the outer one
        let shadowed_binding = "abc";
        println!("shadowed in inner block: {shadowed_binding}");
    }
    println!("outside inner block: {shadowed_binding}");

    // This binding *shadows* the previous binding
    let mut shadowed_binding = shadowed_binding as i64;
    println!("shadowed in outer block: {shadowed_binding}");

    shadowed_binding = 5;
    println!("mutated shadowed in outer block: {shadowed_binding}");

    // This binding *shadows* the previous mutable binding with new immutable one
    let shadowed_binding = shadowed_binding;
    println!("immutable shadowed in outer block: {shadowed_binding}");
    // shadowed_binding = 5; // Error

    // But we still have access to first shadowed through reference
    println!("shadowed reference: {shadowed_ref}");
}

// Differences:
// - can shadow variable even in the same scope
//
// Similarities:
// - can shadow variables in different scopes
//
// Pros:
// - useful than a couple of variables represent the same abstraction, but collide with names, so one name sometimes is a better choice
//   examples could be: type conversions; type casting; changing from mutable to immutable and vice-versa; when ownership is moved
//
// Cons:
// - can introduce some unclear bugs??
//
// Notes:
// - in C++, shadowing is considered bad, and better to avoid by enabling compiler flag (-Wshadow) - https://www.learncpp.com/cpp-tutorial/variable-shadowing-name-hiding
// - in Rust, shadowing is considered good, and can be used in some cases - https://www.reddit.com/r/rust/comments/xx6ibp/what_is_the_logic_behind_shadowing
//                                                                          https://users.rust-lang.org/t/newbie-question-memory-leaks-by-shadowing/9347
//                                                                          https://users.rust-lang.org/t/what-is-the-usage-of-variable-shadowing/41280
//                                                                          https://www.reddit.com/r/rust/comments/yduaan/shadowing_variable_vs_mut_variable/
//                                       about performance with shadowing - https://zakuarbor.github.io/blog/rust-shadow-vs-mut/
// - still, it can be disabled also in Rust - https://rust-lang.github.io/rust-clippy/master/#shadow
// - good to think "Am I changing x or replacing x?" whether to decide to use mut or shadowing
