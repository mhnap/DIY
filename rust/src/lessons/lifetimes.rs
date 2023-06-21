// https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

fn main() {
    // Lifetimes are another kind of generic that we’ve already been using.
    // Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need them to be.

    // Every reference in Rust has a lifetime, which is the scope for which that reference is valid.
    // Most of the time, lifetimes are implicit and inferred, just like most of the time, types are inferred.
    // We only must annotate types when multiple types are possible.
    // In a similar way, we must annotate lifetimes when the lifetimes of references could be related in a few different ways.
    // Rust requires us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

    {
        // The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than the data it’s intended to reference.
        let a = 42;
        let mut r = &a; // 'a lifetime
        {
            let x = 5;   // 'b lifetime
            // r = &x;
            // error[E0597]: `x` does not live long enough
            //   --> src/lessons/lifetimes.rs:20:17
            //    |
            // 18 |             let x = 5;
            //    |                 - binding `x` declared here
            // 19 |             // An attempt to use a reference whose value has gone out of scope.
            // 20 |             r = &x;
            //    |                 ^^ borrowed value does not live long enough
            // ...
            // 33 |         }
            //    |         - `x` dropped here while still borrowed
            // 34 |         println!("r: {}", r);
            //    |                           - borrow later used here
        }
        println!("r: {}", r);

        // The Rust compiler has a `borrow checker` that compares scopes to determine whether all borrows are valid.
        // At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of 'a but that it refers to memory with a lifetime of 'b.
        // The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
    }

    {
        // In some situations, when Rust compiler cannot deduce lifetime rules, we need to help it manually with lifetime annotation.
        // fn longest(x: &str, y: &str) -> &str {
        //     if x.len() > y.len() {
        //         x
        //     } else {
        //         y
        //     }
        // }
        // error[E0106]: missing lifetime specifier
        //   --> src/lessons/lifetimes.rs:43:41
        //    |
        // 43 |         fn longest(x: &str, y: &str) -> &str {
        //    |                       ----     ----     ^ expected named lifetime parameter
        //    |
        //    = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
        // help: consider introducing a named lifetime parameter
        //    |
        // 43 |         fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        //    |                   ++++     ++          ++          ++

        // The help text reveals that the return type needs a generic lifetime parameter on it because Rust can’t tell whether the reference being returned refers to x or y.
        // Actually, we don’t know either, because the if block in the body of this function returns a reference to x and the else block returns a reference to y!

        // When we’re defining this function, we don’t know the concrete values that will be passed into this function, so we don’t know whether the if case or the else case will execute.
        // We also don’t know the concrete lifetimes of the references that will be passed in, so we can’t look at the scopes as we did to determine whether the reference we return will always be valid.
        // The borrow checker can’t determine this either, because it doesn’t know how the lifetimes of x and y relate to the lifetime of the return value.
        // To fix this error, we’ll add generic lifetime parameters that define the relationship between the references so the borrow checker can perform its analysis.

        // Lifetime annotations don’t change how long any of the references live.
        // Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes.
        // Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

        // To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.
        // We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid.
        // This is the relationship between lifetimes of the parameters and the return value.

        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(&string1, string2);
        println!("The longest string is {}", result);

        // When annotating lifetimes in functions, the annotations go in the function signature, not in the function body.
        // The lifetime annotations become part of the contract of the function, much like the types in the signature.
        // Having function signatures contain the lifetime contract means the analysis the Rust compiler does can be simpler.
        // If there’s a problem with the way a function is annotated or the way it is called, the compiler errors can point to the part of our code and the constraints more precisely.
        // If, instead, the Rust compiler made more inferences about what we intended the relationships of the lifetimes to be, the compiler might only be able to point to a use of our code many steps away from the cause of the problem.

        // Lifetimes can be different, the only rule is they must have the same smaller part.
        let string1 = String::from("long string is long");
        {
            let string2 = String::from("xyz");
            let result = longest(string1.as_str(), string2.as_str());
            println!("The longest string is {}", result);
        }

        // Next, let’s try an example that shows that the lifetime of the reference in result must be the smaller lifetime of the two arguments.
        let string1 = String::from("long string is long");
        let mut result: &str = &string1;
        {
            let string2 = String::from("xyz");
            // result = longest(string1.as_str(), string2.as_str());
            // error[E0597]: `string2` does not live long enough
            //    --> src/lessons/lifetimes.rs:110:48
            //     |
            // 109 |             let string2 = String::from("xyz");
            //     |                 ------- binding `string2` declared here
            // 110 |             result = longest(string1.as_str(), string2.as_str());
            //     |                                                ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
            // 111 |         }
            //     |         - `string2` dropped here while still borrowed
            // 112 |         println!("The longest string is {}", result);
            //     |                                              ------ borrow later used here
        }
        println!("The longest string is {}", result);

        // We’ve told Rust that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the references passed in.
        // Therefore, the borrow checker disallows the code as possibly having an invalid reference.
    }

    {
        // We can omit lifetime for `y` here because the lifetime of `y` does not have any relationship with the lifetime of `x` or the return value.
        fn longest<'a>(x: &'a str, y: &str) -> &'a str {
            x
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";
        let result = longest(&string1, string2);
        println!("The longest string is {}", result);

        // But lifetime for `y` would be required if we return `y` instead and don't specify `y`'s lifetime.
        // fn longest_v2<'a>(x: &'a str, y: &str) -> &'a str {
        //     y
        // }
        // error[E0621]: explicit lifetime required in the type of `y`
        //    --> src/lessons/lifetimes.rs:141:13
        //     |
        // 140 |         fn longest_v2<'a>(x: &'a str, y: &str) -> &'a str {
        //     |                                          ---- help: add explicit lifetime `'a` to the type of `y`: `&'a str`
        // 141 |             y
        //     |             ^ lifetime `'a` required
    }

    {
        // When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
        // If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
        // However, this would be a dangling reference because the value will go out of scope at the end of the function.
        // fn longest<'a>(x: &str, y: &str) -> &'a str {
        //     let result = String::from("really long string");
        //     result.as_str()
        // }
        // error[E0515]: cannot return reference to local variable `result`
        //    --> src/lessons/lifetimes.rs:158:13
        //     |
        // 158 |             result.as_str()
        //     |             ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function
        // In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.
    }

    //

    {
        //  We can define structs to hold references, but in that case we would need to add a lifetime annotation on every reference in the struct’s definition.
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        dbg!(i.part);

        // This annotation means an instance of `ImportantExcerpt` can’t outlive the reference it holds in its `part` field.
    }

    {
        // In certain cases, Rust compiler can accept references without lifetimes.
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();

            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }

            &s[..]
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        dbg!(first_word(&novel));

        // The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.
        // These aren’t rules for programmers to follow; they’re a set of particular cases that the compiler will consider, and if your code fits these cases, you don’t need to write the lifetimes explicitly.

        // Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.
        // The compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations.
        // The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes.
        // If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error.
        // These rules apply to `fn` definitions as well as `impl` blocks.
        // 1. Compiler assigns separate lifetime parameter to each parameter that’s a reference.
        // 2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters.
        // 3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self` because this is a method, the lifetime of `self` is assigned to all output lifetime parameters.
    }

    {
        // Lifetimes also can be annotated for struct methods.
        struct ImportantExcerpt<'a> {
            part: &'a str,
        }

        // If out struct has lifetimes, we need to annotate `impl` too.
        // impl ImportantExcerpt {
        //     fn level(&self) -> i32 {
        //         3
        //     }
        // }
        // error[E0726]: implicit elided lifetime not allowed here
        //    --> src/lessons/lifetimes.rs:223:14
        //     |
        // 223 |         impl ImportantExcerpt {
        //     |              ^^^^^^^^^^^^^^^^ expected lifetime parameter
        //     |
        // help: indicate the anonymous lifetime
        //     |
        // 223 |         impl ImportantExcerpt<'_> {
        //     |                              ++++

        // Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.
        // The lifetime parameter declaration after impl and its use after the type name are required, but we’re not required to annotate the lifetime of the reference to `self` because of the first elision rule.
        impl<'a> ImportantExcerpt<'a> {
            fn level(&self) -> i32 {
                3
            }
        }

        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let i = ImportantExcerpt {
            part: first_sentence,
        };
        dbg!(i.level());

        // Here is an example where the third lifetime elision rule applies.
        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_part(&self, announcement: &str) -> &str {
                println!("Attention please: {}", announcement);
                self.part
            }
        }

        // Need to annotate at least `announcement` to return `announcement`.
        impl<'a> ImportantExcerpt<'a> {
            fn announce_and_return_announcement(&self, announcement: &'a str) -> &str {
                println!("Attention please: {}", announcement);
                announcement
            }
        }

        dbg!(i.announce_and_return_part("Hello!"));
        dbg!(i.announce_and_return_announcement("Hello!"));
    }

    //

    {
        // One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.
        // All string literals have the 'static lifetime.
        let s: &'static str = "I have a static lifetime.";
        dbg!(s);
        // The text of this string is stored directly in the program’s binary, which is always available.
        // Therefore, the lifetime of all string literals is 'static.
    }

    {
        // Let’s briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!
        use std::fmt::Display;

        fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
            where T: Display
        {
            println!("Announcement! {}", ann);
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }

        let string1 = String::from("abcd");
        let string2 = "xyz";

        let result = longest_with_an_announcement(
            string1.as_str(),
            string2,
            "Today is someone's birthday!",
        );
        println!("The longest string is {}", result);
    }
}
