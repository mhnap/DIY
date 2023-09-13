// https://doc.rust-lang.org/book/ch19-06-macros.html

// The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:
// - Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
// - Attribute-like macros that define custom attributes usable on any item
// - Function-like macros that look like function calls but operate on the tokens specified as their argument

fn main() {
    // Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.
    // Macros expand to produce more code than the code you’ve written manually.

    // Metaprogramming is useful for reducing the amount of code you have to write and maintain, which is also one of the roles of functions.
    // However, macros have some additional powers that functions don’t.

    // A function signature must declare the number and type of parameters the function has.
    // Macros, on the other hand, can take a variable number of parameters.
    // Also, macros are expanded before the compiler interprets the meaning of the code, so a macro can, for example, implement a trait on a given type.
    // A function can’t, because it gets called at runtime and a trait needs to be implemented at compile time.

    // The downside to implementing a macro instead of a function is that macro definitions are more complex than function definitions because you’re writing Rust code that writes Rust code.
    // Due to this indirection, macro definitions are generally more difficult to read, understand, and maintain than function definitions.

    // Another important difference between macros and functions is that you must define macros or bring them into scope before you call them in a file, as opposed to functions you can define anywhere and call anywhere.

    //

    // The most widely used form of macros in Rust is the declarative macro.
    // These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros.”
    // At their core, declarative macros allow you to write something similar to a Rust match expression.
    // "match" expressions are control structures that take an expression, compare the resulting value of the expression to patterns, and then run the code associated with the matching pattern.
    // Macros also compare a value to patterns that are associated with particular code: in this situation, the value is the literal Rust source code passed to the macro; the patterns are compared with the structure of that source code; and the code associated with each pattern, when matched, replaces the code passed to the macro.
    // This all happens during compilation.

    // To define a macro, you use the macro_rules! construct.
    #[macro_export]
    macro_rules! my_vec {
        ( $( $x:expr ),* ) => {
            {
                let mut temp_vec = Vec::new();
                $(
                    temp_vec.push($x);
                )*
                temp_vec
            }
        };
    }

    let my_vec = my_vec![1, 2, 3];
    dbg!(&my_vec);

    // The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope.
    // Without this annotation, the macro can’t be brought into scope.

    // We then start the macro definition with macro_rules! and the name of the macro we’re defining without the exclamation mark.
    // The name, in this case `my_vec`, is followed by curly brackets denoting the body of the macro definition.

    // The structure in the `my_vec!` body is similar to the structure of a match expression.
    // Here we have one arm with the pattern ( $( $x:expr ),* ), followed by => and the block of code associated with this pattern.
    // If the pattern matches, the associated block of code will be emitted.
    // Given that this is the only pattern in this macro, there is only one valid way to match; any other pattern will result in an error.
    // More complex macros will have more than one arm.

    // Valid pattern syntax in macro definitions is different than the regular pattern syntax because macro patterns are matched against Rust code structure rather than values.
    // First, we use a set of parentheses to encompass the whole pattern.
    // We use a dollar sign ($) to declare a variable in the macro system that will contain the Rust code matching the pattern.
    // The dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable.
    // Next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code.
    // Within $() is $x:expr, which matches any Rust expression and gives the expression the name $x.
    // The comma following $() indicates that a literal comma separator character could optionally appear after the code that matches the code in $().
    // The * specifies that the pattern matches zero or more of whatever precedes the *.

    //

    // The second form of macros is the procedural macro, which acts more like a function (and is a type of procedure).
    // Procedural macros accept some code as an input, operate on that code, and produce some code as an output rather than matching against patterns and replacing the code with other code as declarative macros do.
    // The three kinds of procedural macros are custom derive, attribute-like, and function-like, and all work in a similar fashion.

    // When creating procedural macros, the definitions must reside in their own crate with a special crate type.
    // use proc_macro;
    // #[some_attribute]
    // pub fn some_name(input: TokenStream) -> TokenStream {
    // }

    // The function that defines a procedural macro takes a TokenStream as an input and produces a TokenStream as an output.
    // The TokenStream type is defined by the proc_macro crate that is included with Rust and represents a sequence of tokens.
    // This is the core of the macro: the source code that the macro is operating on makes up the input TokenStream, and the code the macro produces is the output TokenStream.
    // The function also has an attribute attached to it that specifies which kind of procedural macro we’re creating.
    // We can have multiple kinds of procedural macros in the same crate.

    //

    // Let’s create a crate named hello_macro that defines a trait named HelloMacro with one associated function named hello_macro.
    // We have defined a trait and its function in the hello_macro crate.
    // At this point, our crate user could implement the trait to achieve the desired functionality, like so:
    {
        use hello_macro::HelloMacro;

        struct Pancakes;

        impl HelloMacro for Pancakes {
            fn hello_macro() {
                println!("Hello, Mike's Macro! My name is Pancakes!");
            }
        }

        Pancakes::hello_macro();

        // However, they would need to write the implementation block for each type they wanted to use with hello_macro; we want to spare them from having to do this work.
        // Additionally, we can’t yet provide the hello_macro function with default implementation that will print the name of the type the trait is implemented on: Rust doesn’t have reflection capabilities, so it can’t look up the type’s name at runtime.
        // We need a macro to generate code at compile time.
    }

    // The next step is to define the procedural macro.
    // At the time of this writing, procedural macros need to be in their own crate.
    // Eventually, this restriction might be lifted.
    // The convention for structuring crates and macro crates is as follows: for a crate named foo, a custom derive procedural macro crate is called foo_derive.

    // Our two crates are tightly related, so we create the procedural macro crate within the directory of our hello_macro crate.
    // If we change the trait definition in hello_macro, we’ll have to change the implementation of the procedural macro in hello_macro_derive as well.
    // The two crates will need to be published separately, and programmers using these crates will need to add both as dependencies and bring them both into scope.
    // We could instead have the hello_macro crate use hello_macro_derive as a dependency and re-export the procedural macro code.
    // However, the way we’ve structured the project makes it possible for programmers to use hello_macro even if they don’t want the derive functionality.

    // We need to declare the hello_macro_derive crate as a procedural macro crate.
    // We’ll also need functionality from the syn and quote crates, as you’ll see in a moment, so we need to add them as dependencies.

    // We’ve introduced three new crates: proc_macro, syn, and quote.
    // The proc_macro crate comes with Rust, so we didn’t need to add that to the dependencies in Cargo.toml.
    // The proc_macro crate is the compiler’s API that allows us to read and manipulate Rust code from our code.
    // The syn crate parses Rust code from a string into a data structure that we can perform operations on.
    // The quote crate turns syn data structures back into Rust code.
    // These crates make it much simpler to parse any sort of Rust code we might want to handle: writing a full parser for Rust code is no simple task.

    {
        use hello_macro::HelloMacro;
        use hello_macro_derive::HelloMacro;

        #[derive(HelloMacro)]
        struct Pancakes;

        Pancakes::hello_macro();
    }

    //

    // Attribute-like macros are similar to custom derive macros, but instead of generating code for the derive attribute, they allow you to create new attributes.
    // They’re also more flexible: derive only works for structs and enums; attributes can be applied to other items as well, such as functions.

    // Here’s an example of using an attribute-like macro: say you have an attribute named route that annotates functions when using a web application framework:
    // #[route(GET, "/")]
    // fn index() {

    // This #[route] attribute would be defined by the framework as a procedural macro.
    // The signature of the macro definition function would look like this:
    // #[proc_macro_attribute]
    // pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {

    // Here, we have two parameters of type TokenStream.
    // The first is for the contents of the attribute: the GET, "/" part.
    // The second is the body of the item the attribute is attached to: in this case, fn index() {} and the rest of the function’s body.
    // Other than that, attribute-like macros work the same way as custom derive macros: you create a crate with the proc-macro crate type and implement a function that generates the code you want!

    //

    // Function-like macros define macros that look like function calls.
    // Function-like macros take a TokenStream parameter and their definition manipulates that TokenStream using Rust code as the other two types of procedural macros do.
    // An example of a function-like macro is an sql! macro that might be called like so:
    // let sql = sql!(SELECT * FROM posts WHERE id=1);

    // This macro would parse the SQL statement inside it and check that it’s syntactically correct, which is much more complex processing than a macro_rules! macro can do.
    // The sql! macro would be defined like this:
    // #[proc_macro]
    // pub fn sql(input: TokenStream) -> TokenStream {

    // This definition is similar to the custom derive macro’s signature: we receive the tokens that are inside the parentheses and return the code we wanted to generate.
}
