use serde::Serialize;

fn main() {}

#[cfg(test)]
mod tests {
    use crate::AssertJson;
    use expect_test::expect;
    use serde::Serialize;

    #[test]
    fn test_addition() {
        let actual = 2 + 2;
        let expected = expect!["4"];
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
                value: 92,
            }
        "#]];
        expected.assert_debug_eq(&actual);
    }

    #[test]
    fn test_json() {
        #[derive(Debug, Serialize)]
        struct Foo {
            value: i32,
        }

        let actual = Foo { value: 92 };
        let expected = expect![[r#"
            {
              "value": 92
            }"#]];
        expected.assert_json_eq(&actual);
    }
}

pub trait AssertJson {
    #[track_caller]
    fn assert_json_eq<T>(&self, actual: &T)
    where
        T: ?Sized + Serialize;
}

impl AssertJson for expect_test::Expect {
    fn assert_json_eq<T>(&self, actual: &T)
    where
        T: ?Sized + Serialize,
    {
        let actual = serde_json::to_string_pretty(&actual).unwrap();
        self.assert_eq(&actual)
    }
}

impl AssertJson for expect_test::ExpectFile {
    fn assert_json_eq<T>(&self, actual: &T)
    where
        T: ?Sized + Serialize,
    {
        let actual = serde_json::to_string_pretty(&actual).unwrap();
        self.assert_eq(&actual)
    }
}
