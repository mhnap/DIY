// https://doc.rust-lang.org/book/ch03-04-comments.html
// https://doc.rust-lang.org/rust-by-example/hello/comment.html
// https://doc.rust-lang.org/rust-by-example/meta/doc.html
// https://doc.rust-lang.org/stable/rustdoc/
// https://doc.rust-lang.org/stable/reference/comments.html
// https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html
// https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#appendix-a-full-conventions-text

fn main() {
    // In Rust, the idiomatic comment style starts a comment with two slashes, and the comment continues until the end of the line.
    // For comments that extend beyond a single line, you’ll need to include // on each line, like these comments.

    // Comments can also be placed at the end of lines containing code:
    let lucky_number = 7; // I’m feeling lucky today

    // But you’ll more often see them used in this format, with the comment on a separate line above the code it’s annotating:
    // I’m feeling lucky today
    let lucky_number = 7;

    /*
     * This is another type of comment, a block comment. In general,
     * line comments are the recommended comment style. But block comments
     * are extremely useful for temporarily disabling chunks of code.
     * /* Block comments can be /* nested, */ */ so it takes only a few
     * keystrokes to comment out everything in this main() function.
     * /*/*/* Try it yourself! */*/*/
     */

    /*
    Note: The previous column of `*` was entirely for style. There's
    no actual need for it.
    */

    //

    mod intra_rustdoc_links {
        //! AST types representing various typed SQL expressions. Almost all types
        //! implement either [`Expression`] or [`AsExpression`].

        /// Represents a typed fragment of SQL. Apps should not need to implement this
        /// type directly, but it may be common to use this as type boundaries.
        /// Libraries should consider using [`infix_predicate!`] or
        /// [`postfix_predicate!`] instead of implementing this directly.
        pub trait Expression {
            type SqlType;
        }

        /// Describes how a type can be represented as an expression for a given type.
        /// These types couldn't just implement [`Expression`] directly, as many things
        /// can be used as an expression of multiple types. ([`String`] for example, can
        /// be used as either [`VarChar`] or [text type][`Text`]).
        /// Taken from [this link][Link].
        ///
        /// [`VarChar`]: super::VarChar
        /// [`Text`]: super::Text
        /// [Link]: https://rust-lang.github.io/rfcs/1946-intra-rustdoc-links.html
        pub trait AsExpression<T> {
            type Expression: Expression<SqlType = T>;
            fn as_expression(self) -> Self::Expression;
        }
    }
}

struct VarChar;
struct Text;
