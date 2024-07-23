// https://vojtechkral.github.io/blag/rust-drop-order/
// https://doc.rust-lang.org/std/ops/trait.Drop.html#drop-order
// https://doc.rust-lang.org/reference/destructors.html

struct Token(&'static str);

impl Drop for Token {
    fn drop(&mut self) {
        println!("[drop] {}", self.0);
    }
}

#[allow(unused)]
fn scope() {
    // Temporary value without binding will be dropped immediately.
    let _ = Token("_ pattern");

    let t1 = Token("1");
    // Value will not be moved and thus not dropped to `_` binding.
    let _ = t1;
    println!("t1 created");

    let t2 = Token("2");
    // `1` will be moved to new shadowed variable and thus will be dropped sooner in reverse order.
    // Drop for `t1` will never be called, as it's moved.
    let t2 = t1;
    println!("t2 shadowed");

    let mut t3 = Token("3");
    // Value `3` will be dropped after assigning `4`.
    t3 = Token("4");
    println!("t3 re-assigned");
}

fn exprs() {
    fn make_token(s: &'static str) -> Result<Token, ()> {
        Ok(Token(s))
    }

    // Will live till the end of `match`.
    match make_token("matched token").is_ok() {
        true => println!("match arm"),
        false => unreachable!(),
    }
    println!("after match");

    // Will live till the end of `if let`.
    if let Ok(_) = make_token("if let token") {
        println!("if let body");
    }
    println!("after if let");

    // Will be dropped after converting to bool.
    if make_token("if token").is_ok() {
        println!("if body");
    }
    println!("after if");
}

fn fn_args() {
    #[allow(unused)]
    fn takes_args(t1: Token, (_, t2): (Token, Token), _: Token) {
        println!("function body");
    }

    // Function arguments are dropped in reverse order.
    // Tuple values should be dropped in order of the sequence, but their bindings change order.
    takes_args(Token("t1"), (Token("t2.0"), Token("t2.1")), Token("_"));
}

fn fn_final_value() {
    #[allow(unused)]
    fn final_value(t1: Token) -> &'static str {
        let t2 = Token("in body");
        Token("final value").0
    }

    // `final value` gets dropped after all local variables (but before arguments).
    let res = final_value(Token("arg"));
    println!("{}", res);
}

#[allow(unused)]
fn structs() {
    struct SomeStruct {
        a: Token,
        b: Token,
        c: Token,
    }

    // struct fields are dropped in the same order as declared in the struct (source-code-wise, not memory-layout-wise).
    let s = SomeStruct {
        b: Token("b"),
        a: Token("a"),
        c: Token("c"),
    };
}

macro_rules! section {
    ($fn:ident) => {
        println!("\n=== {} ===", stringify!($fn));
        $fn()
    };
}

fn main() {
    section!(scope);
    section!(exprs);
    section!(fn_args);
    section!(fn_final_value);
    section!(structs);
}
