// https://manishearth.github.io/blog/2017/01/10/rust-tidbits-box-is-special

fn main() {
    {
        // You could move out of deref with Box<T>.
        // For a regular type, *foo will produce a temporary that must be immediately borrowed or copied.
        // You cannot do let x = *y for a non-Copy type.
        // This dereference operation will call DerefMut::deref_mut or Deref::deref based on how it gets borrowed.
        // With Box<T>, you can do this:
        let x = Box::new(vec![1, 2, 3, 4]);
        let _y = *x; // moves the vec out into `y`, then deallocates the box
        // but does not call a destructor on the vec
        // This operation is colloquially called DerefMove.
    }

    {
        let _x = &mut vec![1, 2, 3, 4];
        // let _y = *x;
        // error[E0507]: cannot move out of `*x` which is behind a mutable reference
        //   --> my/experiments/src/bin/test.rs:12:18
        //    |
        // 12 |         let _y = *x;
        //    |                  ^^ move occurs because `*x` has type `Vec<i32>`, which does not implement the `Copy` trait
        //    |
        // help: consider removing the dereference here
        //    |
        // 12 -         let _y = *x;
        // 12 +         let _y = x;
        //    |
        // help: consider cloning the value if the performance cost is acceptable
        //    |
        // 12 -         let _y = *x;
        // 12 +         let _y = x.clone();
        //    |
    }
}
