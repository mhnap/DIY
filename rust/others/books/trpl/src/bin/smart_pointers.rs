// https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

// A pointer is a general concept for a variable that contains an address in memory.
// This address refers to, or “points at,” some other data.
// The most common kind of pointer in Rust is a reference.
// References are indicated by the & symbol and borrow the value they point to.
// They don’t have any special capabilities other than referring to data, and have no overhead.

// Smart pointers, on the other hand, are data structures that act like a pointer but also have additional metadata and capabilities.
// The concept of smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist in other languages as well.
// Rust has a variety of smart pointers defined in the standard library that provide functionality beyond that provided by references.

// Rust, with its concept of ownership and borrowing, has an additional difference between references and smart pointers: while references only borrow data, in many cases, smart pointers own the data they point to.

// Smart pointers are usually implemented using structs.
// Unlike an ordinary struct, smart pointers implement the Deref and Drop traits.
// The Deref trait allows an instance of the smart pointer struct to behave like a reference so you can write your code to work with either references or smart pointers.
// The Drop trait allows you to customize the code that’s run when an instance of the smart pointer goes out of scope.

use std::cell::RefCell;
use std::rc::{Rc, Weak};

fn main() {
    // https://doc.rust-lang.org/book/ch15-01-box.html

    // The most straightforward smart pointer is a box, whose type is written Box<T>.
    // Boxes allow you to store data on the heap rather than the stack.
    // What remains on the stack is the pointer to the heap data.

    // Boxes don’t have performance overhead, other than storing their data on the heap instead of on the stack.
    // But they don’t have many extra capabilities either.
    // You’ll use them most often in these situations:
    // - When you have a type whose size can’t be known at compile time and you want to use a value of that type in a context that requires an exact size.
    // - When you have a large amount of data and you want to transfer ownership but ensure the data won’t be copied when you do so.
    // - When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type.

    //

    // Before we discuss the heap storage use case for Box<T>, we’ll cover the syntax and how to interact with values stored within a Box<T>.
    let b = Box::new(5);
    println!("b = {}", b);
    // We define the variable b to have the value of a Box that points to the value 5, which is allocated on the heap.
    // This program will print b = 5; in this case, we can access the data in the box similar to how we would if this data were on the stack.
    // Just like any owned value, when a box goes out of scope, as b does at the end of main, it will be deallocated.
    // The deallocation happens both for the box (stored on the stack) and the data it points to (stored on the heap).

    //

    // A value of recursive type can have another value of the same type as part of itself.
    // Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up.
    // However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs.
    // Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

    // As an example of a recursive type, let’s explore the cons list.
    // This is a data type commonly found in functional programming languages.
    // The cons list type we’ll define is straightforward except for the recursion; therefore, the concepts in the example we’ll work with will be useful any time you get into more complex situations involving recursive types.

    // enum List {
    //     Cons(i32, List),
    //     Nil,
    // }
    // error[E0072]: recursive type `List` has infinite size
    //   --> src/lessons/smart_pointers.rs:54:5
    //    |
    // 54 |     enum List {
    //    |     ^^^^^^^^^
    // 55 |         Cons(i32, List),
    //    |                   ---- recursive without indirection
    //    |
    // help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to break the cycle
    //    |
    // 55 |         Cons(i32, Box<List>),
    //    |                   ++++    +

    // The error shows this type “has infinite size.”
    // The reason is that we’ve defined List with a variant that is recursive: it holds another value of itself directly.
    // As a result, Rust can’t figure out how much space it needs to store a List value.

    // In help suggestion, “indirection” means that instead of storing a value directly, we should change the data structure to store the value indirectly by storing a pointer to the value instead.
    // Because a Box<T> is a pointer, Rust always knows how much space a Box<T> needs: a pointer’s size doesn’t change based on the amount of data it’s pointing to.
    // This means we can put a Box<T> inside the Cons variant instead of another List value directly.
    // The Box<T> will point to the next List value that will be on the heap rather than inside the Cons variant.
    // Conceptually, we still have a list, created with lists holding other lists, but this implementation is now more like placing the items next to one another rather than inside one another.

    {
        #[derive(Debug)]
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
        dbg!(&list);
    }

    // The Cons variant needs the size of an i32 plus the space to store the box’s pointer data.
    // The Nil variant stores no values, so it needs less space than the Cons variant.
    // We now know that any List value will take up the size of an i32 plus the size of a box’s pointer data.
    // By using a box, we’ve broken the infinite, recursive chain, so the compiler can figure out the size it needs to store a List value.

    //

    // https://doc.rust-lang.org/book/ch15-02-deref.html

    // Implementing the Deref trait allows you to customize the behavior of the dereference operator * (not to be confused with the multiplication or glob operator).
    // By implementing Deref in such a way that a smart pointer can be treated like a regular reference, you can write code that operates on references and use that code with smart pointers too.

    // A regular reference is a type of pointer, and one way to think of a pointer is as an arrow to a value stored somewhere else.
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y);
    // error[E0277]: can't compare `{integer}` with `&{integer}`
    //    --> src/lessons/smart_pointers.rs:109:5
    //     |
    // 109 |     assert_eq!(5, y);
    //     |     ^^^^^^^^^^^^^^^^ no implementation for `{integer} == &{integer}`
    //     |
    //     = help: the trait `PartialEq<&{integer}>` is not implemented for `{integer}`
    //     = help: the following other types implement trait `PartialEq<Rhs>`:
    //               f32
    //               f64
    //               i128
    //               i16
    //               i32
    //               i64
    //               i8
    //               isize
    //             and 6 others
    //     = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

    // Comparing a number and a reference to a number isn’t allowed because they’re different types.
    // We must use the dereference operator to follow the reference to the value it’s pointing to.
    assert_eq!(5, *y);

    //

    // We can rewrite the code to use a Box<T> instead of a reference.
    // The dereference operator used on the Box<T> functions in the same way as the dereference operator used on the reference.
    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // The main difference between reference version is that here we set y to be an instance of a Box<T> pointing to a copied value of x rather than a reference pointing to the value of x.
    // In the last assertion, we can use the dereference operator to follow the pointer of the Box<T> in the same way that we did when y was a reference.

    //

    // Let’s build a smart pointer similar to the Box<T> type provided by the standard library to experience how smart pointers behave differently from references by default.
    // Then we’ll look at how to add the ability to use the dereference operator.

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // assert_eq!(5, *y);
    // error[E0614]: type `MyBox<{integer}>` cannot be dereferenced
    //    --> src/lessons/smart_pointers.rs:163:19
    //     |
    // 163 |     assert_eq!(5, *y);
    //     |                   ^^

    // Our MyBox<T> type can’t be dereferenced because we haven’t implemented that ability on our type.
    // To enable dereferencing with the * operator, we need to implement the Deref trait.

    //

    // The Deref trait, provided by the standard library, requires us to implement one method named deref that borrows self and returns a reference to the inner data.
    use std::ops::Deref;

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    assert_eq!(5, *y);

    // Without the Deref trait, the compiler can only dereference & references.
    // The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.

    // Behind the scenes Rust actually ran this code.
    assert_eq!(5, *(y.deref()));

    // Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method.
    // This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.

    // The reason the deref method returns a reference to a value, and that the plain dereference outside the parentheses in *(y.deref()) is still necessary, is to do with the ownership system.
    // If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self.
    // We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.

    //

    // Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
    // For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.
    // Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait.
    // It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.
    // A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

    // Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *.
    // The deref coercion feature also lets us write more code that can work for either references or smart pointers.

    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // Here we’re calling the hello function with the argument &m, which is a reference to a MyBox<String> value.
    // Because we implemented the Deref trait on MyBox<T>, Rust can turn &MyBox<String> into &String by calling deref.
    // The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref.
    // Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.

    // If Rust didn’t implement deref coercion, we would have to write the code below instead to call hello with a value of type &MyBox<String>.
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);

    // The (*m) dereferences the MyBox<String> into a String.
    // Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.
    // This code without deref coercions is harder to read, write, and understand with all of these symbols involved.
    // Deref coercion allows Rust to handle these conversions for us automatically.

    // When the Deref trait is defined for the types involved, Rust will analyze the types and use Deref::deref as many times as necessary to get a reference to match the parameter’s type.
    // The number of times that Deref::deref needs to be inserted is resolved at compile time, so there is no runtime penalty for taking advantage of deref coercion!

    //

    // Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

    // Rust does deref coercion when it finds types and trait implementations in three cases:
    // - From &T to &U when T: Deref<Target=U>
    // - From &mut T to &mut U when T: DerefMut<Target=U>
    // - From &mut T to &U when T: Deref<Target=U>

    // The first two cases are the same as each other except that the second implements mutability.
    // The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently.
    // The second case states that the same deref coercion happens for mutable references.

    // The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
    // But the reverse is not possible: immutable references will never coerce to mutable references.
    // Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
    // Converting one mutable reference to one immutable reference will never break the borrowing rules.
    // Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that.
    // Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.

    //

    // Move a value from the stack to the heap by creating a Box.
    let val = "Hi".to_string();
    let boxed = Box::new(val);
    // dbg!(&val); // error[E0382]: borrow of moved value: `val`
    dbg!(&boxed);

    // Move a value from a Box back to the stack by dereferencing.
    let val = *boxed;
    dbg!(&val);

    // Cannot dereference again as value is already moved.
    // let val = *boxed; // error[E0382]: use of moved value: `*boxed`

    //

    // https://doc.rust-lang.org/book/ch15-03-drop.html

    // The second trait important to the smart pointer pattern is Drop, which lets you customize what happens when a value is about to go out of scope.
    // You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections.

    // You specify the code to run when a value goes out of scope by implementing the Drop trait.
    // The Drop trait requires you to implement one method named drop that takes a mutable reference to self.

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`!", self.data);
        }
    }

    {
        let c = CustomSmartPointer { data: String::from("my stuff") };
        let d = CustomSmartPointer { data: String::from("other stuff") };

        println!("CustomSmartPointers created.");
    }

    // The Drop trait is included in the prelude, so we don’t need to bring it into scope.

    // At the end, our instances of CustomSmartPointer will go out of scope, and Rust will call the code we put in the drop method, printing our final message.
    // Note that we didn’t need to call the drop method explicitly.

    // Rust automatically called drop for us when our instances went out of scope, calling the code we specified.
    // Variables are dropped in the reverse order of their creation, so d was dropped before c.

    //

    // Unfortunately, it’s not straightforward to disable the automatic drop functionality.
    // Disabling drop isn’t usually necessary; the whole point of the Drop trait is that it’s taken care of automatically.
    // Occasionally, however, you might want to clean up a value early.

    // Rust doesn’t let you call the Drop trait’s drop method manually.
    let c = CustomSmartPointer { data: String::from("my stuff") };
    // c.drop();
    // error[E0040]: explicit use of destructor method
    //    --> src/lessons/smart_pointers.rs:321:7
    //     |
    // 321 |     c.drop();
    //     |     --^^^^--
    //     |     | |
    //     |     | explicit destructor calls not allowed
    //     |     help: consider using `drop` function: `drop(c)`

    // This error message states that we’re not allowed to explicitly call drop.
    // The error message uses the term destructor, which is the general programming term for a function that cleans up an instance.
    // A destructor is analogous to a constructor, which creates an instance.
    // The drop function in Rust is one particular destructor.

    // Rust doesn’t let us call drop explicitly because Rust would still automatically call drop on the value at the end of main.
    // This would cause a double free error because Rust would be trying to clean up the same value twice.
    // We can’t disable the automatic insertion of drop when a value goes out of scope, and we can’t call the drop method explicitly.

    // So, if we need to force a value to be cleaned up early, we use the std::mem::drop function.
    std::mem::drop(c);
    println!("c is dropped here!");

    // You can use code specified in a Drop trait implementation in many ways to make cleanup convenient and safe: for instance, you could use it to create your own memory allocator!
    // With the Drop trait and Rust’s ownership system, you don’t have to remember to clean up because Rust does it automatically.
    // You also don’t have to worry about problems resulting from accidentally cleaning up values still in use: the ownership system that makes sure references are always valid also ensures that drop gets called only once when the value is no longer being used.

    //

    // https://doc.rust-lang.org/book/ch15-04-rc.html

    // In the majority of cases, ownership is clear: you know exactly which variable owns a given value.
    // However, there are cases when a single value might have multiple owners.
    // For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all of the edges that point to it.
    // A node shouldn’t be cleaned up unless it doesn’t have any edges pointing to it and so has no owners.

    // You have to enable multiple ownership explicitly by using the Rust type Rc<T>, which is an abbreviation for reference counting.
    // The Rc<T> type keeps track of the number of references to a value to determine whether or not the value is still in use.
    // If there are zero references to a value, the value can be cleaned up without any references becoming invalid.

    // Note that Rc<T> is only for use in single-threaded scenarios.

    //

    // We’ll create two lists that both share ownership of a third list.

    {
        // Implement this scenario using our definition of List with Box<T> won’t work.

        // enum List {
        //     Cons(i32, Box<List>),
        //     Nil,
        // }
        //
        // use List::{Cons, Nil};
        //
        // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
        // let b = Cons(3, Box::new(a));
        // let c = Cons(4, Box::new(a));

        // The Cons variants own the data they hold, so when we create the b list, a is moved into b and b owns a.
        // Then, when we try to use a again when creating c, we’re not allowed to because a has been moved.
    }

    // We could change the definition of Cons to hold references instead, but then we would have to specify lifetime parameters.
    // By specifying lifetime parameters, we would be specifying that every element in the list will live at least as long as the entire list.
    // This is the case for the elements and lists in this code, but not in every scenario.

    // Instead, we’ll change our definition of List to use Rc<T> in place of Box<T>.
    {
        #[derive(Debug)]
        enum List {
            Cons(i32, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        let b = Cons(3, Rc::clone(&a));
        let c = Cons(4, Rc::clone(&a));

        dbg!(a, b, c);

        // Each Cons variant will now hold a value and an Rc<T> pointing to a List.
        // When we create b, instead of taking ownership of a, we’ll clone the Rc<List> that a is holding, thereby increasing the number of references from one to two and letting a and b share ownership of the data in that Rc<List>.
        // We’ll also clone a when creating c, increasing the number of references from two to three.
        // Every time we call Rc::clone, the reference count to the data within the Rc<List> will increase, and the data won’t be cleaned up unless there are zero references to it.

        // We could have called a.clone() rather than Rc::clone(&a), but Rust’s convention is to use Rc::clone in this case.
        // The implementation of Rc::clone doesn’t make a deep copy of all the data like most types’ implementations of clone do.
        // The call to Rc::clone only increments the reference count, which doesn’t take much time.
        // Deep copies of data can take a lot of time.
        // By using Rc::clone for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones that increase the reference count.
        // When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to Rc::clone.

        //

        // We can see the reference counts changing as we create and drop references.
        let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
        println!("count after creating a = {}", Rc::strong_count(&a));
        let b = Cons(3, Rc::clone(&a));
        println!("count after creating b = {}", Rc::strong_count(&a));
        {
            let c = Cons(4, Rc::clone(&a));
            println!("count after creating c = {}", Rc::strong_count(&a));
        }
        println!("count after c goes out of scope = {}", Rc::strong_count(&a));

        // At each point in the program where the reference count changes, we print the reference count, which we get by calling the Rc::strong_count function.
        // This function is named strong_count rather than count because the Rc<T> type also has a weak_count.
    }

    // Via immutable references, Rc<T> allows you to share data between multiple parts of your program for reading only.
    // If Rc<T> allowed you to have multiple mutable references too, you might violate one of the borrowing rules: multiple mutable borrows to the same place can cause data races and inconsistencies.

    //

    // https://doc.rust-lang.org/book/ch15-05-interior-mutability.html

    // Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data; normally, this action is disallowed by the borrowing rules.
    // To mutate data, the pattern uses unsafe code inside a data structure to bend Rust’s usual rules that govern mutation and borrowing.
    // Unsafe code indicates to the compiler that we’re checking the rules manually instead of relying on the compiler to check them for us.

    // We can use types that use the interior mutability pattern only when we can ensure that the borrowing rules will be followed at runtime, even though the compiler can’t guarantee that.
    // The unsafe code involved is then wrapped in a safe API, and the outer type is still immutable.

    //

    // Unlike Rc<T>, the RefCell<T> type represents single ownership over the data it holds.
    // With references and Box<T>, the borrowing rules’ invariants are enforced at compile time.
    // With RefCell<T>, these invariants are enforced at runtime.
    // With references, if you break these rules, you’ll get a compiler error.
    // With RefCell<T>, if you break these rules, your program will panic and exit.

    // The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

    // Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

    // Mutating the value inside an immutable value is the interior mutability pattern.

    //

    // When creating immutable and mutable references, we use the & and &mut syntax, respectively.
    // With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>.
    // The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>.
    // Both types implement Deref, so we can treat them like regular references.

    let refcell = RefCell::new("hello".to_string());
    dbg!(refcell.borrow());
    refcell.borrow_mut().push('!');
    dbg!(refcell.borrow());

    // The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active.
    // Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active.
    // When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one.
    // Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.

    // If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of RefCell<T> will panic at runtime.
    let mutref = refcell.borrow_mut();
    // refcell.borrow_mut(); // thread 'main' panicked at 'already borrowed: BorrowMutError'

    //

    // A common way to use RefCell<T> is in combination with Rc<T>.
    // Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data.
    // If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!
    {
        #[derive(Debug)]
        enum List {
            Cons(Rc<RefCell<i32>>, Rc<List>),
            Nil,
        }

        use List::{Cons, Nil};

        let value = Rc::new(RefCell::new(5));

        let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

        let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
        let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

        *value.borrow_mut() += 10;

        println!("a after = {:?}", a);
        println!("b after = {:?}", b);
        println!("c after = {:?}", c);

        // After we’ve created the lists in a, b, and c, we want to add 10 to the value in value.
        // We do this by calling borrow_mut on value, which uses the automatic dereferencing feature to dereference the Rc<T> to the inner RefCell<T> value.
        // The borrow_mut method returns a RefMut<T> smart pointer, and we use the dereference operator on it and change the inner value.
    }

    //

    // https://doc.rust-lang.org/book/ch15-06-reference-cycles.html

    // Rust’s memory safety guarantees make it difficult, but not impossible, to accidentally create memory that is never cleaned up (known as a memory leak).
    // Preventing memory leaks entirely is not one of Rust’s guarantees, meaning memory leaks are memory safe in Rust.
    // We can see that Rust allows memory leaks by using Rc<T> and RefCell<T>: it’s possible to create references where items refer to each other in a cycle.
    // This creates memory leaks because the reference count of each item in the cycle will never reach 0, and the values will never be dropped.

    {
        use List::{Cons, Nil};

        #[derive(Debug)]
        enum List {
            Cons(i32, RefCell<Rc<List>>),
            Nil,
        }

        impl List {
            fn tail(&self) -> Option<&RefCell<Rc<List>>> {
                match self {
                    Cons(_, item) => Some(item),
                    Nil => None,
                }
            }
        }

        let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

        println!("a initial rc count = {}", Rc::strong_count(&a));
        println!("a next item = {:?}", a.tail());

        let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

        println!("a rc count after b creation = {}", Rc::strong_count(&a));
        println!("b initial rc count = {}", Rc::strong_count(&b));
        println!("b next item = {:?}", b.tail());

        if let Some(link) = a.tail() {
            *link.borrow_mut() = Rc::clone(&b);
        }

        println!("b rc count after changing a = {}", Rc::strong_count(&b));
        println!("a rc count after changing a = {}", Rc::strong_count(&a));

        // Uncomment the next line to see that we have a cycle;
        // it will overflow the stack
        // println!("a next item = {:?}", a.tail());

        // The reference count of the Rc<List> instances in both a and b are 2 after we change the list in a to point to b.
        // At the end of main, Rust drops the variable b, which decreases the reference count of the b Rc<List> instance from 2 to 1.
        // The memory that Rc<List> has on the heap won’t be dropped at this point, because its reference count is 1, not 0.
        // Then Rust drops a, which decreases the reference count of the a Rc<List> instance from 2 to 1 as well.
        // This instance’s memory can’t be dropped either, because the other Rc<List> instance still refers to it.
        // The memory allocated to the list will remain uncollected forever.

        // Creating reference cycles is not easily done, but it’s not impossible either.
        // If you have RefCell<T> values that contain Rc<T> values or similar nested combinations of types with interior mutability and reference counting, you must ensure that you don’t create cycles; you can’t rely on Rust to catch them.
        // Creating a reference cycle would be a logic bug in your program that you should use automated tests, code reviews, and other software development practices to minimize.
    }

    // Another solution for avoiding reference cycles is reorganizing your data structures so that some references express ownership and some references don’t.
    // As a result, you can have cycles made up of some ownership relationships and some non-ownership relationships, and only the ownership relationships affect whether or not a value can be dropped.
    // In above case, we always want Cons variants to own their list, so reorganizing the data structure isn’t possible.
    // Let’s look at an example using graphs made up of parent nodes and child nodes to see when non-ownership relationships are an appropriate way to prevent reference cycles.

    //

    // Calling Rc::clone increases the strong_count of an Rc<T> instance, and an Rc<T> instance is only cleaned up if its strong_count is 0.
    // It is possible also to create a weak reference to the value within an Rc<T> instance by calling Rc::downgrade and passing a reference to the Rc<T>.
    // Strong references are how you can share ownership of an Rc<T> instance.
    // Weak references don’t express an ownership relationship, and their count doesn’t affect when an Rc<T> instance is cleaned up.
    // They won’t cause a reference cycle because any cycle involving some weak references will be broken once the strong reference count of values involved is 0.

    // When you call Rc::downgrade, you get a smart pointer of type Weak<T>.
    // Instead of increasing the strong_count in the Rc<T> instance by 1, calling Rc::downgrade increases the weak_count by 1.
    // The Rc<T> type uses weak_count to keep track of how many Weak<T> references exist, similar to strong_count.
    // The difference is the weak_count doesn’t need to be 0 for the Rc<T> instance to be cleaned up.

    // Because the value that Weak<T> references might have been dropped, to do anything with the value that a Weak<T> is pointing to, you must make sure the value still exists.
    // Do this by calling the upgrade method on a Weak<T> instance, which will return an Option<Rc<T>>.
    // You’ll get a result of Some if the Rc<T> value has not been dropped yet and a result of None if the Rc<T> value has been dropped.
    // Because upgrade returns an Option<Rc<T>>, Rust will ensure that the Some case and the None case are handled, and there won’t be an invalid pointer.

    // As an example, rather than using a list whose items know only about the next item, we’ll create a tree whose items know about their children items and their parent items.
    {
        #[derive(Debug)]
        struct Node {
            value: i32,
            children: RefCell<Vec<Rc<Node>>>,
        }

        let leaf = Rc::new(Node { value: 3, children: RefCell::new(vec![]) });
        println!("leaf = {:?}", leaf);

        let branch = Rc::new(Node { value: 5, children: RefCell::new(vec![Rc::clone(&leaf)]) });
        println!("branch = {:?}", branch);

        // We clone the Rc<Node> in leaf and store that in branch, meaning the Node in leaf now has two owners: leaf and branch.
        // We can get from branch to leaf through branch.children, but there’s no way to get from leaf to branch.
        // The reason is that leaf has no reference to branch and doesn’t know they’re related.
        // We want leaf to know that branch is its parent.
        // We’ll do that next.
    }

    //

    // To make the child node aware of its parent, we need to add a parent field to our Node struct definition.
    // The trouble is in deciding what the type of parent should be.
    // We know it can’t contain an Rc<T>, because that would create a reference cycle with leaf.parent pointing to branch and branch.children pointing to leaf, which would cause their strong_count values to never be 0.

    // Thinking about the relationships another way, a parent node should own its children: if a parent node is dropped, its child nodes should be dropped as well.
    // However, a child should not own its parent: if we drop a child node, the parent should still exist.
    // This is a case for weak references!

    // So instead of Rc<T>, we’ll make the type of parent use Weak<T>, specifically a RefCell<Weak<Node>>.

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    // A node will be able to refer to its parent node but doesn’t own its parent.

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // The lack of infinite output indicates that this code didn’t create a reference cycle.

    //

    // Let’s look at how the strong_count and weak_count values of the Rc<Node> instances change by creating a new inner scope and moving the creation of branch into that scope.
    // By doing so, we can see what happens when branch is created and then dropped when it goes out of scope.

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),);

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch),
        );

        println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),);
    }

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!("leaf strong = {}, weak = {}", Rc::strong_count(&leaf), Rc::weak_count(&leaf),);

    // All of the logic that manages the counts and value dropping is built into Rc<T> and Weak<T> and their implementations of the Drop trait.
    // By specifying that the relationship from a child to its parent should be a Weak<T> reference in the definition of Node, you’re able to have parent nodes point to child nodes and vice versa without creating a reference cycle and memory leaks.
}
