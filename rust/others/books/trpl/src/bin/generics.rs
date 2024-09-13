// https://doc.rust-lang.org/book/ch10-01-syntax.html

fn main() {
    {
        // Use of generics in function definitions.

        // Find largest for i32 type.
        fn largest_i32(list: &[i32]) -> &i32 {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        // Find largest for char type.
        fn largest_char(list: &[char]) -> &char {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_char(&char_list);
        println!("The largest char is {}", result);

        // Remove code duplication by using a generic function.

        // fn largest<T>(list: &[T]) -> &T {
        //     let mut largest = &list[0];
        //     for item in list {
        //         if item > largest {
        //             largest = item;
        //         }
        //     }
        //     largest
        // }
        // error[E0369]: binary operation `>` cannot be applied to type `&T`
        //   --> src/lessons/generics.rs:41:25
        //    |
        // 41 |                 if item > largest {
        //    |                    ---- ^ ------- &T
        //    |                    |
        //    |                    &T
        //    |
        // help: consider restricting type parameter `T`
        //    |
        // 38 |         fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
        //    |                     ++++++++++++++++++++++

        // This error states that the body of `largest` won’t work for all possible types that T could be.
        // Because we want to compare values of type T in the body, we can only use types whose values can be ordered.
        // Need to restrict on types that implement the `std::cmp::PartialOrd` trait.

        fn largest<T: PartialOrd>(list: &[T]) -> &T {
            let mut largest = &list[0];
            for item in list {
                if item > largest {
                    largest = item;
                }
            }
            largest
        }

        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {}", result);

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {}", result);
    }

    {
        // We can also define structs to use a generic type parameter in one or more fields using the <> syntax.
        #[derive(Debug)]
        struct Point<T> {
            x: T,
            y: T,
        }

        let integer = Point { x: 5, y: 10 };
        dbg!(integer);
        let float = Point { x: 1.0, y: 4.0 };
        dbg!(float);

        // The fields `x` and `y` are both must have the same type.
        // let wont_work = Point { x: 5, y: 4.0 };
        // error[E0308]: mismatched types
        //   --> src/lessons/generics.rs:98:42
        //    |
        // 98 |         let wont_work = Point { x: 5, y: 4.0 };
        //    |                                          ^^^ expected integer, found floating-point number

        // Need to use two generic types to support different field types.
        #[derive(Debug)]
        struct PointV2<T, U> {
            x: T,
            y: U,
        }

        let integer_and_float = PointV2 { x: 5, y: 4.0 };
        dbg!(integer_and_float);
    }

    {
        // The same way generics can be used in enums.
        // Classic examples are Option<T> and Result<T, E>.
        enum Option<T> {
            Some(T),
            None,
        }

        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    }

    {
        // Use of generics in method definitions.
        struct Point<T> {
            x: T,
            y: T,
        }

        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }

        // Note that we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>.
        // By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.

        let p = Point { x: 5, y: 10 };
        println!("p.x = {}", p.x());

        // We can also specify constraints on generic types when defining methods on the type.
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }

        let pf = Point::<f32> { x: 5.0, y: 10.0 };

        // f32 have it's method specialization.
        pf.distance_from_origin();

        // Other types don't have such method.
        // p.distance_from_origin();
        // error[E0599]: no method named `distance_from_origin` found for struct `main::Point<{integer}>` in the current scope
        //    --> src/lessons/generics.rs:157:11
        //     |
        // 130 |         struct Point<T> {
        //     |         --------------- method `distance_from_origin` not found for this struct
        // ...
        // 157 |         p.distance_from_origin();
        //     |           ^^^^^^^^^^^^^^^^^^^^ method not found in `Point<{integer}>`
        //     |
        //     = note: the method was found for
        //             - `main::Point<f32>`

        // Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
        {
            struct Point<X1, Y1> {
                x: X1,
                y: Y1,
            }

            impl<X1, Y1> Point<X1, Y1> {
                fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> {
                    Point { x: self.x, y: other.y }
                }
            }

            let p1 = Point { x: 5, y: 10.4 };
            let p2 = Point { x: "Hello", y: 'c' };
            let p3 = p1.mixup(p2);
            println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
        }
    }

    {
        // You might be wondering whether there is a runtime cost when using generic type parameters.
        // The good news is that using generic types won't make your program run any slower than it would with concrete types.
        // Rust accomplishes this by performing monomorphization of the code using generics at compile time.
        // Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.
        // The generic Option<T> is replaced with the specific definitions created by the compiler.
        // Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics.
        // When the code runs, it performs just as it would if we had duplicated each definition by hand.
        // The process of monomorphization makes Rust’s generics extremely efficient at runtime.
    }

    {
        // https://www.reddit.com/r/rust/comments/wvhu6z/generic_param_accepts_both_referencetype_and/
        let mut a = 1;

        // Generics match concrete type.
        fn foo<T>(t: T) {
            println!("T is {}", std::any::type_name::<T>());
        }
        foo(a); // T is i32
        foo(&a); // T is &i32
        foo(&mut a); // T is &mut i32
        foo::<&i32>(&mut a); // Can make T as &i32.

        // When there is ref or mut ref with a generic type, it adds just constraints.
        fn bar<T>(t: &T) {
            println!("T is {}", std::any::type_name::<T>());
        }
        // bar(a); // error[E0308]: mismatched types
        bar::<i32>(&a); // T is i32
        bar::<&i32>(&&a); // Need to add one more reference to make T as &i32.
        bar(&mut a); // T is i32

        fn baz<T>(t: &mut T) {
            println!("T is {}", std::any::type_name::<T>());
        }
        // baz(a); // error[E0308]: mismatched types
        // baz(&a); // error[E0308]: mismatched types
        baz(&mut a); // T is i32
    }
}
