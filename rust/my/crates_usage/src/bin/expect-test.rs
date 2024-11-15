fn main() {}

#[cfg(test)]
#[ignore]
mod tests {
    use expect_test::expect;

    #[test]
    fn test_addition() {
        let actual = 2 + 2;
        let expected = expect!["5"];
        expected.assert_eq(&actual.to_string())
    }

    #[test]
    fn test_struct() {
        #[derive(Debug)]
        struct Foo {
            value: i32,
        }

        let actual = Foo { value: 92 };
        let expected = expect![[r#"
            Foo {
                value: 91,
            }
        "#]];
        expected.assert_debug_eq(&actual);
    }
}
