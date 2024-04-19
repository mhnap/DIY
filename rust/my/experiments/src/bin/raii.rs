struct A {
    a: u8,
}

impl Drop for A {
    fn drop(&mut self) {
        println!("Drop {:?}", self.a);
    }
}

// impl Copy for A {}
// error[E0184]: the trait `Copy` cannot be implemented for this type; the type has a destructor
//   --> src/experiments/raii.rs:11:15
//    |
// 11 | impl Copy for A {}
//    |               ^ `Copy` not allowed on types with destructors

fn main() {
    let a1 = A { a: 42 };
    let a2 = A { a: 43 };
    let mut a3 = A { a: 44 };

    {
        let a4 = A { a: 45 };
        println!("Old a3 {}", a3.a);
        // Drop will be called for `a3` here.
        a3 = a4;
        println!("New a3 {}", a3.a);
    }

    // Can manually call drop.
    drop(a1);

    // Also, drop is called after moved object goes out of scope.
    fn foo(a: A) {}
    foo(a2);

    // Drop won't be called in this case because ownership is transferred back.
    fn bar(a: A) -> A {
        a
    }
    a3 = bar(a3);
    println!("New a3 {}", a3.a);

    // Drop will be called for `a3`.
}
