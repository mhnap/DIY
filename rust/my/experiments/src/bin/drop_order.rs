// https://vojtechkral.github.io/blag/rust-drop-order
// https://doc.rust-lang.org/std/ops/trait.Drop.html#drop-order
// https://doc.rust-lang.org/reference/destructors.html
// https://github.com/rust-lang/lang-team/blob/master/design-meeting-minutes/2023-03-15-temporary-lifetimes.md
// https://blog.m-ou.se/super-let
// https://marabos.nl/atomics/basics.html#lifetime-of-mutexguard
// https://smallcultfollowing.com/babysteps/blog/2023/03/15/temporary-lifetimes
// https://blog.dureuill.net/articles/recurring-lifetime
// https://hackmd.io/@rust-lang-team/HJpRzPzoa
// https://hackmd.io/@rust-lang-team/BJZgkKqs6
// https://fasterthanli.me/articles/a-rust-match-made-in-hell
// https://github.com/rust-lang/rust/issues/93883
// https://rust-lang.github.io/rust-clippy/master/#/significant_drop_in_scrutinee
// https://doc.rust-lang.org/edition-guide/rust-2024/temporary-tail-expr-scope.html

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

    drop(t3);

    // Value can be reassigned after drop.
    t3 = Token("5");
}

fn exprs() {
    fn make_token(s: &'static str) -> Result<Token, ()> {
        Ok(Token(s))
    }

    // Will live till the end of `match`.
    match make_token("match token 1") {
        Ok(_) => println!("match arm 1"),
        Err(_) => unreachable!(),
    }
    println!("after match 1\n");

    // Even when no value can be referenced (in this case, but could return temp reference).
    match make_token("match token 2").is_ok() {
        true => println!("match arm 2"),
        false => unreachable!(),
    }
    println!("after match 2\n");

    // Will live till the end of `if let`.
    if let Ok(_) = make_token("if let token 1") {
        println!("if let body 1");
    } else {
        println!("if let else body 1");
    }
    println!("after if let 1\n");

    // Even in else branch.
    // Not true for 2024 edition.
    if let false = make_token("if let token 2").is_ok() {
        println!("if let body 2");
    } else {
        println!("if let else body 2");
    }
    println!("after if let 2\n");

    // But not in `if`.
    if make_token("if token").is_ok() {
        println!("if body");
    }
    println!("after if\n");

    // And not in `let else`.
    let Err(_) = make_token("let else token") else {
        println!("let else body");
        return;
    };
}

fn fn_args() {
    #[allow(unused)]
    fn takes_args(t1: &Token, (_, t2): (Token, Token), _: Token) {
        println!("function body");
    }

    // Function arguments are dropped in reverse order.
    // Tuple values should be dropped in order of the sequence, but their bindings change order.
    takes_args(&Token("t1"), (Token("t2.0"), Token("t2.1")), Token("_"));
    // Note that `t1` temporary lifetime is extended.

    // But not in a let statement.
    fn get_back(t: &Token) -> &Token {
        t
    }
    // let t = get_back(&Token("t1"));
    // drop(t);
    //     error[E0716]: temporary value dropped while borrowed
    //     --> my/experiments/src/bin/drop_order.rs:101:23
    //      |
    //  101 |     let t = get_back(&Token("t1"));
    //      |                       ^^^^^^^^^^^ - temporary value is freed at the end of this statement
    //      |                       |
    //      |                       creates a temporary value which is freed while still in use
    //  102 |     drop(t);
    //      |          - borrow later used here
    //      |
    //  help: consider using a `let` binding to create a longer lived value
    //      |
    //  101 ~     let binding = Token("t1");
    //  102 ~     let t = get_back(&binding);
    //      |
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
    let s = SomeStruct { b: Token("b"), a: Token("a"), c: Token("c") };
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

// === scope ===
// [drop] _ pattern
// t1 created
// t2 shadowed
// [drop] 3
// t3 re-assigned
// [drop] 4
// [drop] 1
// [drop] 2

// === exprs ===
// match arm 1
// [drop] match token 1
// after match 1

// match arm 2
// [drop] match token 2
// after match 2

// if let body 1
// [drop] if let token 1
// after if let 1

// if let else body 2
// [drop] if let token 2
// after if let 2

// [drop] if token
// if body
// after if

// [drop] let else token
// let else body

// === fn_args ===
// function body
// [drop] _
// [drop] t2.1
// [drop] t2.0
// [drop] t1

// === fn_final_value ===
// [drop] in body
// [drop] final value
// [drop] arg
// final value

// === structs ===
// [drop] a
// [drop] b
// [drop] c
