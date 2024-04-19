// https://users.rust-lang.org/t/how-to-create-large-objects-directly-in-heap/26405
// https://github.com/rust-lang/rust/issues/53827
// https://www.reddit.com/r/rust/comments/1347g60/anyway_to_initialize_objects_on_heap/

const SIZE: usize = 1 << 32;

struct BigStruct {
    data: [u8; SIZE],
}

impl Default for BigStruct {
    fn default() -> Self {
        Self { data: [0; SIZE] }
    }
}

impl Drop for BigStruct {
    fn drop(&mut self) {
        println!("drop");
    }
}

fn main() {
    // This code _can_ overflow the stack cause the value _can_ be created first on the stack,
    // and only after _can_ be moved to the heap inside `Box`.
    let boxed = Box::new(BigStruct::default());
    // There is `_can_`s in the comment above because there was stack overflow in the debug build for me,
    // but there was no stack overflow in the release build, possibly because of optimizations made.

    // This is really sad that currently there is no simple, unsafe-free, guaranteed solution for this.

    // Seems the `placement new` feature could resolve this, but it was eventually removed from unstable.
    // See https://github.com/rust-lang/rust/issues/27779#issuecomment-378416911.

    // Still, currently, there are other solutions for this, for example below.
    let boxed = {
        let layout = std::alloc::Layout::new::<BigStruct>();
        unsafe {
            let raw = std::alloc::alloc_zeroed(layout) as *mut BigStruct;
            Box::from_raw(raw)
        }
    };
    // But not sure how to initialize some custom data that should not be just zeroed.
}
