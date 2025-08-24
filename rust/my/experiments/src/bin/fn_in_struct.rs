// https://stackoverflow.com/questions/27831944/how-do-i-store-a-closure-in-a-struct-in-rust

fn main() {
    // Using function pointer.
    {
        struct Holder {
            func: fn(usize) -> usize,
        }

        impl Holder {
            fn new(func: fn(usize) -> usize) -> Self {
                Self { func }
            }
        }

        let mut holder = Holder::new(|a| a + 1);
        assert_eq!((holder.func)(42), 43);

        // Can reassign to another function pointer.
        holder.func = |a| a + 2;
        assert_eq!((holder.func)(42), 44);

        // But cannot capture any variables.
        //
        // error[E0308]: mismatched types
        //   --> my/experiments/src/bin/fn_in_struct.rs:48:33
        //    |
        // 48 |         assert_eq!((Holder::new(|a| a + local_a).func)(42), 44);
        //    |                     ----------- ^^^^^^^^^^^^^^^ expected fn pointer, found closure
        //    |                     |
        //    |                     arguments to this function are incorrect
        //    |
        //    = note: expected fn pointer `fn(usize) -> usize`
        //                  found closure `{closure@my/experiments/src/bin/fn_in_struct.rs:48:33: 48:36}`
        // note: closures can only be coerced to `fn` types if they do not capture any variables
        //   --> my/experiments/src/bin/fn_in_struct.rs:48:41
        //    |
        // 48 |         assert_eq!((Holder::new(|a| a + local_a).func)(42), 44);
        //    |                                         ^^^^^^^ `local_a` captured here
        // note: associated function defined here
        //   --> my/experiments/src/bin/fn_in_struct.rs:39:16
        //    |
        // 39 |             fn new(func: fn(usize) -> usize) -> Self {
        //    |                ^^^ ------------------------
        //
        // let local_a = 2;
        // assert_eq!((Holder::new(|a| a + local_a).func)(42), 44);
    }

    // Using a generic type parameter.
    // This is the most efficient, but it does mean that a specific `Holder` instance
    // can only ever store one closure, since every closure has a different concrete type.
    {
        // It's not necessary to specify type bound during struct definition.
        struct Holder<F /* : Fn(usize) -> usize */> {
            func: F,
        }

        // But if struct definition has type bound, it's necessary to specify type bound in impl.
        //
        // error[E0277]: expected a `Fn(usize)` closure, found `F`
        //   --> my/experiments/src/bin/fn_in_struct.rs:12:44
        //    |
        // 12 |         impl<F /*  : Fn(usize) -> usize*/> Holder<F> {
        //    |                                            ^^^^^^^^^ expected an `Fn(usize)` closure, found `F`
        //    |
        impl<F /* : Fn(usize) -> usize */> Holder<F> {
            fn new(func: F) -> Self {
                Self { func }
            }
        }

        let holder = Holder::new(|a| a + 1);
        assert_eq!((holder.func)(42), 43);

        // Cannot reassign to another closure.
        // holder.func = |a| a + 2;
        // assert_eq!((holder.func)(42), 44);
        //
        // error[E0308]: mismatched types
        //   --> my/experiments/src/bin/fn_in_struct.rs:77:23
        //    |
        // 73 |         let mut holder = Holder::new(|a| a + 1);
        //    |                                      --- the expected closure
        // ...
        // 77 |         holder.func = |a| a + 2;
        //    |                       ^^^^^^^^^ expected closure, found a different closure
        //    |
        //    = note: expected closure `{closure@my/experiments/src/bin/fn_in_struct.rs:73:38: 73:41}`
        //               found closure `{closure@my/experiments/src/bin/fn_in_struct.rs:77:23: 77:26}`
        //    = note: no two closures, even if identical, have the same type
        //    = help: consider boxing your closure and/or using it as a trait object

        let local_a = 2;
        assert_eq!((Holder::new(|a| a + local_a).func)(42), 44);
    }

    // Using trait object reference.
    // There's a pointer indirection, but now it can store a reference to any
    // closure that has a compatible call signature.
    {
        struct Holder<'a> {
            func: &'a mut dyn FnMut(usize) -> usize,
        }

        impl<'a> Holder<'a> {
            fn new(func: &'a mut dyn FnMut(usize) -> usize) -> Self {
                Self { func }
            }
        }

        let mut c1 = |a| a + 1;
        let mut holder = Holder { func: &mut c1 };
        assert_eq!((holder.func)(42), 43);

        // Can reassign to another closure.
        let mut c2 = |a| a + 2;
        holder.func = &mut c2;
        assert_eq!((holder.func)(42), 44);

        let mut local_a = 2;
        assert_eq!(
            (Holder::new(&mut |a| {
                local_a += a;
                local_a
            })
            .func)(42),
            44
        );
        assert_eq!(local_a, 44);
    }

    // Using boxed trait object.
    // This involves allocating the closure on the heap, but we don't have to worry about lifetimes.
    // As with a reference, we can store any closure with a compatible signature.
    {
        struct Holder {
            func: Box<dyn Fn(usize) -> usize>,
        }

        impl Holder {
            fn new(func: Box<dyn Fn(usize) -> usize>) -> Self {
                Self { func }
            }
        }

        let mut holder = Holder { func: Box::new(|a| a + 1) };
        assert_eq!((holder.func)(42), 43);

        // Can reassign to another closure.
        holder.func = Box::new(|a| a + 2);
        assert_eq!((holder.func)(42), 44);

        let local_a = 2;
        assert_eq!((Holder::new(Box::new(move |a| a + local_a)).func)(42), 44);
    }
}
