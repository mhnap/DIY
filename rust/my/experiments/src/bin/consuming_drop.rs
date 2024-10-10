// https://langdev.stackexchange.com/questions/3926/in-rust-why-did-the-designers-choose-to-make-drop-take-mut-self-instead-of-sel
// https://stackoverflow.com/questions/30905826/why-does-drop-take-mut-self-instead-of-self
// https://github.com/rust-lang/rust/issues/4330

fn main() {
    struct Foo {
        // ... some fields
    }

    fn do_something(foo: Foo) {
        // ... some code
        foo;
    }

    // impl Drop for Foo {
    //     fn drop(self) {
    //         do_something(self);
    //     }
    // }

    let foo = Foo {};
}
