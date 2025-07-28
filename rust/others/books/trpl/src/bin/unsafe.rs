// https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html

// Rust has a second language hidden inside it that doesn’t enforce these memory safety guarantees: it’s called unsafe Rust and works just like regular Rust, but gives us extra superpowers.

// The reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.
// If Rust didn’t let you do unsafe operations, you couldn’t do certain tasks.
// Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even writing your own operating system.

fn main() {
    // To switch to unsafe Rust, use the unsafe keyword and then start a new block that holds the unsafe code.
    // You can take five actions in unsafe Rust that you can’t in safe Rust, which we call unsafe superpowers.
    // Those superpowers include the ability to:
    // - Dereference a raw pointer
    // - Call an unsafe function or method
    // - Access or modify a mutable static variable
    // - Implement an unsafe trait
    // - Access fields of unions

    // It’s important to understand that unsafe doesn’t turn off the borrow checker or disable any other of Rust’s safety checks: if you use a reference in unsafe code, it will still be checked.

    //

    // Unsafe Rust has two new types called raw pointers that are similar to references.
    // As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
    // The asterisk isn’t the dereference operator; it’s part of the type name.
    // In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

    // Different from references and smart pointers, raw pointers:
    // - Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
    // - Aren’t guaranteed to point to valid memory
    // - Are allowed to be null
    // - Don’t implement any automatic cleanup

    // By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or the ability to interface with another language or hardware where Rust’s guarantees don’t apply.

    // Creating raw pointers from references.
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    dbg!(num, r1, r2);

    // Notice that we don’t include the unsafe keyword in this code.
    // We can create raw pointers in safe code; we just can’t dereference raw pointers outside an unsafe block.

    // Creating a raw pointer to an arbitrary memory address.
    let address = 0x012345usize;
    let r = address as *const i32;
    dbg!(r);

    // Dereferencing raw pointers needs an unsafe block.
    // dbg!(*r1, *r2);
    // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
    //   --> src/lessons/unsafe.rs:51:10
    //    |
    // 51 |     dbg!(*r1, *r2);
    //    |          ^^^ dereference of raw pointer
    //    |
    //    = note: raw pointers may be null, dangling or unaligned; they can violate aliasing rules and cause data races: all of these are undefined behavior
    unsafe {
        dbg!(*r1, *r2);
    }

    // Creating a pointer does no harm; it’s only when we try to access the value that it points at that we might end up dealing with an invalid value.

    // unsafe {
    //     dbg!(*r);
    // }
    // thread 'main' panicked at 'misaligned pointer dereference: address must be a multiple of 0x4 but is 0x12345', src/lessons/unsafe.rs:66:9

    // unsafe {
    //     dbg!(*(0x012344usize as *const i32));
    // }
    // Process finished with exit code 139 (interrupted by signal 11: SIGSEGV)

    //

    // The second type of operation you can perform in an unsafe block is calling unsafe functions.
    // Unsafe functions and methods look exactly like regular functions and methods, but they have an extra unsafe before the rest of the definition.
    // The unsafe keyword in this context indicates the function has requirements we need to uphold when we call this function, because Rust can’t guarantee we’ve met these requirements.
    // By calling an unsafe function within an unsafe block, we’re saying that we’ve read this function’s documentation and take responsibility for upholding the function’s contracts.
    unsafe fn dangerous() {
        // Bodies of unsafe functions are effectively unsafe blocks, so to perform other unsafe operations within an unsafe function, we don’t need to add another unsafe block.
        dbg!(*(&5 as *const i32));
    }
    // dangerous();
    // error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
    //   --> src/lessons/unsafe.rs:82:5
    //    |
    // 82 |     dangerous();
    //    |     ^^^^^^^^^^^ call to unsafe function
    //    |
    //    = note: consult the function's documentation for information on how to avoid undefined behavior
    unsafe {
        dangerous();
    }

    //

    // Just because a function contains unsafe code doesn’t mean we need to mark the entire function as unsafe.
    // In fact, wrapping unsafe code in a safe function is a common abstraction.
    fn not_dangerous() {
        unsafe {
            dbg!(*(&5 as *const i32));
        }
    }
    not_dangerous();

    //

    // Sometimes, your Rust code might need to interact with code written in another language.
    // For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI).
    // An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

    // Functions declared within extern blocks are always unsafe to call from Rust code.
    // The reason is that other languages don’t enforce Rust’s rules and guarantees, and Rust can’t check them, so responsibility falls on the programmer to ensure safety.
    unsafe extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // Within the extern "C" block, we list the names and signatures of external functions from another language we want to call.
    // The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level.
    // The "C" ABI is the most common and follows the C programming language’s ABI.

    // We can also use extern to create an interface that allows other languages to call Rust functions.
    #[unsafe(no_mangle)]
    pub extern "C" fn call_from_c() {
        println!("Just called a Rust function from C!");
    }
    call_from_c();

    //

    // We’ve not yet talked about global variables, which Rust does support but can be problematic with Rust’s ownership rules.
    // If two threads are accessing the same mutable global variable, it can cause a data race.
    // In Rust, global variables are called static variables.
    static HELLO_WORLD: &str = "Hello, world!";
    println!("name is: {}", HELLO_WORLD);

    // Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.
    // Accessing an immutable static variable is safe.

    // A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
    // Using the value will always access the same data.
    // Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
    // Another difference is that static variables can be mutable.
    // Accessing and modifying mutable static variables is unsafe.
    static mut COUNTER: u32 = 0;
    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }
    add_to_count(3);
    println!("COUNTER: {}", unsafe { COUNTER });

    // This code compiles and prints COUNTER: 3 as we would expect because it’s single threaded.
    // Having multiple threads access COUNTER would likely result in data races.

    //

    // We can use unsafe to implement an unsafe trait.
    // A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
    unsafe trait Foo {
        // methods go here
    }
    unsafe impl Foo for i32 {
        // method implementations go here
    }

    // If we implement a type that contains a type that is not Send or Sync, such as raw pointers, and we want to mark that type as Send or Sync, we must use unsafe.
    // Rust can’t verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple threads; therefore, we need to do those checks manually and indicate as such with unsafe.

    //

    // The final action that works only with unsafe is accessing fields of a union.
    // A union is similar to a struct, but only one declared field is used in a particular instance at one time.
    // Unions are primarily used to interface with unions in C code.
    // Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.

    //

    // Using unsafe to take one of the five actions (superpowers) just discussed isn’t wrong or even frowned upon.
    // But it is trickier to get unsafe code correct because the compiler can’t help uphold memory safety.
    // When you have a reason to use unsafe code, you can do so, and having the explicit unsafe annotation makes it easier to track down the source of problems when they occur.
}
