fn main() {
    // Need to manually specify all fields without `Default` trait.
    #[derive(Debug)]
    struct WithoutDefault {
        first: i32,
        second: i32,
    }

    let a1 = WithoutDefault { first: 0, second: 0 };
    dbg!(&a1);

    //

    // Or we can derive `Default` trait and use the `default` method.
    #[derive(Debug, Default)]
    struct WithDerivedDefault {
        first: i32,
        second: i32,
    }

    let a2 = WithDerivedDefault::default();
    dbg!(&a2);

    //

    // Also we can override a particular field, but still retain the other defaults.
    let a3: WithDerivedDefault = WithDerivedDefault {
        second: 42,
        ..WithDerivedDefault::default() // or ..Default::default()
                                        // or ..<_>::default()
    };
    dbg!(&a3);

    //

    // There is a special syntax to choose the enum default value.
    #[derive(Debug, Default)]
    enum Kind {
        A,
        #[default]
        B,
        C,
    }

    let a4 = Kind::default();
    dbg!(&a4);

    //

    // But it works only for unit enum variants.
    // #[derive(Debug, Default)]
    // enum Kind2 {
    //     A,
    //     #[default]
    //     B {
    //         i: i32,
    //     },
    //     C,
    // }
    //
    // let a4 = Kind2::default();
    // dbg!(&a4);
    //
    // error: the `#[default]` attribute may only be used on unit enum variants
    //   --> others/books/trpl/src/bin/default_trait.rs:46:9
    //    |
    // 46 |         B {
    //    |         ^
    //    |
    //    = help: consider a manual implementation of `Default`

    //

    // Need to implement `Default` trait for type to add custom default values.
    #[derive(Debug)]
    struct WithImplDefault {
        first: i32,
        second: i32,
    }

    impl Default for WithImplDefault {
        fn default() -> Self {
            Self { first: 42, second: 43 }
        }
    }

    // Note, we can use such syntax with type annotation and `Default` trait.
    let a5: WithImplDefault = Default::default();
    dbg!(&a5);

    //

    // One often usage of `Default` is in Option::unwrap_or_default method.
    let op = Option::<Kind>::None;
    dbg!(&op);
    dbg!(&op.unwrap_or_default());
}
