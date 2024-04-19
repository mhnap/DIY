fn main() {
    // Primitive types are Copy by default.
    let a: u8 = 1;
    let b: u8 = a;
    dbg!(a);
    dbg!(b);

    // But structs don't derive Copy by default.
    #[derive(Debug)]
    struct A {
        i: u8,
    }
    let a = A { i: 1 };
    let b = a;
    // dbg!(a);
    dbg!(b);

    // But can be derived manually.
    #[derive(Debug, Copy, Clone)]
    struct B {
        i: u8,
    }
    let a = B { i: 1 };
    let b = a;
    dbg!(a);
    dbg!(b);

    // It's trivial to implement Copy and Clone deriving automatically by the compiler.
    // But it's not done intentionally, because if someone decides to add a not-Copy member to a type,
    // that is automatically marked as Copy, it would break all existing usage of this type.
    // And it's the developer's only responsibility to decide such things, not the compiler one.
    // Also, it would need some strange syntax to "underive" Copy if someone wanted manually, even when all members are Copy.
}
