// https://www.lurklurk.org/effective-rust/references.html
// Item 8: Familiarize yourself with reference and pointer types

fn main() {
    // For programming in general, a reference is a way to indirectly access some data structure, separately from whatever variable owns that data structure.
    // In practice, this is usually implemented as a pointer: a number whose value is the address in memory of the data structure.

    // A modern CPU will typically police a few constraints on pointers—the memory address should be in a valid range of memory (whether virtual or physical)
    // and may need to be aligned (e.g., a 4-byte integer value might be accessible only if its address is a multiple of 4).

    // However, higher-level programming languages usually encode more information about pointers in their type systems.
    // In C-derived languages, including Rust, pointers have a type that indicates what kind of data structure is expected to be present at the pointed-to memory address.
    // This allows the code to interpret the contents of memory at that address and in the memory following that address.

    // This basic level of pointer information—putative memory location and expected data structure layout—is represented in Rust as a raw pointer.
    // However, safe Rust code does not use raw pointers, because Rust provides richer reference and pointer types that provide additional safety guarantees and constraints.

    //

    // # Rust References

    // The most ubiquitous pointer-like type in Rust is the reference, with a type that is written as &T for some type T.
    // Although this is a pointer value under the covers, the compiler ensures that various rules around its use are observed:
    // it must always point to a valid, correctly aligned instance of the relevant type T, whose lifetime (Item 14) extends beyond its use,
    // and it must satisfy the borrow checking rules (Item 15).
    // These additional constraints are always implied by the term reference in Rust, and so the bare term pointer is generally rare.

    // Naming pattern reflects a slightly different mindset between Rust and C++:
    // In Rust, the default variant is read-only, and writable types are marked specially (with mut).
    // In C++, the default variant is writable, and read-only types are marked specially (with const).

    pub struct Point {
        pub x: u32,
        pub y: u32,
    }

    let pt = Point { x: 1, y: 2 };
    let x = 0u64;
    let ref_x = &x;
    let ref_pt = &pt;

    // A Rust reference can refer to items that are located either on the stack or on the heap.
    // Rust allocates items on the stack by default, but the Box<T> pointer type (roughly equivalent to C++'s std::unique_ptr<T>) forces allocation to occur on the heap,
    // which in turn means that the allocated item can outlive the scope of the current block.
    // Under the covers, Box<T> is also a simple eight-byte pointer value:

    let box_pt = Box::new(Point { x: 10, y: 20 });

    //

    // # Pointer Traits

    // A method that expects a reference argument like &Point can also be fed a &Box<Point>:

    fn show(pt: &Point) {
        println!("({}, {})", pt.x, pt.y);
    }
    show(ref_pt);
    show(&box_pt);

    // This is possible because Box<T> implements the Deref trait, with Target = T.
    // An implementation of this trait for some type means that the trait's deref() method can be used to create a reference to the Target type.
    // There's also an equivalent DerefMut trait, which emits a mutable reference to the Target type.

    // The Deref/DerefMut traits are somewhat special, because the Rust compiler has specific behavior when dealing with types that implement them.
    // When the compiler encounters a dereferencing expression (e.g., *x), it looks for and uses an implementation of one of these traits, depending on whether the dereference requires mutable access or not.
    // This Deref coercion allows various smart pointer types to behave like normal references and is one of the few mechanisms that allow implicit type conversion in Rust (as described in Item 5).

    // As a technical aside, it's worth understanding why the Deref traits can't be generic (Deref<Target>) for the destination type.
    // If they were, then it would be possible for some type ConfusedPtr to implement both Deref<TypeA> and Deref<TypeB>, and that would leave the compiler unable to deduce a single unique type for an expression like *x.
    // So instead, the destination type is encoded as the associated type named Target.

    // This technical aside provides a contrast to two other standard pointer traits, the AsRef and AsMut traits.
    // These traits don't induce special behavior in the compiler but allow conversions to a reference or mutable reference via an explicit call to their trait functions (as_ref() and as_mut(), respectively).
    // The destination type for these conversions is encoded as a type parameter (e.g., AsRef<Point>), which means that a single container type can support multiple destinations.

    //

    // # Fat Pointer Types

    // Rust has two built-in fat pointer types: slices and trait objects.
    // These are types that act as pointers but hold additional information about the thing they are pointing to.

    // ## Slices

    // The first fat pointer type is the slice: a reference to a subset of some contiguous collection of values.
    // It's built from a (non-owning) simple pointer, together with a length field, making it twice the size of a simple pointer (16 bytes on a 64-bit platform).
    // The type of a slice is written as &[T]—a reference to [T], which is the notional type for a contiguous collection of values of type T.

    // The notional type [T] can't be instantiated, but there are two common containers that embody it.
    // The first is the array: a contiguous collection of values having a size that is known at compile time—an array with five values will always have five values.
    // A slice can therefore refer to a subset of an array:

    let array: [u64; 5] = [0, 1, 2, 3, 4];
    let slice = &array[1..3];

    // The other common container for contiguous values is a Vec<T>.
    // This holds a contiguous collection of values like an array, but unlike an array, the number of values in the Vec can grow (e.g., with push(value)) or shrink (e.g., with pop()).

    // The contents of the Vec are kept on the heap (which allows for this variation in size) but are always contiguous, and so a slice can refer to a subset of a vector:

    let mut vector = Vec::<u64>::with_capacity(8);
    for i in 0..5 {
        vector.push(i);
    }
    let vslice = &vector[1..3];

    // There's quite a lot going on under the covers for the expression &vector[1..3], so it's worth breaking it down into its components:
    // - The 1..3 part is a range expression; the compiler converts this into an instance of the Range<usize> type, which holds an inclusive lower bound and an exclusive upper bound.
    // - The Range type implements the SliceIndex<T> trait, which describes indexing operations on slices of an arbitrary type T (so the Output type is [T]).
    // - The vector[ ] part is an indexing expression; the compiler converts this into an invocation of the Index trait's index method on vector, together with a dereference (i.e., *vector.index( )).
    // - vector[1..3] therefore invokes Vec<T>'s implementation of Index<I>, which requires I to be an instance of SliceIndex<[u64]>. This works because Range<usize> implements SliceIndex<[T]> for any T, including u64.
    // - &vector[1..3] undoes the dereference, resulting in a final expression type of &[u64].

    // ## Trait objects

    // The second built-in fat pointer type is a trait object: a reference to some item that implements a particular trait.
    // It's built from a simple pointer to the item, together with an internal pointer to the type's vtable, giving a size of 16 bytes (on a 64-bit platform).
    // The vtable for a type's implementation of a trait holds function pointers for each of the method implementations, allowing dynamic dispatch at runtime.

    trait Calculate {
        fn add(&self, l: u64, r: u64) -> u64;
        fn mul(&self, l: u64, r: u64) -> u64;
    }

    struct Modulo(pub u64);

    impl Calculate for Modulo {
        fn add(&self, l: u64, r: u64) -> u64 {
            (l + r) % self.0
        }
        fn mul(&self, l: u64, r: u64) -> u64 {
            (l * r) % self.0
        }
    }

    let mod3 = Modulo(3);

    // Need an explicit type to force dynamic dispatch.
    let tobj: &dyn Calculate = &mod3;
    let result = tobj.add(2, 2);
    assert_eq!(result, 1);

    // Code that holds a trait object can invoke the methods of the trait via the function pointers in the vtable, passing in the item pointer as the &self parameter.

    // # More Pointer Traits

    // A previous section described two pairs of traits (Deref/DerefMut, AsRef/AsMut) that are used when dealing with types that can be easily converted into references.
    // There are a few more standard traits that can also come into play when working with pointer-like types, whether from the standard library or user defined.

    // The simplest of these is the Pointer trait, which formats a pointer value for output.
    // This can be helpful for low-level debugging, and the compiler will reach for this trait automatically when it encounters the {:p} format specifier.

    // More intriguing are the Borrow and BorrowMut traits, which each have a single method (borrow and borrow_mut, respectively).
    // This method has the same signature as the equivalent AsRef/AsMut trait methods.

    // The key difference in intents between these traits is visible via the blanket implementations that the standard library provides.
    // Given an arbitrary Rust reference &T, there is a blanket implementation of both AsRef and Borrow; likewise, for a mutable reference &mut T, there's a blanket implementation of both AsMut and BorrowMut.

    // However, Borrow also has a blanket implementation for (non-reference) types: impl<T> Borrow<T> for T.
    // This means that a method accepting the Borrow trait can cope equally with instances of T as well as references-to-T:

    struct Int(i32);

    {
        // fn add_four(v: Int) -> i32 {
        //     v.0 + 4
        // }
        // assert_eq!(add_four(&Int(2)), 6);
        // assert_eq!(add_four(Int(2)), 6);
        //     error[E0308]: mismatched types
        //     --> others/books/effective_rust/src/bin/item_8.rs:169:29
        //      |
        //  169 |         assert_eq!(add_four(&Int(2)), 6);
        //      |                    -------- ^^^^^^^ expected `Int`, found `&Int`
        //      |                    |
        //      |                    arguments to this function are incorrect
        //      |
        //  note: function defined here
        //     --> others/books/effective_rust/src/bin/item_8.rs:166:12
        //      |
        //  166 |         fn add_four(v: Int) -> i32 {
        //      |            ^^^^^^^^ ------
        //  help: consider removing the borrow
        //      |
        //  169 -         assert_eq!(add_four(&Int(2)), 6);
        //  169 +         assert_eq!(add_four(Int(2)), 6);
        //      |
    }

    {
        // fn add_four(v: &Int) -> i32 {
        //     v.0 + 4
        // }
        // assert_eq!(add_four(&Int(2)), 6);
        // assert_eq!(add_four(Int(2)), 6);
        //     error[E0308]: mismatched types
        //     --> others/books/effective_rust/src/bin/item_8.rs:196:29
        //      |
        //  196 |         assert_eq!(add_four(Int(2)), 6);
        //      |                    -------- ^^^^^^ expected `&Int`, found `Int`
        //      |                    |
        //      |                    arguments to this function are incorrect
        //      |
        //  note: function defined here
        //     --> others/books/effective_rust/src/bin/item_8.rs:192:12
        //      |
        //  192 |         fn add_four(v: &Int) -> i32 {
        //      |            ^^^^^^^^ -------
        //  help: consider borrowing here
        //      |
        //  196 |         assert_eq!(add_four(&Int(2)), 6);
        //      |                             +
    }

    {
        fn add_four<T: std::borrow::Borrow<Int>>(v: T) -> i32 {
            v.borrow().0 + 4
        }
        assert_eq!(add_four(&Int(2)), 6);
        assert_eq!(add_four(Int(2)), 6);

        // let int = Int(3);
        // add_four(int);
        // dbg!(int.0);
        //     error[E0382]: use of moved value: `int`
        //     --> others/books/effective_rust/src/bin/item_8.rs:225:9
        //      |
        //  223 |         let int = Int(3);
        //      |             --- move occurs because `int` has type `Int`, which does not implement the `Copy` trait
        //  224 |         add_four(int);
        //      |                  --- value moved here
        //  225 |         dbg!(int.0);
        //      |         ^^^^^^^^^^^ value used here after move
        //      |
        //  note: consider changing this parameter type in function `add_four` to borrow instead if owning the value isn't necessary
        //     --> others/books/effective_rust/src/bin/item_8.rs:217:53
        //      |
        //  217 |         fn add_four<T: std::borrow::Borrow<Int>>(v: T) -> i32 {
        //      |            -------- in this function                ^ this parameter takes ownership of the value
        //      = note: this error originates in the macro `dbg` (in Nightly builds, run with -Z macro-backtrace for more info)
    }

    {
        // fn add_four<T: std::convert::AsRef<Int>>(v: T) -> i32 {
        //     v.as_ref().0 + 4
        // }
        // assert_eq!(add_four(&2), 6);
        // assert_eq!(add_four(2), 6);
        //     error[E0277]: the trait bound `{integer}: AsRef<Int>` is not satisfied
        //     --> others/books/effective_rust/src/bin/item_8.rs:248:30
        //      |
        //  248 |         assert_eq!(add_four(&2), 6);
        //      |                    --------  ^ the trait `AsRef<Int>` is not implemented for `{integer}`, which is required by `&{integer}: AsRef<Int>`
        //      |                    |
        //      |                    required by a bound introduced by this call
        //      |
        //      = note: required for `&{integer}` to implement `AsRef<Int>`
        //  note: required by a bound in `main::add_four`
        //     --> others/books/effective_rust/src/bin/item_8.rs:245:24
        //      |
        //  245 |         fn add_four<T: std::convert::AsRef<Int>>(v: T) -> i32 {
        //      |                        ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `add_four`

        //  error[E0277]: the trait bound `{integer}: AsRef<Int>` is not satisfied
        //     --> others/books/effective_rust/src/bin/item_8.rs:249:29
        //      |
        //  249 |         assert_eq!(add_four(2), 6);
        //      |                    -------- ^ the trait `AsRef<Int>` is not implemented for `{integer}`
        //      |                    |
        //      |                    required by a bound introduced by this call
        //      |
        //  note: required by a bound in `main::add_four`
        //     --> others/books/effective_rust/src/bin/item_8.rs:245:24
        //      |
        //  245 |         fn add_four<T: std::convert::AsRef<Int>>(v: T) -> i32 {
        //      |                        ^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `add_four`
    }

    // The standard library's container types have more realistic uses of Borrow.
    // For example, HashMap::get uses Borrow to allow convenient retrieval of entries whether keyed by value or by reference.

    // The ToOwned trait builds on the Borrow trait, adding a to_owned() method that produces a new owned item of the underlying type.
    // This is a generalization of the Clone trait: where Clone specifically requires a Rust reference &T, ToOwned instead copes with things that implement Borrow.

    // This gives a couple of possibilities for handling both references and moved items in a unified way:
    // - A function that operates on references to some type can accept Borrow so that it can also be called with moved items as well as references.
    // - A function that operates on owned items of some type can accept ToOwned so that it can also be called with references to items as well as moved items; any references passed to it will be replicated into a locally owned item.

    // Although it's not a pointer type, the Cow type is worth mentioning at this point, because it provides an alternative way of dealing with the same kind of situation.
    // Cow is an enum that can hold either owned data or a reference to borrowed data.
    // The peculiar name stands for "clone-on-write": a Cow input can remain as borrowed data right up to the point where it needs to be modified, but it becomes an owned copy at the point where the data needs to be altered.

    // # Smart Pointer Types

    // The Rust standard library includes a variety of types that act like pointers to some degree or another, mediated by the standard library traits previously described.
    // These smart pointer types each come with some particular semantics and guarantees, which has the advantage that the right combination of them can give fine-grained control over the pointer's behavior, but has the disadvantage that the resulting types can seem overwhelming at first (Rc<RefCell<Vec<T>>>, anyone?).

    // The first smart pointer type is Rc<T>, which is a reference-counted pointer to an item (roughly analogous to C++'s std::shared_ptr<T>).
    // It implements all of the pointer-related traits and so acts like a Box<T> in many ways.

    // This is useful for data structures where the same item can be reached in different ways, but it removes one of Rust's core rules around ownership—that each item has only one owner.
    // Relaxing this rule means that it is now possible to leak data: if item A has an Rc pointer to item B, and item B has an Rc pointer to A, then the pair will never be dropped.
    // To put it another way: you need Rc to support cyclical data structures, but the downside is that there are now cycles in your data structures.

    // The risk of leaks can be ameliorated in some cases by the related Weak<T> type, which holds a non-owning reference to the underlying item (roughly analogous to C++'s std::weak_ptr<T>).
    // Holding a weak reference doesn't prevent the underlying item from being dropped (when all strong references are removed), so making use of the Weak<T> involves an upgrade to an Rc<T>—which can fail.
    // Under the hood, Rc is (currently) implemented as a pair of reference counts together with the referenced item, all stored on the heap:

    use std::rc::Rc;
    let rc1: Rc<u64> = Rc::new(42);
    let rc2 = rc1.clone();
    let wk = Rc::downgrade(&rc1);
    assert_eq!(*rc1, 42);
    assert_eq!(*rc2, 42);
    assert_eq!(*wk.upgrade().unwrap(), 42);

    // The underlying item is dropped when the strong reference count drops to zero, but the bookkeeping structure is dropped only when the weak reference count also drops to zero.

    // An Rc on its own gives you the ability to reach an item in different ways, but when you reach that item, you can modify it (via get_mut) only if there are no other ways to reach the item—i.e., there are no other extant Rc or Weak references to the same item.
    // That's hard to arrange, so Rc is often combined with RefCell.

    // The next smart pointer type, RefCell<T>, relaxes the rule (Item 15) that an item can be mutated only by its owner or by code that holds the (only) mutable reference to the item.
    // This interior mutability allows for greater flexibility—for example, allowing trait implementations that mutate internals even when the method signature allows only &self.
    // However, it also incurs costs: as well as the extra storage overhead (an extra isize to track current borrows, as shown in Figure 1-8), the normal borrow checks are moved from compile time to runtime:

    use std::cell::RefCell;
    let rc: RefCell<u64> = RefCell::new(42);
    let b1 = rc.borrow();
    let b2 = rc.borrow();
    assert_eq!(*b1, 42);
    assert_eq!(*b2, 42);

    // The runtime nature of these checks means that the RefCell user has to choose between two options, neither pleasant:
    // - Accept that borrowing is an operation that might fail, and cope with Result values from try_borrow[_mut];
    // - Use the allegedly infallible borrowing methods borrow[_mut], and accept the risk of a panic! at runtime (Item 18) if the borrow rules have not been complied with;

    // In either case, this runtime checking means that RefCell itself implements none of the standard pointer traits; instead, its access operations return a Ref<T> or RefMut<T> smart pointer type that does implement those traits.

    // If the underlying type T implements the Copy trait (indicating that a fast bit-for-bit copy produces a valid item; see Item 10), then the Cell<T> type allows interior mutation with less overhead—the get(&self) method copies out the current value, and the set(&self, val) method copies in a new value.
    // The Cell type is used internally by both the Rc and RefCell implementations, for shared tracking of counters that can be mutated without a &mut self.

    // The smart pointer types described so far are suitable only for single-threaded use; their implementations assume that there is no concurrent access to their internals.
    // If this is not the case, then smart pointers that include additional synchronization overhead are needed.

    // The thread-safe equivalent of Rc<T> is Arc<T>, which uses atomic counters to ensure that the reference counts remain accurate.
    // Like Rc, Arc implements all of the various pointer-related traits.

    // However, Arc on its own does not allow any kind of mutable access to the underlying item.
    // This is covered by the Mutex type, which ensures that only one thread has access—whether mutably or immutably—to the underlying item.
    // As with RefCell, Mutex itself does not implement any pointer traits, but its lock() operation returns a value of a type that does: MutexGuard, which implements Deref[Mut].

    // If there are likely to be more readers than writers, the RwLock type is preferable, as it allows multiple readers access to the underlying item in parallel, provided that there isn't currently a (single) writer.

    // In either case, Rust's borrowing and threading rules force the use of one of these synchronization containers in multithreaded code (but this guards against only some of the problems of shared-state concurrency; see Item 17).

    // The same strategy—see what the compiler rejects and what it suggests instead—can sometimes be applied with the other smart pointer types.
    // However, it's faster and less frustrating to understand what the behavior of the different smart pointers implies.
    // To borrow (pun intended) an example from the first edition of the Rust book:
    // - Rc<RefCell<Vec<T>>> holds a vector (Vec) with shared ownership (Rc), where the vector can be mutated—but only as a whole vector.
    // - Rc<Vec<RefCell<T>>> also holds a vector with shared ownership, but here each individual entry in the vector can be mutated independently of the others.
    // The types involved precisely describe these behaviors.
}
