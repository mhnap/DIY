// From <https://www.youtube.com/watch?v=dHkzSZnYXmk&list=PLqbS7AVVErFiWDOAVrPt7aYmnuuOLYvOa&index=12&ab_channel=JonGjengset>
// More:
/*
https://doc.rust-lang.org/reference/types/function-item.html
https://doc.rust-lang.org/reference/types/function-pointer.html
https://doc.rust-lang.org/reference/type-coercions.html#r-coerce.types.fn
https://doc.rust-lang.org/std/primitive.fn.html
https://doc.rust-lang.org/book/ch13-01-closures.html
https://doc.rust-lang.org/reference/types/closure.html
https://doc.rust-lang.org/std/ops/trait.Fn.html
https://doc.rust-lang.org/std/ops/trait.FnMut.html
https://doc.rust-lang.org/std/ops/trait.FnOnce.html
*/

#[allow(unused)]
fn main() {
    {
        {
            fn bar() {}
            let x = bar;
        }

        {
            fn bar<T>() {}
            // let x = bar;
            let mut x = bar::<i32>;
            // x = bar::<u32>;

            {
                assert_eq!(size_of_val(&x), 0);
                fn take_fn(f: fn()) {
                    assert_eq!(size_of_val(&f), 8);
                }
                take_fn(x);
            }
            {
                let mut x = x as fn();
                assert_eq!(size_of_val(&x), 8);
                x = bar::<u32>;
            }
            {
                dbg!(bar::<i32> as fn());
                dbg!(bar::<u32> as fn());
                // assert_ne!(bar::<i32>, bar::<u32>);
                assert_ne!(bar::<i32> as fn(), bar::<u32> as fn());
            }
            {
                fn take_fn(f: fn(i32)) {}
                // take_fn(bar::<i32>);
            }
            {
                fn add_one(i: i32) -> i32 {
                    i + 1
                }
                fn add_two(i: i32) -> i32 {
                    i + 2
                }
                let fns = vec![add_one, add_two];
                for f in fns {
                    f(1);
                }
            }
        }
    }

    //

    {
        let mut i = 1;

        {
            let mut c = || i += 1;
            c();
            assert_eq!(i, 2);
        }
        {
            struct C<'a> {
                i_ref: &'a mut i32,
            }

            impl<'a> C<'a> {
                fn call(&mut self) {
                    *self.i_ref += 1;
                }
            }

            let mut c = C { i_ref: &mut i };
            c.call();
            assert_eq!(i, 3);
        }
        {
            let mut c = || i += 1;
            // i;
            c();
        }
        {
            let c = |i: i32| i + 1;
            let f = c as fn(i32) -> i32;
        }
        {
            let c = || i + 1;
            // let f = c as fn() -> i32;
        }
    }

    //

    {
        fn check_fn<F: Fn()>(f: F) {}
        fn check_fnmut<F: FnMut()>(f: F) {}
        fn check_fnonce<F: FnOnce()>(f: F) {}

        fn func() {}
        check_fn(func);
        check_fnmut(func);
        check_fnonce(func);

        {
            struct A(i32);
            let mut a = A(1);

            check_fn(|| {
                &a;
                // &mut a; // error[E0596]: cannot borrow `a` as mutable, as it is a captured variable in a `Fn` closure
                // a; // error[E0507]: cannot move out of `a`, a captured variable in an `Fn` closure
            });
            check_fnmut(|| {
                &a;
                &mut a;
                // a; // error[E0507]: cannot move out of `a`, a captured variable in an `FnMut` closure
            });
            check_fnonce(|| {
                &a;
                &mut a;
                a;
            });
        }
        {
            let s = String::new();
            let c = move || {
                s.len();
            };
            // s;
            c();
            c();
            check_fn(c);
        }
        {
            fn make_fn() -> impl Fn() {
                let s = String::new();
                // || {
                //     dbg!(&s);
                // }
                move || {
                    dbg!(&s);
                }
            }
        }
    }
}
