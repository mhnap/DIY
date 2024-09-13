fn main() {
    // Clone trait can be used with #[derive] if all fields are Clone.
    // The derived implementation of Clone calls clone on each field.
    #[derive(Debug, Clone)]
    struct TwoStrings {
        string1: String,
        string2: String,
    }

    let two_strings = TwoStrings { string1: "hello".to_string(), string2: "world".to_string() };
    let two_strings_cloned = two_strings.clone();
    dbg!(&two_strings);
    dbg!(&two_strings_cloned);

    // For a generic struct, #[derive] implements Clone conditionally by adding bound Clone on generic parameters.
    #[derive(Debug, Clone)]
    struct TwoThings<T> {
        thing1: T,
        thing2: T,
    }

    let two_ints = TwoThings { thing1: 42, thing2: 43 };
    let two_ints_cloned = two_ints.clone();
    dbg!(&two_ints);
    dbg!(&two_ints_cloned);

    // Not satisfied Clone trait bound in this case.
    struct A;
    let two_a = TwoThings { thing1: A, thing2: A };
    // let two_a_cloned = two_a.clone();
    // error[E0599]: the method `clone` exists for struct `TwoThings<A>`, but its trait bounds were not satisfied
    //   --> src/experiments/generics_with_derived_clone.rs:39:30
    //    |
    // 20 |     struct TwoThings<T> {
    //    |     -------------------
    //    |     |
    //    |     method `clone` not found for this struct
    //    |     doesn't satisfy `TwoThings<A>: Clone`
    // ...
    // 34 |     struct A;
    //    |     -------- doesn't satisfy `A: Clone`
    // ...
    // 39 |     let two_a_cloned = two_a.clone();
    //    |                              ^^^^^ method cannot be called on `TwoThings<A>` due to unsatisfied trait bounds
    //    |
    // note: trait bound `A: Clone` was not satisfied
    //   --> src/experiments/generics_with_derived_clone.rs:19:21
    //    |
    // 19 |     #[derive(Debug, Clone)]
    //    |                     ^^^^^ unsatisfied trait bound introduced in this `derive` macro
    //    = help: items from traits can only be used if the trait is implemented and in scope
    //    = note: the following trait defines an item `clone`, perhaps you need to implement it:
    //            candidate #1: `Clone`
    // help: consider annotating `A` with `#[derive(Clone)]`
    //    |
    // 34 +     #[derive(Clone)]
    // 35 |     struct A;
    //    |
}
