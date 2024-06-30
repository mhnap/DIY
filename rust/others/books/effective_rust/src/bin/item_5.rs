// https://www.lurklurk.org/effective-rust/casts.html
// Item 5: Understand type conversions

fn main() {
    // Rust type conversions fall into three categories:
    // Manual: User-defined type conversions provided by implementing the From and Into traits
    // Semi-automatic: Explicit casts between values using the as keyword
    // Automatic: Implicit coercion into a new type

    // Note that in contrast to many older languages, Rust does not perform automatic conversion between numeric types.
    // This even applies to "safe" transformations of integral types:
    // let x: u32 = 2;
    // let y: u64 = x;
    //     error[E0308]: mismatched types
    //     --> others/books/effective_rust/src/bin/item_5.rs:13:14
    //      |
    //   13 | let y: u64 = x;
    //      |        ---   ^ expected `u64`, found `u32`
    //      |        |
    //      |        expected due to this
    //      |
    //   help: you can convert a `u32` to a `u64`
    //      |
    //   13 | let y: u64 = x.into();
    //      |               +++++++

    // The first piece of advice is therefore to **implement (just) the Try... trait if it's possible for a conversion to fail**, in line with Item 4.
    // The alternative is to ignore the possibility of error (e.g., with .unwrap()), but that needs to be a deliberate choice, and in most cases it's best to leave that choice to the caller.

    // The type conversion traits have an obvious symmetry: if a type T can be transformed into a type U (via Into<U>), isn't that the same as it being possible to create an item of type U by transforming from an item of type T (via From<T>)?

    // This is indeed the case, and it leads to the second piece of advice: **implement the From trait for conversions**.
    // The Rust standard library had to pick just one of the two possibilities, in order to prevent the system from spiraling around in dizzy circles (more properly known as the trait coherence rules), and it came down on the side of automatically providing Into from a From implementation.

    // If you're consuming one of these two traits, as a trait bound on a new generic of your own, then the advice is reversed: **use the Into trait for trait bounds**.
    // That way, the bound will be satisfied both by things that directly implement Into and by things that only directly implement From.

    // This automatic conversion is highlighted by the documentation for From and Into, but it's worth reading the relevant part of the standard library code too, which is a blanket trait implementation:
    // impl<T, U> Into<U> for T
    // where
    //     U: From<T>,
    // {
    //     fn into(self) -> U {
    //         U::from(self)
    //     }
    // }

    // The TryFrom trait also has a blanket implementation for any type that already implements the Into trait in the opposite direction—which automatically includes (as shown previously) any type that implements From in the same direction.
    // In other words, if you can infallibly convert a T into a U, you can also fallibly obtain a U from a T; as this conversion will always succeed, the associated error type is the helpfully named Infallible.

    let u_8 = 1u8;
    let u_16: u16 = u_8.into();
    dbg!(u_16);
    let u_16 = u16::try_from(u_8).unwrap();
    dbg!(u_16);

    // There's also one very specific generic implementation of From that sticks out, the reflexive implementation:
    // impl<T> From<T> for T {
    //     fn from(t: T) -> T {
    //         t
    //     }
    // }

    // Translated into words, this just says that “given a T, I can get a T.”
    // That's such an obvious "well, duh" that it's worth stopping to understand why this is useful.

    /// Integer value from an IANA-controlled range.
    #[derive(Clone, Copy, Debug)]
    pub struct IanaAllocated(pub u64);

    /// Indicate whether value is reserved.
    pub fn is_iana_reserved(s: IanaAllocated) -> bool {
        s.0 == 0 || s.0 == 65535
    }

    let s = IanaAllocated(1);
    println!("{:?} reserved? {}", s, is_iana_reserved(s));
    // output: "IanaAllocated(1) reserved? false"

    impl From<u64> for IanaAllocated {
        fn from(v: u64) -> Self {
            Self(v)
        }
    }

    // if is_iana_reserved(42) {
    //     // ...
    // }
    //     error[E0308]: mismatched types
    //   --> others/books/effective_rust/src/bin/item_5.rs:86:25
    //    |
    // 86 |     if is_iana_reserved(42) {
    //    |        ---------------- ^^ expected `IanaAllocated`, found integer
    //    |        |
    //    |        arguments to this function are incorrect
    //    |
    // note: function defined here
    //   --> others/books/effective_rust/src/bin/item_5.rs:72:12
    //    |
    // 72 |     pub fn is_iana_reserved(s: IanaAllocated) -> bool {
    //    |            ^^^^^^^^^^^^^^^^ ----------------
    // help: try wrapping the expression in `main::IanaAllocated`
    //    |
    // 86 |     if is_iana_reserved(main::IanaAllocated(42)) {
    //    |                         ++++++++++++++++++++  +

    pub fn is_iana_reserved_v2<T>(s: T) -> bool
    where
        T: Into<IanaAllocated>,
    {
        let s = s.into();
        s.0 == 0 || s.0 == 65535
    }

    if is_iana_reserved_v2(42) {
        // ...
    }

    let s = IanaAllocated(1);
    println!("{:?} reserved? {}", s, is_iana_reserved_v2(s));
    // output: "IanaAllocated(1) reserved? false"

    // With this trait bound in place, the reflexive trait implementation of From<T> makes more sense: it means that the generic function copes with items that are already IanaAllocated instances, no conversion needed.

    //

    // Rust includes the as keyword to perform explicit casts between some pairs of types.

    // The pairs of types that can be converted in this way constitute a fairly limited set, and the only user-defined types it includes are "C-like" enums (those that have just an associated integer value).
    // General integral conversions are included, though, giving an alternative to into():

    let x: u32 = 9;
    let y = x as u64;
    let z: u64 = x.into();

    // The as version also allows lossy conversions:
    let x: u32 = 9;
    let y = x as u16;

    // which would be rejected by the from/into versions:
    // let y: u16 = x.into();
    // error[E0277]: the trait bound `u16: From<u32>` is not satisfied
    //    --> others/books/effective_rust/src/bin/item_5.rs:134:16
    //     |
    // 134 | let y: u16 = x.into();
    //     |                ^^^^ the trait `From<u32>` is not implemented for `u16`, which is required by `u32: Into<_>`
    //     |
    //     = help: the following other types implement trait `From<T>`:
    //               <u16 as From<bool>>
    //               <u16 as From<u8>>
    //               <u16 as From<Char>>
    //     = note: required for `u32` to implement `Into<u16>`

    // Allowing lossy conversions in Rust was probably a mistake, and there have been discussions around trying to remove this behavior.
    // https://internals.rust-lang.org/t/lets-deprecate-as-for-lossy-numeric-casts/16283

    // For consistency and safety, you should **prefer from/into conversions over as casts**, unless you understand and need the precise casting semantics (e.g., for C interoperability).
    // This advice can be reinforced by Clippy (Item 29), which includes several lints about as conversions; however, these lints are disabled by default.
    // https://rust-lang.github.io/rust-clippy/stable/index.html#/as_conversions
}
