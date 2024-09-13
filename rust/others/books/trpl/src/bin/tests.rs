// https://doc.rust-lang.org/book/ch11-00-testing.html

// Correctness in our programs is the extent to which our code does what we intend it to do.
// Rust is designed with a high degree of concern about the correctness of programs, but correctness is complex and not easy to prove.
// Rust’s type system shoulders a huge part of this burden, but the type system cannot catch everything.
// As such, Rust includes support for writing automated software tests.

//

// https://doc.rust-lang.org/book/ch11-01-writing-tests.html

// Tests are Rust functions that verify that the non-test code is functioning in the expected manner.
// The bodies of test functions typically perform these three actions:
// 1. Set up any needed data or state.
// 2. Run the code you want to test.
// 3. Assert the results are what you expect.

#[cfg(test)]
mod tests_v1 {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

// Note the #[test] annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
// We might also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we always need to indicate which functions are tests.

// The example function body uses the assert_eq! macro to assert that result, which contains the result of adding 2 and 2, equals 4.
// This assertion serves as an example of the format for a typical test.

// Tests fail when something in the test function panics.
// Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.

#[cfg(test)]
mod tests_v2 {
    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
}

// Instead of ok, the line test tests::another shows FAILED.
// Two new sections appear between the individual results and the summary: the first displays the detailed reason for each test failure.
// In this case, we get the details that another failed because it panicked.
// The next section lists just the names of all the failing tests, which is useful when there are lots of tests and lots of detailed failing test output.
// We can use the name of a failing test to run just that test to more easily debug it.
// The summary line displays at the end: overall, our test result is FAILED.
// We had one test pass and one test fail.

//

// The assert! macro, provided by the standard library, is useful when you want to ensure that some condition in a test evaluates to true.
// We give the assert! macro an argument that evaluates to a Boolean.
// If the value is true, nothing happens and the test passes.
// If the value is false, the assert! macro calls panic! to cause the test to fail.
// Using the assert! macro helps us check that our code is functioning in the way we intend.

#[derive(Debug, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests_v3 {
    // Because the tests module is an inner module, we need to bring the code under test in the outer module into the scope of the inner module.
    // We use a glob here so anything we define in the outer module is available to this tests module.
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { width: 8, height: 7 };
        let smaller = Rectangle { width: 5, height: 1 };

        // Because the correct result of the can_hold function in this case is false, we need to negate that result before we pass it to the assert! macro.
        assert!(!smaller.can_hold(&larger));
    }
}

//

// A common way to verify functionality is to test for equality between the result of the code under test and the value you expect the code to return.
// You could do this using the assert! macro and passing it an expression using the == operator.
// However, this is such a common test that the standard library provides a pair of macros—assert_eq! and assert_ne!—to perform this test more conveniently.
// These macros compare two arguments for equality or inequality, respectively.
// They’ll also print the two values if the assertion fails, which makes it easier to see why the test failed; conversely, the assert! macro only indicates that it got a false value for the == expression, without printing the values that led to the false value.

pub fn add_two(a: i32) -> i32 {
    a + 3
}

#[cfg(test)]
mod tests_v4 {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
}

// Note that in some languages and test frameworks, the parameters to equality assertion functions are called expected and actual, and the order in which we specify the arguments matters.
// However, in Rust, they’re called left and right, and the order in which we specify the value we expect and the value the code produces doesn’t matter.
// We could write the assertion in this test as assert_eq!(add_two(2), 4), which would result in the same failure message that displays assertion failed: `(left == right)`.

// The assert_ne! macro will pass if the two values we give it are not equal and fail if they’re equal.
// This macro is most useful for cases when we’re not sure what a value will be, but we know what the value definitely shouldn’t be.
// For example, if we’re testing a function that is guaranteed to change its input in some way, but the way in which the input is changed depends on the day of the week that we run our tests, the best thing to assert might be that the output of the function is not equal to the input.

// Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively.
// When the assertions fail, these macros print their arguments using debug formatting, which means the values being compared must implement the PartialEq and Debug traits.
// All primitive types and most of the standard library types implement these traits.
// For structs and enums that you define yourself, you’ll need to implement PartialEq to assert equality of those types.
// You’ll also need to implement Debug to print the values when the assertion fails.
// Because both traits are derivable traits, this is usually as straightforward as adding the #[derive(PartialEq, Debug)] annotation to your struct or enum definition.

