// https://www.lurklurk.org/effective-rust/raii.html
// Item 11: Implement the Drop trait for RAII patterns

fn main() {
    // RAII stands for "Resource Acquisition Is Initialization" which is a programming pattern where the lifetime of a value is exactly tied to the lifecycle of some additional resource.
    // The RAII pattern was popularized by the C++ programming language and is one of C++'s biggest contributions to programming.

    // The correlation between the lifetime of a value and the lifecycle of a resource is encoded in an RAII type:

    // The type's constructor acquires access to some resource
    // The type's destructor releases access to that resource
    // The result of this is that the RAII type has an invariant: access to the underlying resource is available if and only if the item exists.
    // Because the compiler ensures that local variables are destroyed at scope exit, this in turn means that the underlying resources are also released at scope exit.1

    // This is particularly helpful for maintainability: if a subsequent change to the code alters the control flow, item and resource lifetimes are still correct.
}
