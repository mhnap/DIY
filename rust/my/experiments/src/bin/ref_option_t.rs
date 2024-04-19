// https://www.youtube.com/watch?v=6c7pZYP_iIE
// https://www.reddit.com/r/rust/comments/wqsxk2/is_it_better_to_pass_optiont_or_optiont/
// https://users.rust-lang.org/t/api-design-option-t-vs-option-t/34139

#[derive(Debug)]
struct Data;

impl Data {
    fn crunch(&self) -> i32 {
        42
    }
}

#[derive(Debug, Default)]
struct Widget {
    data: Option<Data>,
    // What if `data` type should be changed to `Option<Box<Data>>`?
}

impl Widget {
    fn data_a(&self) -> &Option<Data> {
        &self.data
    }

    fn data_b(&self) -> Option<&Data> {
        self.data.as_ref()
    }
}

fn main() {
    let widget = Widget::default();

    let a = widget.data_a();
    let b = widget.data_b();

    assert_eq!(a.is_some(), b.is_some());

    // let crunch = a.map(|data| data.crunch());
    //     error[E0507]: cannot move out of `*a` which is behind a shared reference
    //     --> experiments/src/bin/ref_option_t.rs:35:18
    //      |
    // 35   |     let crunch = a.map(|data| data.crunch());
    //      |                  ^ ------------------------- `*a` moved due to this method call
    //      |                  |
    //      |                  help: consider calling `.as_ref()` or `.as_mut()` to borrow the type's contents
    //      |                  move occurs because `*a` has type `Option<Data>`, which does not implement the `Copy` trait
    //      |
    // note: `Option::<T>::map` takes ownership of the receiver `self`, which moves `*a`
    //     --> /home/mhnap/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/option.rs:1067:22
    //      |
    // 1067 |     pub fn map<U, F>(self, f: F) -> Option<U>
    //      |                      ^^^^

    let _crunch = a.as_ref().map(|data| data.crunch());
    let _crunch = b.map(|data| data.crunch());
}
