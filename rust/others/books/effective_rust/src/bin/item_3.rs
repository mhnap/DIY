// https://www.lurklurk.org/effective-rust/transform.html
// Item 3: Avoid matching Option and Result

fn main() {
    // The standard library provides a wide variety of these transformation methods to make this possible, as shown in the following map.
    // https://docs.google.com/drawings/d/1EOPs0YTONo_FygWbuJGPfikO9Myt5HwtiFUHRuE1JVM/preview

    // To sum up:
    // Get used to the transformations of Option and Result, and prefer Result to Option.
    // Use .as_ref() as needed when transformations involve references.
    // Use them in preference to explicit match operations.
    // In particular, use them to transform result types into a form where the ? operator applies.
}
