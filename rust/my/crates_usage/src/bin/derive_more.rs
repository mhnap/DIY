use my_practices::print_err;

fn main() {
    /// Needs manual [`std::fmt::Display`] and [`std::error::Error`] implementations.
    #[derive(Debug)]
    enum MyError1 {
        SomeErr(String),
    }

    impl std::fmt::Display for MyError1 {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                MyError1::SomeErr(err) => write!(f, "my error 1: {err}"),
            }
        }
    }

    impl std::error::Error for MyError1 {}

    let my_error1 = MyError1::SomeErr(String::from("some error"));
    print_err!(my_error1);

    //

    /// [`derive_more::Error`] take care of implementing [`std::error::Error`] and
    /// [`derive_more::Display`] take care of implementing [`std::fmt::Display`].
    #[derive(Debug, derive_more::Error, derive_more::Display)]
    enum MyError2 {
        #[display("my error 2: {_0}")]
        // https://github.com/JelteF/derive_more/issues/403#issuecomment-2314975408
        SomeErr(#[error(not(source))] String),
    }

    let my_error2 = MyError2::SomeErr(String::from("some another error"));
    print_err!(my_error2);

    //

    /// [`derive_more::From`] take care of implementing [`core::convert::From`].
    #[derive(Debug, derive_more::Error, derive_more::Display, derive_more::From)]
    enum MyError3 {
        #[display("my error 3")]
        SomeErr1(MyError2),

        #[display("my error 3")]
        #[from(ignore)]
        SomeErr2(MyError2),
    }

    let my_error3 = MyError3::from(my_error2);
    print_err!(my_error3);
}
