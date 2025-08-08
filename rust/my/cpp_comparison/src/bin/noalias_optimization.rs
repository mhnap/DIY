// Rust guarantee that there is only one mutable reference or any immutable references at once.
// So, this rule gives some nice optimization possibilities.

// This is a simple example of such optimization.
fn add(a: &mut i32, b: &i32) {
    *a += *b;
    *a += *b;
}
// Above function produces the following assembly with optimizations turned on.
// example::add:
//         mov     eax, dword ptr [rsi]
//         add     eax, eax
//         add     dword ptr [rdi], eax
//         ret
// Here, we know that `a` cannot alias `b`, so we read `b` only once.

fn main() {
    let mut a = 42;
    // Aliasing causes a compiler error.
    // add(&mut a, &a);
    // error[E0502]: cannot borrow `a` as immutable because it is also borrowed as mutable
    //   --> src/cpp_comparison/noalias_optimization.rs:21:17
    //    |
    // 21 |     add(&mut a, &a);
    //    |     --- ------  ^^ immutable borrow occurs here
    //    |     |   |
    //    |     |   mutable borrow occurs here
    //    |     mutable borrow later used by call

    let b = 1;
    // No problem because we use two different values.
    add(&mut a, &b);
    dbg!(a, b);
}

// More can be read here:
// https://doc.rust-lang.org/nomicon/aliasing.html
// https://llvm.org/docs/LangRef.html#noalias
// https://stackoverflow.com/questions/57259126/why-does-the-rust-compiler-not-optimize-code-assuming-that-two-mutable-reference

// Pros:
// - gives more optimizations possibilities
