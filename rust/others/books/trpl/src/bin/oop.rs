// https://doc.rust-lang.org/book/ch17-00-oop.html

// Need for later use.
trait Draw {
    fn draw(&self);
}

fn main() {
    // https://doc.rust-lang.org/book/ch17-01-what-is-oo.html

    // OOP languages share certain common characteristics, namely objects, encapsulation, and inheritance.
    // Let’s look at what each of those characteristics means and whether Rust supports it.

    //

    // The Gang of Four book defines OOP this way:
    // Object-oriented programs are made up of objects.
    // An object packages both data and the procedures that operate on that data.
    // The procedures are typically called methods or operations.

    // Using this definition, Rust is object-oriented: structs and enums have data, and impl blocks provide methods on structs and enums.
    // Even though structs and enums with methods aren’t called objects, they provide the same functionality, according to the Gang of Four’s definition of objects.

    //

    // Another aspect commonly associated with OOP is the idea of encapsulation, which means that the implementation details of an object aren’t accessible to code using that object.
    // Therefore, the only way to interact with an object is through its public API; code using the object shouldn’t be able to reach into the object’s internals and change data or behavior directly.
    // This enables the programmer to change and refactor an object’s internals without needing to change the code that uses the object.

    // We can use the pub keyword to decide which modules, types, functions, and methods in our code should be public, and by default everything else is private.
    mod averaged_collection {
        pub struct AveragedCollection {
            list: Vec<i32>,
            average: f64,
        }

        impl AveragedCollection {
            pub fn add(&mut self, value: i32) {
                self.list.push(value);
                self.update_average();
            }

            pub fn remove(&mut self) -> Option<i32> {
                let result = self.list.pop();
                match result {
                    Some(value) => {
                        self.update_average();
                        Some(value)
                    }
                    None => None,
                }
            }

            pub fn average(&self) -> f64 {
                self.average
            }

            fn update_average(&mut self) {
                let total: i32 = self.list.iter().sum();
                self.average = total as f64 / self.list.len() as f64;
            }
        }
    }

    // If encapsulation is a required aspect for a language to be considered object-oriented, then Rust meets that requirement.
    // The option to use pub or not for different parts of code enables encapsulation of implementation details.

    //

    // Inheritance is a mechanism whereby an object can inherit elements from another object’s definition, thus gaining the parent object’s data and behavior without you having to define them again.

    // If a language must have inheritance to be an object-oriented language, then Rust is not one.
    // There is no way to define a struct that inherits the parent struct’s fields and method implementations without using a macro.

    // However, if you’re used to having inheritance in your programming toolbox, you can use other solutions in Rust, depending on your reason for reaching for inheritance in the first place.

    // You would choose inheritance for two main reasons.

    // One is for reuse of code: you can implement particular behavior for one type, and inheritance enables you to reuse that implementation for a different type.
    // You can do this in a limited way in Rust code using default trait method implementations.

    // The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the parent type.
    // This is also called polymorphism, which means that you can substitute multiple objects for each other at runtime if they share certain characteristics.

    // Rust takes the different approach of using trait objects instead of inheritance.

    //

    // https://doc.rust-lang.org/book/ch17-02-trait-objects.html

    // A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.
    // We create a trait object by specifying some sort of pointer, such as a & reference or a Box<T> smart pointer, then the dyn keyword, and then specifying the relevant trait.
    // We can use trait objects in place of a generic or concrete type.
    // Wherever we use a trait object, Rust’s type system will ensure at compile time that any value used in that context will implement the trait object’s trait.
    // Consequently, we don’t need to know all the possible types at compile time.

    // We’ve mentioned that, in Rust, we refrain from calling structs and enums “objects” to distinguish them from other languages’ objects.
    // In a struct or enum, the data in the struct fields and the behavior in impl blocks are separated, whereas in other languages, the data and behavior combined into one concept is often labeled an object.
    // However, trait objects are more like objects in other languages in the sense that they combine data and behavior.
    // But trait objects differ from traditional objects in that we can’t add data to a trait object.
    // Trait objects aren’t as generally useful as objects in other languages: their specific purpose is to allow abstraction across common behavior.

    struct A;
    impl Draw for A {
        fn draw(&self) {
            println!("draw A!");
        }
    }
    let a = A;

    struct B;
    impl Draw for B {
        fn draw(&self) {
            println!("draw B!");
        }
    }
    let b = B;

    // Dynamic dispatch.
    mod gui_dynamic {
        use crate::Draw;

        pub struct Screen<'a> {
            pub components: Vec<&'a dyn Draw>,
        }

        impl<'a> Screen<'a> {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
    }

    let screen = gui_dynamic::Screen {
        components: vec![&a, &b],
    };
    screen.run();

    // This works differently from defining a struct that uses a generic type parameter with trait bounds.
    // A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

    // Static dispatch.
    mod gui_static {
        use crate::Draw;

        pub struct Screen<'a, T: Draw> {
            pub components: Vec<&'a T>,
        }

        impl<'a, T: Draw> Screen<'a, T> {
            pub fn run(&self) {
                for component in self.components.iter() {
                    component.draw();
                }
            }
        }
    }

    // let screen = gui_static::Screen {
    //     components: vec![&a, &b],
    // };
    // error[E0308]: mismatched types
    //    --> src/lessons/oop.rs:162:30
    //     |
    // 162 |         components: vec![&a, &b],
    //     |                              ^^ expected `&A`, found `&B`
    //     |
    //     = note: expected reference `&A`
    //                found reference `&B`

    // This restricts us to a Screen instance that has a list of components all of type A or all of type B.
    let screen_a = gui_static::Screen {
        components: vec![&a, &a],
    };
    screen_a.run();
    let screen_b = gui_static::Screen {
        components: vec![&b, &b],
    };
    screen_b.run();

    // If you’ll only ever have homogeneous collections, using generics and trait bounds is preferable because the definitions will be monomorphized at compile time to use the concrete types.
    // On the other hand, with the method using trait objects, one Screen instance can hold a Vec<T> that contains a &A as well as a &B.

    // This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is similar to the concept of duck typing in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck!
    // In the implementation of run on Screen, run doesn’t need to know what the concrete type of each component is.
    // It doesn’t check whether a component is an instance of a A or a B, it just calls the draw method on the component.
    // By specifying Box<dyn Draw> as the type of the values in the components vector, we’ve defined Screen to need values that we can call the draw method on.

    // The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway.
    // Rust won’t compile our code if the values don’t implement the traits that the trait objects need.

    // let screen = gui_dynamic::Screen {
    //     components: vec![&String::from("Hi")],
    // };
    // error[E0277]: the trait bound `String: Draw` is not satisfied
    //    --> src/lessons/oop.rs:195:26
    //     |
    // 195 |         components: vec![&String::from("Hi")],
    //     |                          ^^^^^^^^^^^^^^^^^^^ the trait `Draw` is not implemented for `String`
    //     |
    //     = help: the following other types implement trait `Draw`:
    //               A
    //               B
    //     = note: required for the cast from `String` to the object type `dyn Draw`

    //

    // The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
    // This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.
    // In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

    // When we use trait objects, Rust must use dynamic dispatch.
    // The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call.
    // Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.
    // This lookup incurs a runtime cost that doesn’t occur with static dispatch.
    // Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.
    // However, we did get extra flexibility in the code that we wrote and were able to support, so it’s a trade-off to consider.

    //

    // https://doc.rust-lang.org/book/ch17-03-oo-design-patterns.html

    // The state pattern is an object-oriented design pattern.
    // The crux of the pattern is that we define a set of states a value can have internally.
    // The states are represented by a set of state objects, and the value’s behavior changes based on its state.
    // We’re going to work through an example of a blog post struct that has a field to hold its state, which will be a state object from the set "draft", "review", or "published".

    // First, we’re going to implement the state pattern in a more traditional object-oriented way.
    mod blog_oop_way {
        pub struct Post {
            state: Option<Box<dyn State>>,
            content: String,
        }

        impl Post {
            pub fn new() -> Post {
                Post {
                    state: Some(Box::new(Draft {})),
                    content: String::new(),
                }
            }

            pub fn add_text(&mut self, text: &str) {
                if self.state.as_ref().unwrap().can_add_text() {
                    self.content.push_str(text);
                }
            }

            pub fn content(&self) -> &str {
                self.state.as_ref().unwrap().content(self)
            }

            pub fn request_review(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.request_review())
                }
            }

            pub fn approve(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.approve())
                }
            }

            pub fn reject(&mut self) {
                if let Some(s) = self.state.take() {
                    self.state = Some(s.reject())
                }
            }
        }

        // The logic related to the rules lives in the state objects rather than being scattered throughout Post.
        trait State {
            // Note that rather than having self, &self, or &mut self as the first parameter of the method, we have self: Box<Self>.
            // This syntax means the method is only valid when called on a Box holding the type.
            // This syntax takes ownership of Box<Self>, invalidating the old state so the state value of the Post can transform into a new state.
            fn request_review(self: Box<Self>) -> Box<dyn State>;
            fn approve(self: Box<Self>) -> Box<dyn State>;
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                ""
            }
            fn reject(self: Box<Self>) -> Box<dyn State>;
            fn can_add_text(&self) -> bool {
                false
            }
        }

        struct Draft {}

        impl State for Draft {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                Box::new(PendingReview {
                    got_first_approve: false,
                })
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn can_add_text(&self) -> bool {
                true
            }
        }

        struct PendingReview {
            got_first_approve: bool,
        }

        impl State for PendingReview {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(mut self: Box<Self>) -> Box<dyn State> {
                if self.got_first_approve {
                    Box::new(Published {})
                } else {
                    self.got_first_approve = true;
                    self
                }
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                Box::new(Draft {})
            }
        }

        struct Published {}

        impl State for Published {
            fn request_review(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn approve(self: Box<Self>) -> Box<dyn State> {
                self
            }
            fn content<'a>(&self, post: &'a Post) -> &'a str {
                &post.content
            }
            fn reject(self: Box<Self>) -> Box<dyn State> {
                self
            }
        }
    }

    let mut post = blog_oop_way::Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.reject();
    assert_eq!("", post.content());

    post.request_review();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.approve();
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());

    // By implementing the state pattern exactly as it’s defined for object-oriented languages, we’re not taking as full advantage of Rust’s strengths as we could.
    // Let’s look at some changes we can make to the blog crate that can make invalid states and transitions into compile time errors.

    //

    // We’ll show you how to rethink the state pattern to get a different set of trade-offs.
    // Rather than encapsulating the states and transitions completely so outside code has no knowledge of them, we’ll encode the states into different types.
    // Consequently, Rust’s type checking system will prevent attempts to use draft posts where only published posts are allowed by issuing a compiler error.

    mod blog_rust_way {
        pub struct Post {
            content: String,
        }

        pub struct DraftPost {
            content: String,
        }

        impl Post {
            pub fn new() -> DraftPost {
                DraftPost {
                    content: String::new(),
                }
            }

            pub fn content(&self) -> &str {
                &self.content
            }
        }

        impl DraftPost {
            pub fn add_text(&mut self, text: &str) {
                self.content.push_str(text);
            }

            pub fn request_review(self) -> PendingTwoReviewPost {
                PendingTwoReviewPost {
                    content: self.content,
                }
            }
        }

        pub struct PendingTwoReviewPost {
            content: String,
        }

        impl PendingTwoReviewPost {
            pub fn approve(self) -> PendingOneReviewPost {
                PendingOneReviewPost {
                    content: self.content,
                }
            }

            pub fn reject(self) -> DraftPost {
                DraftPost {
                    content: self.content,
                }
            }
        }

        pub struct PendingOneReviewPost {
            content: String,
        }

        impl PendingOneReviewPost {
            pub fn approve(self) -> Post {
                Post {
                    content: self.content,
                }
            }

            pub fn reject(self) -> PendingTwoReviewPost {
                PendingTwoReviewPost {
                    content: self.content,
                }
            }
        }
    }

    let mut post = blog_rust_way::Post::new();
    post.add_text("I ate a salad for lunch today");
    let post = post.request_review();
    let post = post.reject();
    let mut post = post.request_review();
    let post = post.approve();
    let post = post.reject();
    let post = post.approve();
    let post = post.approve();

    assert_eq!("I ate a salad for lunch today", post.content());

    // The changes we needed to make to main to reassign post mean that this implementation doesn’t quite follow the object-oriented state pattern anymore: the transformations between the states are no longer encapsulated entirely within the Post implementation.
    // However, our gain is that invalid states are now impossible because of the type system and the type checking that happens at compile time!
    // This ensures that certain bugs, such as display of the content of an unpublished post, will be discovered before they make it to production.

    // We’ve seen that even though Rust is capable of implementing object-oriented design patterns, other patterns, such as encoding state into the type system, are also available in Rust.
    // These patterns have different trade-offs.
    // Although you might be very familiar with object-oriented patterns, rethinking the problem to take advantage of Rust’s features can provide benefits, such as preventing some bugs at compile time.
    // Object-oriented patterns won’t always be the best solution in Rust due to certain features, like ownership, that object-oriented languages don’t have.
}
