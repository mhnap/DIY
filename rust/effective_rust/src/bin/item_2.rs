// https://www.lurklurk.org/effective-rust/use-types-2.html
// Item 2: Use the type system to express common behaviour

fn main() {
    // Returning to the rough mental model of closures above, which of the traits the compiler auto-implements roughly corresponds to whether the captured environmental context has:
    // FnOnce: any moved values
    // FnMut: any mutable references to values (&mut T)
    // Fn: only normal references to values (&T).
    // The latter two traits in the list above each has a trait bound of the preceding trait, which makes sense when you consider the things that use the closures.

    // If something only expects to call a closure once (indicated by receiving a FnOnce), it's OK to pass it a closure that's capable of being repeatedly called (FnMut).
    // If something expects to repeatedly call a closure that might mutate its environment (indicated by receiving a FnMut), it's OK to pass it a closure that doesn't need to mutate its environment (Fn).
    // The bare function pointer type fn also notionally belongs at the end of this list; any (not-unsafe) fn type automatically implements all of the Fn* traits, because it borrows nothing from the environment.

    // As a result, when writing code that accepts closures, **use the most general Fn* trait that works**, to allow the greatest flexibility for callers – for example, accept FnOnce for closures that are only used once.
    // The same reasoning also leads to advice to **prefer Fn* trait bounds to bare function pointers (fn)**.

    //

    // Code that accepts a struct and calls methods on it is constrained to only ever work with that specific type.
    // If there are multiple types that implement common behaviour, then it is more flexible to define a trait that encapsulates that common behaviour, and have the code make use of the trait's methods rather than methods on a specific struct.

    // This leads to the same kind of advice that turns up for other OO-influenced languages: **prefer accepting trait types to concrete types** if future flexibility is anticipated.

    //

    // A marker trait has no methods, but an implementation still has to declare that it is implementing the trait – which acts as a promise from the implementer: "I solemnly swear that my implementation sorts stably".
    // Code that relies on a stable sort can then specify the StableSort trait bound, relying on the honour system to preserve its invariants.
    // **Use marker traits to distinguish behaviours that cannot be expressed in the trait method signatures.**

    //

    // A trait bound indicates that generic code which is parameterized by some type T can only be used when that type T implements some specific trait.
    // The presence of the trait bound means that the implementation of the generic can use the methods from that trait, secure in the knowledge that the compiler will ensure that any T that compiles does indeed have those methods.
    // This check happens at compile-time, when the generic is monomorphized (Rust's term for what C++ would call "template instantiation").

    // This restriction on the target type T is explicit, encoded in the trait bounds: the trait can only be implemented by types that satisfy the trait bounds.
    // This is in contrast to the equivalent situation in C++, where the constraints on the type T used in a template<typename T> are implicit (the addition of concepts in C++20 allows explicit specification of constraints on template types, but the checks are still only performed when the template is instantiated, not when it is declared):
    // C++ template code still only compiles if all of the referenced methods are available at compile-time, but the checks are purely based on method and signature.
    // (This "duck typing" leads to the chance of confusion; a C++ template that uses t.pop() might compile for a T type parameter of either Stack or Balloon – which is unlikely to be desired behaviour.)

    // So the advice here is to **use trait bounds to express requirements on the types used in generics**, but it's easy advice to follow – the compiler will force you to comply with it regardless.
}
