fn main() {
    let mut a = 1;

    // Generics match concrete type.
    fn foo<T>(t: T) {}
    foo(a); // T is i32
    foo(&a); // T is &i32
    foo(&mut a); // T is &mut i32

    // When there is ref or mut ref with a generic type, it adds just constraints.
    fn bar<T>(t: &T) {}
    // bar(a); // error[E0308]: mismatched types
    bar(&a); // T is &i32
    bar(&mut a); // T is &mut i32

    fn baz<T>(t: &mut T) {}
    // baz(a); // error[E0308]: mismatched types
    // baz(&a); // error[E0308]: mismatched types
    baz(&mut a); // T is &mut i32
}
