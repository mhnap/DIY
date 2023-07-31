// https://predr.ag/blog/definitive-guide-to-sealed-traits-in-rust

mod non_sealed {
    pub trait MyTrait {
        fn hi_from(&self) {
            println!("Hi from MyTrait!");
        }
    }

    pub struct MyStruct;

    impl MyTrait for MyStruct {}
}

mod sealed {
    mod private {
        pub trait Sealed {}
    }

    pub trait MyTrait: private::Sealed {
        fn hi_from(&self) {
            println!("Hi from MyTrait!");
        }
    }

    pub struct MyStruct;

    impl private::Sealed for MyStruct {}

    impl MyTrait for MyStruct {}
}

mod sealed_method {
    mod private {
        pub struct Sealed {}
    }

    pub trait MyTrait {
        fn hi_from(&self, _: private::Sealed);
        fn hi(&self);
    }

    pub struct MyStruct;

    impl MyTrait for MyStruct {
        fn hi_from(&self, _: private::Sealed) {
            println!("Hi from MyStruct!");
        }

        fn hi(&self) {
            self.hi_from(private::Sealed {});
        }
    }
}

mod partially_sealed {
    mod private {
        pub struct Sealed;
    }

    pub trait MyTrait {
        fn hi_from(&self, _: private::Sealed) {
            println!("Hi from MyTrait!");
        }
        fn hi(&self);
    }

    pub struct MyStruct;

    impl MyTrait for MyStruct {
        fn hi(&self) {
            self.hi_from(private::Sealed {});
        }
    }
}

mod partially_sealed_with_bound {
    mod private {
        pub struct Sealed;
        pub trait IsSealed {}
        impl IsSealed for Sealed {}
    }

    pub trait MyTrait {
        fn hi_from<L: private::IsSealed>(&self) {
            println!("Hi from MyStruct!");
        }
        fn hi(&self);
    }

    pub struct MyStruct;

    impl MyTrait for MyStruct {
        fn hi(&self) {
            self.hi_from::<private::Sealed>();
        }
    }
}

fn main() {
    {
        use non_sealed::*;

        fn foo(bar: impl MyTrait) {
            bar.hi_from();
        }

        let my_struct = MyStruct;
        foo(my_struct);

        struct A;
        impl MyTrait for A {}
        let a = A;
        foo(a);
    }

    {
        use sealed::*;

        fn foo(bar: impl MyTrait) {
            bar.hi_from();
        }

        let my_struct = MyStruct;
        foo(my_struct);

        struct A;
        // impl MyTrait for A {}
        // error[E0277]: the trait bound `main::A: sealed::private::Sealed` is not satisfied
        //    --> src/experiments/sealed_trait.rs:105:26
        //     |
        // 105 |         impl MyTrait for A {}
        //     |                          ^ the trait `sealed::private::Sealed` is not implemented for `main::A`
        //     |
        //     = help: the trait `sealed::private::Sealed` is implemented for `sealed::MyStruct`
        // note: required by a bound in `sealed::MyTrait`
        //    --> src/experiments/sealed_trait.rs:20:24
        //     |
        // 20  |     pub trait MyTrait: private::Sealed {
        //     |                        ^^^^^^^^^^^^^^^ required by this bound in `MyTrait`

        // impl sealed::private::Sealed for A {}
        // error[E0603]: module `private` is private
        //    --> src/experiments/sealed_trait.rs:119:22
        //     |
        // 119 |         impl sealed::private::Sealed for A {}
        //     |                      ^^^^^^^ private module
        //     |
        // note: the module `private` is defined here
        //    --> src/experiments/sealed_trait.rs:16:5
        //     |
        // 16  |     mod private {
        //     |     ^^^^^^^^^^^
    }

    {
        use sealed_method::*;

        fn foo(bar: impl MyTrait) {
            // bar.hi_from(sealed_method::private::Sealed {});
            // error[E0603]: module `private` is private
            //    --> src/experiments/sealed_trait.rs:137:40
            //     |
            // 137 |             bar.hi_from(sealed_method::private::Sealed {});
            //     |                                        ^^^^^^^ private module
            //     |
            // note: the module `private` is defined here
            //    --> src/experiments/sealed_trait.rs:34:5
            //     |
            // 34  |     mod private {
            //     |     ^^^^^^^^^^^

            bar.hi();
        }

        let my_struct = MyStruct;
        foo(my_struct);

        struct A;
        // impl MyTrait for A {
        //     fn hi_from(&self, _: sealed_method::private::Sealed) {
        //         todo!()
        //     }
        // }
        // error[E0603]: module `private` is private
        //    --> src/experiments/sealed_trait.rs:158:49
        //     |
        // 158 |             fn hi_from(&self, _: sealed_method::private::Sealed) {
        //     |                                                 ^^^^^^^ private module
        //     |
        // note: the module `private` is defined here
        //    --> src/experiments/sealed_trait.rs:34:5
        //     |
        // 34  |     mod private {
        //     |     ^^^^^^^^^^^
    }

    {
        use partially_sealed::*;

        fn foo(bar: impl MyTrait) {
            // bar.hi_from(partially_sealed::private::Sealed {});
            // error[E0603]: module `private` is private
            //    --> src/experiments/sealed_trait.rs:179:43
            //     |
            // 179 |             bar.hi_from(partially_sealed::private::Sealed {});
            //     |                                           ^^^^^^^ private module
            //     |
            // note: the module `private` is defined here
            //    --> src/experiments/sealed_trait.rs:57:5
            //     |
            // 57  |     mod private {
            //     |     ^^^^^^^^^^^

            bar.hi();
        }

        let my_struct = MyStruct;
        foo(my_struct);

        struct A;
        impl MyTrait for A {
            fn hi(&self) {
                println!("Hi from A!");
            }
        }
        let a = A;
        foo(a);
    }

    {
        use partially_sealed_with_bound::*;

        fn foo(bar: impl MyTrait) {
            // bar.hi_from::<partially_sealed_with_bound::private::Sealed>();
            // error[E0603]: module `private` is private
            //    --> src/experiments/sealed_trait.rs:258:56
            //     |
            // 258 |             bar.hi_from::<partially_sealed_with_bound::private::Sealed>();
            //     |                                                        ^^^^^^^ private module
            //     |
            // note: the module `private` is defined here
            //    --> src/experiments/sealed_trait.rs:78:5
            //     |
            // 78  |     mod private {
            //     |     ^^^^^^^^^^^

            bar.hi();
        }

        let my_struct = MyStruct;
        foo(my_struct);

        struct A;
        impl MyTrait for A {
            fn hi(&self) {
                println!("Hi from A!");
            }
        }
        let a = A;
        foo(a);
    }
}
