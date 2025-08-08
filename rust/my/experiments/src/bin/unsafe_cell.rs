use std::cell::UnsafeCell;

fn main() {
    // No UB.
    {
        let s = String::from("hi");
        let cell = UnsafeCell::new(s);
        let s1: *mut String = cell.get();
        let s2: *mut String = cell.get();
        unsafe {
            dbg!(&*s1, &*s2);
            (*s1).push_str("ho");
            (*s2).push_str("ha");
            dbg!(&*s1, &*s2);
        }
    }
    // UB.
    {
        let mut s = String::from("hi");
        let s1: *mut String = &mut s as *mut String;
        let s2: *mut String = &mut s as *mut String;
        unsafe {
            dbg!(&*s1, &*s2);
            (*s1).push_str("ho");
            (*s2).push_str("ha");
            dbg!(&*s1, &*s2);
        }
        // cargo +nightly miri run --bin test
        // error: Undefined Behavior: trying to retag from <10064> for SharedReadOnly permission at alloc3528[0x0], but that tag does not exist in the borrow stack for this location
        //   --> my/experiments/src/bin/test.rs:23:18
        //    |
        // 23 |             dbg!(&*s1, &*s2);
        //    |                  ^^^^
        //    |                  |
        //    |                  trying to retag from <10064> for SharedReadOnly permission at alloc3528[0x0], but that tag does not exist in the borrow stack for this location
        //    |                  this error occurs as part of retag at alloc3528[0x0..0x18]
        //    |
        //    = help: this indicates a potential bug in the program: it performed an invalid operation, but the Stacked Borrows rules it violated are still experimental
        //    = help: see https://github.com/rust-lang/unsafe-code-guidelines/blob/master/wip/stacked-borrows.md for further information
        // help: <10064> was created by a SharedReadWrite retag at offsets [0x0..0x18]
        //   --> my/experiments/src/bin/test.rs:20:31
        //    |
        // 20 |         let s1: *mut String = &mut s as *mut String;
        //    |                               ^^^^^^
        // help: <10064> was later invalidated at offsets [0x0..0x18] by a Unique retag
        //   --> my/experiments/src/bin/test.rs:21:31
        //    |
        // 21 |         let s2: *mut String = &mut s as *mut String;
        //    |                               ^^^^^^
        //    = note: BACKTRACE (of the first span):
        //    = note: inside `main` at my/experiments/src/bin/test.rs:23:18: 23:22
    }
}
