// https://www.lurklurk.org/effective-rust/std-traits.html
// Item 10: Familiarize yourself with standard traits

fn main() {
    // Rust encodes key behavioral aspects of its type system in the type system itself, through a collection of fine-grained standard traits that describe those behaviors (see Item 2).
    // Many of these traits will seem familiar to programmers coming from C++, corresponding to concepts such as copy-constructors, destructors, equality and assignment operators, etc.
    // As in C++, it's often a good idea to implement many of these traits for your own types; the Rust compiler will give you helpful error messages if some operation needs one of these traits for your type and it isn't present.

    // # Clone

    // Variables captured by mutable reference never implement `Clone`.

    // #[derive(Clone)]
    // struct WithMutRef<'a> {
    //     mut_ref: &'a mut i32,
    // }
    //     error[E0277]: the trait bound `&mut i32: Clone` is not satisfied
    //     --> others/books/effective_rust/src/bin/item_10.rs:13:9
    //      |
    //   11 |     #[derive(Clone)]
    //      |              ----- in this derive macro expansion
    //   12 |     struct WithMutRef<'a> {
    //   13 |         mut_ref: &'a mut i32,
    //      |         ^^^^^^^^^^^^^^^^^^^^ the trait `Clone` is not implemented for `&mut i32`
    //      |
    //      = help: the trait `Clone` is implemented for `i32`
    //      = note: `Clone` is implemented for `&i32`, but not for `&mut i32`
    //      = note: this error originates in the derive macro `Clone` (in Nightly builds, run with -Z macro-backtrace for more info)

    // Variables captured by immutable reference does implement Clone.

    #[derive(Clone)]
    struct WithRef<'a> {
        immut_ref: &'a i32,
    }

    // Auto-derived implementations will have unnecessary T: Copy and T: Clone bounds.

    // #[derive(Copy, Clone)]
    // struct Generate<T>(fn() -> T);

    // struct NotCloneable;

    // fn generate_not_cloneable() -> NotCloneable {
    //     NotCloneable
    // }

    // Generate(generate_not_cloneable).clone();

    //     error[E0599]: the method `clone` exists for struct `Generate<NotCloneable>`, but its trait bounds were not satisfied
    //     --> others/books/effective_rust/src/bin/item_10.rs:46:38
    //      |
    //   38 |     struct Generate<T>(fn() -> T);
    //      |     ------------------ method `clone` not found for this struct because it doesn't satisfy `Generate<NotCloneable>: Clone`
    //   39 |
    //   40 |     struct NotCloneable;
    //      |     ------------------- doesn't satisfy `NotCloneable: Clone`
    //   ...
    //   46 |     Generate(generate_not_cloneable).clone();
    //      |                                      ^^^^^ method cannot be called on `Generate<NotCloneable>` due to unsatisfied trait bounds
    //      |
    //   note: trait bound `NotCloneable: Clone` was not satisfied
    //     --> others/books/effective_rust/src/bin/item_10.rs:37:20
    //      |
    //   37 |     #[derive(Copy, Clone)]
    //      |                    ^^^^^ unsatisfied trait bound introduced in this `derive` macro
    //   help: consider annotating `NotCloneable` with `#[derive(Clone)]`
    //      |
    //   40 +     #[derive(Clone)]
    //   41 |     struct NotCloneable;
    //      |

    // The bounds are unnecessary because clearly the function itself should be copy- and cloneable even if its return type is not.

    struct Generate<T>(fn() -> T);

    impl<T> Copy for Generate<T> {}

    impl<T> Clone for Generate<T> {
        fn clone(&self) -> Self {
            *self
        }
    }

    struct NotCloneable;

    fn generate_not_cloneable() -> NotCloneable {
        NotCloneable
    }

    Generate(generate_not_cloneable).clone();

    // a.clone_from(&b) is equivalent to a = b.clone() in functionality, but can be overridden to reuse the resources of a to avoid unnecessary allocations.

    let mut a = vec![1, 2, 3];
    let b = a.clone();
    a.clone_from(&b);

    // There is Clippy lint that detects possible optimization.
    // https://rust-lang.github.io/rust-clippy/master/index.html#/assigning_clones

    //

    // # Copy

    // It's possible to have a different `Clone` implementation than `Copy`.

    struct POD {
        integer: i32,
    }

    impl Copy for POD {}

    impl Clone for POD {
        fn clone(&self) -> Self {
            Self { integer: self.integer + 1 }
        }
    }

    let pod = POD { integer: 42 };
    assert_eq!(pod.integer, 42);

    let pod2 = pod;
    assert_eq!(pod2.integer, 42);

    let pod3 = pod.clone();
    assert_eq!(pod3.integer, 43);

    // This is weird to do, so there are clippy lints for such case:
    // https://rust-lang.github.io/rust-clippy/master/index.html#/clone_on_copy
    // https://rust-lang.github.io/rust-clippy/master/index.html#/expl_impl_clone_on_copy
    // https://rust-lang.github.io/rust-clippy/master/index.html#/non_canonical_clone_impl

    // There is rustc lint that detect missing `Copy` when each field is `Copy`.
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#missing-copy-implementations

    // Variables captured by mutable reference never implement `Copy`.

    // #[derive(Copy)]
    // struct Foo<'a> {
    //     ty: &'a mut bool,
    // }
    //     error[E0204]: the trait `Copy` cannot be implemented for this type
    //     --> others/books/effective_rust/src/bin/item_10.rs:136:14
    //      |
    //  136 |     #[derive(Copy)]
    //      |              ^^^^
    //  137 |     struct Foo<'a> {
    //  138 |         ty: &'a mut bool,
    //      |         ---------------- this field does not implement `Copy`
    //      |
    //      = note: this error originates in the derive macro `Copy` (in Nightly builds, run with -Z macro-backtrace for more info)

    // Variables captured by immutable reference does implement `Copy`.

    #[derive(Copy, Clone)]
    struct Foo<'a> {
        ty: &'a bool,
    }

    // Any type implementing `Drop` can’t be `Copy`, because it’s managing some resource besides its own size_of::<T> bytes.

    // #[derive(Copy, Clone)]
    // struct Bar<'a> {
    //     ty: &'a bool,
    // }

    // impl Drop for Bar<'_> {
    //     fn drop(&mut self) {
    //         todo!()
    //     }
    // }

    //     error[E0184]: the trait `Copy` cannot be implemented for this type; the type has a destructor
    //     --> others/books/effective_rust/src/bin/item_10.rs:160:14
    //      |
    //  160 |     #[derive(Copy, Clone)]
    //      |              ^^^^ `Copy` not allowed on types with destructors
    //      |

    //

    // # Default

    // The Default trait defines a default constructor, via a default() method.
    // This trait can be derived for user-defined types, provided that all of the subtypes involved have a Default implementation of their own; if they don't, you'll have to implement the trait manually.
    // Continuing the comparison with C++, notice that a default constructor has to be explicitly triggered—the compiler does not create one automatically.

    //

    // # PartialEq and Eq

    // The PartialEq and Eq traits allow you to define equality for user-defined types.
    // These traits have special significance because if they're present, the compiler will automatically use them for equality (==) checks, similarly to operator== in C++.
    // The default derive implementation does this with a recursive field-by-field comparison.

    // The Eq version is just a marker trait extension of PartialEq that adds the assumption of reflexivity: any type T that claims to support Eq should ensure that x == x is true for any x: T.

    // This is sufficiently odd to immediately raise the question, When wouldn't x == x? The primary rationale behind this split relates to floating point numbers, and specifically to the special "not a number" value NaN (f32::NAN / f64::NAN in Rust).
    // The floating point specifications require that nothing compares equal to NaN, including NaN itself; the PartialEq trait is the knock-on effect of this.

    assert_eq!(1.24, 1.24);
    assert_ne!(f64::NAN, f64::NAN);
    assert_eq!((), ());

    // For user-defined types that don't have any float-related peculiarities, **you should implement Eq whenever you implement PartialEq**.

    // There is clippy lint for this:
    // https://rust-lang.github.io/rust-clippy/master/index.html#derive_partial_eq_without_eq

    // The full Eq trait is also required if you want to use the type as the key in a HashMap (as well as the Hash trait).

    // You should implement PartialEq manually if your type contains any fields that do not affect the item's identity, such as internal caches and other performance optimizations.
    // (Any manual implementation will also be used for Eq if it is defined, because Eq is just a marker trait that has no methods of its own.)

    //

    // # PartialOrd and Ord

    // The ordering traits PartialOrd and Ord allow comparisons between two items of a type, returning Less, Greater, or Equal.
    // The traits require equivalent equality traits to be implemented (PartialOrd requires PartialEq; Ord requires Eq), and the two have to agree with each other (watch out for this with manual implementations in particular).

    // As with the equality traits, the comparison traits have special significance because the compiler will automatically use them for comparison operations (<, >, <=, >=).

    // The default implementation produced by derive compares fields (or enum variants) lexicographically in the order they're defined, so if this isn't correct, you'll need to implement the traits manually (or reorder the fields).

    // Unlike PartialEq, the PartialOrd trait does correspond to a variety of real situations.
    // For example, it could be used to express a subset relationship among collections: {1, 2} is a subset of {1, 2, 4}, but {1, 3} is not a subset of {2, 4}, nor vice versa.

    // However, even if a partial order does accurately model the behavior of your type, **be wary of implementing just PartialOrd and not Ord** (a rare occasion that contradicts the advice in Item 2 to encode behavior in the type system)—it can lead to surprising results:

    // There are clippy lints for this:
    // https://rust-lang.github.io/rust-clippy/master/index.html#/derive_ord_xor_partial_ord
    // https://rust-lang.github.io/rust-clippy/master/index.html#/non_canonical_partial_ord_impl

    // Inherit the `PartialOrd` behavior from `f32`.
    #[derive(PartialOrd, PartialEq)]
    struct Oddity(f32);

    // Input data with NaN values is likely to give unexpected results.
    let x = Oddity(f32::NAN);
    let y = Oddity(f32::NAN);

    // A self-comparison looks like it should always be true, but it may not be.
    if x <= x {
        println!("This line doesn't get executed!");
    }

    // Programmers are also unlikely to write code that covers all possible
    // comparison arms; if the types involved implemented `Ord`, then the
    // second two arms could be combined.
    if x <= y {
        println!("y is bigger"); // Not hit.
    } else if y < x {
        println!("x is bigger"); // Not hit.
    } else {
        println!("Neither is bigger");
    }

    dbg!(x.partial_cmp(&y));
    dbg!(x.eq(&y));

    dbg!(().partial_cmp(&()));

    //

    // # Hash

    // The Hash trait is used to produce a single value that has a high probability of being different for different items.
    // This hash value is used as the basis for hash-bucket-based data structures like HashMap and HashSet; as such, the type of the keys in these data structures must implement Hash (and Eq).

    // Flipping this around, it's essential that the "same" items (as per Eq) always produce the same hash: if x == y (via Eq), then it must always be true that hash(x) == hash(y).
    // **If you have a manual Eq implementation, check whether you also need a manual implementation of Hash** to comply with this requirement.

    // There is clippy lint for this:
    // https://rust-lang.github.io/rust-clippy/master/index.html#/derived_hash_with_manual_eq

    //

    // # Debug and Display

    // The Debug and Display traits allow a type to specify how it should be included in output, for either normal ({} format argument) or debugging purposes ({:?} format argument), roughly analogous to an operator<< overload for iostream in C++.

    // The differences between the intents of the two traits go beyond which format specifier is needed, though:
    // - Debug can be automatically derived, Display can only be manually implemented.
    // - The layout of Debug output may change between different Rust versions. If the output will ever be parsed by other code, use Display.
    // - Debug is programmer-oriented; Display is user-oriented. A thought experiment that helps with this is to consider what would happen if the program was localized to a language that the authors don't speak—Display is appropriate if the content should be translated, Debug if not.

    // As a general rule, **add an automatically generated Debug implementation for your types** unless they contain sensitive information (personal details, cryptographic material, etc.).

    // There is rustc lint for this:
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html#missing-debug-implementations

    struct Baz;

    impl std::fmt::Debug for Baz {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                f.write_str("Baz Debug alternate")?;
            } else {
                f.write_str("Baz Debug")?;
            }
            Ok(())
        }
    }

    println!("{:?}", Baz);
    println!("{:#?}", Baz);

    impl std::fmt::Display for Baz {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if f.alternate() {
                f.write_str("Baz Display alternate")?;
            } else {
                f.write_str("Baz Display")?;
            }
            Ok(())
        }
    }

    println!("{}", Baz);
    println!("{:#}", Baz);

    //

    // Operator Overloads

    // The final category of standard traits relates to operator overloads, where Rust allows various built-in unary and binary operators to be overloaded for user-defined types, by implementing various standard traits from the std::ops module.
    // These traits are not derivable and are typically needed only for types that represent "algebraic" objects, where there is a natural interpretation of these operators.

    // However, experience from C++ has shown that it's best to **avoid overloading operators for unrelated types** as it often leads to code that is hard to maintain and has unexpected performance properties (e.g., x + y silently invokes an expensive O(N) method).

    // To comply with the principle of least astonishment, if you implement any operator overloads, you should **implement a coherent set of operator overloads**.
    // For example, if x + y has an overload (Add), and -y (Neg) does too, then you should also implement x - y (Sub) and make sure it gives the same answer as x + (-y).

    // The items passed to the operator overload traits are moved, which means that non-Copy types will be consumed by default.
    // Adding implementations for &'a MyType can help with this but requires more boilerplate to cover all of the possibilities (e.g., there are 4 = 2 × 2 possibilities for combining reference/non-reference arguments to a binary operator).
}