#[cfg(test)]
mod tests_v5 {
    use super::*;

    #[test]
    fn is_different() {
        let larger = Rectangle { width: 8, height: 7 };

        let smaller = Rectangle { width: 5, height: 1 };

        assert_ne!(larger, smaller);
        // We would get such error is Rectangle would not derive PartialEq trait.
        // error[E0369]: binary operation `==` cannot be applied to type `Rectangle`
        //    --> src/lessons/tests.rs:167:9
        //     |
        // 167 |         assert_ne!(larger, smaller);
        //     |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^
        //     |         |
        //     |         Rectangle
        //     |         Rectangle
        //     |
        // note: an implementation of `PartialEq<_>` might be missing for `Rectangle`
        //    --> src/lessons/tests.rs:67:1
        //     |
        // 67  | struct Rectangle {
        //     | ^^^^^^^^^^^^^^^^ must implement `PartialEq<_>`
        //     = note: this error originates in the macro `assert_ne` (in Nightly builds, run with -Z macro-backtrace for more info)
        // help: consider annotating `Rectangle` with `#[derive(PartialEq)]`
        //     |
        // 67  + #[derive(PartialEq)]
        // 68  | struct Rectangle {
        //     |
    }
}

//

// You can also add a custom message to be printed with the failure message as optional arguments to the assert!, assert_eq!, and assert_ne! macros.
// Any arguments specified after the required arguments are passed along to the format! macro, so you can pass a format string that contains {} placeholders and values to go in those placeholders.
// Custom messages are useful for documenting what an assertion means; when a test fails, you’ll have a better idea of what the problem is with the code.

pub fn greeting(name: &str) -> String {
    format!("Hello!")
}

#[cfg(test)]
mod tests_v6 {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), "Greeting did not contain name, value was `{}`", result);
    }
}

// We can see the value we actually got in the test output, which would help us debug what happened instead of what we were expecting to happen.

//

// In addition to checking return values, it’s important to check that our code handles error conditions as we expect.
// We do this by adding the attribute should_panic to our test function.
// The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new_correct(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn new_incorrect(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests_v7 {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100_correct() {
        Guess::new_correct(200);
    }

    #[test]
    #[should_panic]
    fn greater_than_100_incorrect() {
        Guess::new_incorrect(200);
    }
}

// Tests that use should_panic can be imprecise.
// A should_panic test would pass even if the test panics for a different reason from the one we were expecting.
// To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute.
// The test harness will make sure that the failure message contains the provided text.

#[cfg(test)]
mod tests_v8 {
    use super::*;

    #[test]
    #[should_panic(expected = "cannot parse integer")]
    fn greater_than_100() {
        Guess::new_correct(200);
    }
}

//

// We can also write tests that use Result<T, E>!

#[cfg(test)]
mod tests_v9 {
    #[test]
    fn it_works() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }
}

// Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
// You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
// To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value.
// Instead, use assert!(value.is_err()).

//

// https://doc.rust-lang.org/book/ch11-02-running-tests.html

// Just as cargo run compiles your code and then runs the resulting binary, cargo test compiles your code in test mode and runs the resulting test binary.
// The default behavior of the binary produced by cargo test is to run all the tests in parallel and capture output generated during test runs, preventing the output from being displayed and making it easier to read the output related to the test results.
// You can, however, specify command line options to change this default behavior.
// More can be read in book.

// https://doc.rust-lang.org/book/ch11-03-test-organization.html

// The Rust community thinks about tests in terms of two main categories: unit tests and integration tests.
// Unit tests are small and more focused, testing one module in isolation at a time, and can test private interfaces.
// Integration tests are entirely external to your library and use your code in the same way any other external code would, using only the public interface and potentially exercising multiple modules per test.

//

// There’s debate within the testing community about whether or not private functions should be tested directly, and other languages make it difficult or impossible to test private functions.
// Regardless of which testing ideology you adhere to, Rust’s privacy rules do allow you to test private functions.

pub fn add_three(a: i32) -> i32 {
    internal_adder(a, 3)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests_v10 {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

// Note that the internal_adder function is not marked as pub.

//

// In Rust, integration tests are entirely external to your library.
// They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API.
// Their purpose is to test whether many parts of your library work together correctly.
// Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well.
// To create integration tests, you first need a tests directory.
// More can be read in book.

fn main() {}
