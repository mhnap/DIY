// https://doc.rust-lang.org/book/ch19-06-macros.html

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

// Notice that we’ve split the code into the hello_macro_derive function, which is responsible for parsing the TokenStream, and the impl_hello_macro function, which is responsible for transforming the syntax tree: this makes writing a procedural macro more convenient.
// The code in the outer function (hello_macro_derive in this case) will be the same for almost every procedural macro crate you see or create.
// The code you specify in the body of the inner function (impl_hello_macro in this case) will be different depending on your procedural macro’s purpose.

// The hello_macro_derive function will be called when a user of our library specifies #[derive(HelloMacro)] on a type.
// This is possible because we’ve annotated the hello_macro_derive function here with proc_macro_derive and specified the name HelloMacro, which matches our trait name; this is the convention most procedural macros follow.

// You might have noticed that we’re calling unwrap to cause the hello_macro_derive function to panic if the call to the syn::parse function fails here.
// It’s necessary for our procedural macro to panic on errors because proc_macro_derive functions must return TokenStream rather than Result to conform to the procedural macro API.
// We’ve simplified this example by using unwrap; in production code, you should provide more specific error messages about what went wrong by using panic! or expect.

// Now that we have the code to turn the annotated Rust code from a TokenStream into a DeriveInput instance, let’s generate the code that implements the HelloMacro trait on the annotated type.
fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                println!("Hello, Mike's Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

// The quote! macro lets us define the Rust code that we want to return.
// The compiler expects something different to the direct result of the quote! macro’s execution, so we need to convert it to a TokenStream.
// We do this by calling the into method, which consumes this intermediate representation and returns a value of the required TokenStream type.

// The quote! macro also provides some very cool templating mechanics: we can enter #name, and quote! will replace it with the value in the variable name.
// You can even do some repetition similar to the way regular macros work.

// The stringify! macro used here is built into Rust.
// It takes a Rust expression, such as 1 + 2, and at compile time turns the expression into a string literal, such as "1 + 2".
// This is different than format! or println!, macros which evaluate the expression and then turn the result into a String.
// There is a possibility that the #name input might be an expression to print literally, so we use stringify!.
// Using stringify! also saves an allocation by converting #name to a string literal at compile time.
