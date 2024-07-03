fn main() {
    fn foo(string1: String, string2: String) {}

    // Cannot consume `string2` inside `map()` as `map()` takes `f` as `FnMut`.

    // let vec = vec![String::from("hi")];
    // let string2 = String::from("world");
    // let v: Vec<_> = vec
    //     .into_iter()
    //     .map(move |string1| foo(string1, string2))
    //     .collect();
    // error[E0507]: cannot move out of `string2`, a captured variable in an `FnMut` closure
    //   --> my/experiments/src/bin/my_map.rs:49:42
    //    |
    // 46 |     let string2 = String::from("world");
    //    |         ------- captured outer variable
    // ...
    // 49 |         .map(move |string1| foo(string1, string2))
    //    |              --------------              ^^^^^^^ move occurs because `string2` has type `String`, which does not implement the `Copy` trait
    //    |              |
    //    |              captured by this `FnMut` closure

    //

    // Basically the same smaller version of `Map` with `FnMut` for `f`.

    struct MapMut<I, F> {
        iter: I,
        f: F,
    }

    impl<I, F> MapMut<I, F> {
        fn new(iter: I, f: F) -> MapMut<I, F> {
            MapMut { iter, f }
        }
    }

    impl<B, I: Iterator, F> Iterator for MapMut<I, F>
    where
        F: FnMut(I::Item) -> B,
    {
        type Item = B;

        fn next(&mut self) -> Option<B> {
            self.iter.next().map(&mut self.f)
        }
    }

    trait MapMutExt: Iterator {
        fn map_mut<B, F>(self, f: F) -> MapMut<Self, F>
        where
            Self: Sized,
            F: FnMut(Self::Item) -> B,
        {
            MapMut::new(self, f)
        }
    }

    impl<I: Iterator> MapMutExt for I {}

    // let vec = vec![String::from("hi")];
    // let string2 = String::from("world");
    // let v: Vec<_> = vec
    //     .into_iter()
    //     .map_mut(move |string1| foo(string1, string2))
    //     .collect();
    //     error[E0507]: cannot move out of `string2`, a captured variable in an `FnMut` closure
    //     --> my/experiments/src/bin/my_map.rs:65:46
    //      |
    //   62 |     let string2 = String::from("world");
    //      |         ------- captured outer variable
    //   ...
    //   65 |         .map_mut(move |string1| foo(string1, string2))
    //      |                  --------------              ^^^^^^^ move occurs because `string2` has type `String`, which does not implement the `Copy` trait
    //      |                  |
    //      |                  captured by this `FnMut` closure

    //

    // `Map` version with `FnOnce` for `f` instead of `FnMut`.

    // struct MapOnce<I, F> {
    //     iter: I,
    //     f: F,
    // }

    // impl<I, F> MapOnce<I, F> {
    //     fn new(iter: I, f: F) -> MapOnce<I, F> {
    //         MapOnce { iter, f }
    //     }
    // }

    // impl<B, I: Iterator, F> Iterator for MapOnce<I, F>
    // where
    //     F: FnOnce(I::Item) -> B
    // {
    //     type Item = B;

    //     fn next(&mut self) -> Option<B> {
    //         self.iter.next().map(&mut self.f)
    //     }
    // }

    // trait MapOnceExt: Iterator {
    //     fn map_once<B, F>(self, f: F) -> MapOnce<Self, F>
    //     where
    //         Self: Sized,
    //         F: FnOnce(Self::Item) -> B,
    //     {
    //         MapOnce::new(self, f)
    //     }
    // }

    // impl<I: Iterator> MapOnceExt for I {}

    // Yeah... This is not possible..

    //     error[E0277]: expected a `FnOnce(<I as Iterator>::Item)` closure, found `&mut F`
    //     --> my/experiments/src/bin/my_map.rs:100:39
    //      |
    // 100  |             self.iter.next().map(&mut self.f)
    //      |                              ---      ^^^^^^ expected an `FnOnce(<I as Iterator>::Item)` closure, found `&mut F`
    //      |                              |
    //      |                              required by a bound introduced by this call
    //      |
    //      = help: the trait `FnMut<(<I as Iterator>::Item,)>` is not implemented for `&mut F`, which is required by `&mut F: FnOnce(<I as Iterator>::Item)`
    //      = note: required for `&mut F` to implement `FnOnce<(<I as Iterator>::Item,)>`
    // note: required by a bound in `Option::<T>::map`
    //     --> /home/mhnap/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1070:12
    //      |
    // 1068 |     pub fn map<U, F>(self, f: F) -> Option<U>
    //      |            --- required by a bound in this associated function
    // 1069 |     where
    // 1070 |         F: FnOnce(T) -> U,
    //      |            ^^^^^^^^^^^^^^ required by this bound in `Option::<T>::map`
    // help: consider dereferencing here
    //      |
    // 100  |             self.iter.next().map(&mut *self.f)
    //      |                                       +
    // help: consider further restricting this bound
    //      |
    // 95   |         F: FnOnce(I::Item) -> B + std::ops::FnMut<(<I as std::iter::Iterator>::Item,)>,
    //      |                                 ++++++++++++++++++++++++++++++++++++++++++++++++++++++

    //

    // This can be explained by for loop..

    // let vec = vec![String::from("hi")];
    // let string2 = String::from("world");
    // for string1 in vec {
    //     foo(string1, string2);
    // }
    //     error[E0382]: use of moved value: `string2`
    //     --> my/experiments/src/bin/my_map.rs:152:22
    //      |
    //  150 |     let string2 = String::from("world");
    //      |         ------- move occurs because `string2` has type `String`, which does not implement the `Copy` trait
    //  151 |     for string1 in vec {
    //      |     ------------------ inside of this loop
    //  152 |         foo(string1, string2);
    //      |                      ^^^^^^^ value moved here, in previous iteration of loop
    //      |
}
