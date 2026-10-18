use std::collections::HashMap;

/*

JSON iterator

functional requirements:
    - stream? file? socket? object in memory?
    - receive the whole json payload as object (vec<char>, String, &str, vec<u8> ?)
    - output is ?

non-functional requirements:
    - latency or throughput?
    - ndjson?
    - do we need to think about space saving?

structure:
    - object {}
    - array []
    - key "key"
    - value true/123/"value"

example:
{
    "key1": 1,
    "key2": false,
    "key3": [
        {
            "key31": 1,
        },
        {
            "key32": 2
        }
    ]
}

[1, 2, {"1": [{}]}] = Array(vec[Number(1), Number(2), Object(Field("1", Array(vec[Object])))])

algorithm:
    - get text part till /n
    - wait for next
    - while waiting - parse received text into json Value and return/store it

when to parse:
    - on new? (we would parse everything and store parsed Values)
        pros: next is O(1) runtime
        cons: new is O(n) runtime; O(n) space stored till first next(); harder to adapt to streaming
        notes: on next() - we would need to delete Value, so space is reclaimed back
               vec would be bad, as we would need to shift on pop,
               but we can reverse it at the end, so pop_back would be O(1),
               not sure though if Vec realloc it on pop back,
               for sure not on every pop, but at least on some, probably not

    - on next?
        pros: new is O(1)
        cons: next is O(m)

    - start async (on different thread) on next?
        prefetch? k to fetch?

capabilities:
    - parse text to json
    - scheduler (producer/consumer via spsc queue): producer reads till \n and sends, consumer parse


we can firstly start with ended stream

*/

#[derive(Debug, PartialEq)]
pub enum Value {
    Object(Vec<Field>),
    Array(Vec<Value>),
    Primitive(Primitive),
}

type Field = (String, Value);

#[derive(Debug, PartialEq)]
pub enum Primitive {
    String(String),
    Number(i64),
    Bool(bool),
    //...
}

enum ValueType {
    Object,
    Array,
    Primitive(PrimitiveType),
}

enum PrimitiveType {
    String,
    Number,
    Bool,
}

pub struct JsonIter<'a> {
    input: Option<&'a str>,
}

const DELIMITER: char = '\n';

impl<'a> JsonIter<'a> {
    pub fn new(s: &'a str) -> Self {
        JsonIter { input: Some(s) }
    }
}

impl<'a> Iterator for JsonIter<'a> {
    type Item = Value;

    fn next(&mut self) -> Option<Self::Item> {
        let input = self.input?;

        let (value, next) = match input.split_once(DELIMITER) {
            Some((value, next)) => (value, Some(next)),
            None => (input, None), // last part
        };

        self.input = next;

        let (value, rest) = JsonParser::parse_value(value);

        if let Some(rest) = rest
            && !rest.is_empty()
        {
            panic!("rest is not empty");
        }

        Some(value)
    }
}

struct JsonParser;

type Rest<'a> = (Value, Option<&'a str>);

impl JsonParser {
    const COMMA: char = ',';
    const QUOTES: char = '\"';

    fn parse_value(value: &str) -> Rest {
        let value = value.trim_start();

        // Detect value type first
        let parse_map: HashMap<char, ValueType> = HashMap::from([
            ('{', ValueType::Object),
            ('[', ValueType::Array),
            (Self::QUOTES, ValueType::Primitive(PrimitiveType::String)),
            ('0', ValueType::Primitive(PrimitiveType::Number)),
            ('1', ValueType::Primitive(PrimitiveType::Number)),
            ('2', ValueType::Primitive(PrimitiveType::Number)),
            ('3', ValueType::Primitive(PrimitiveType::Number)),
            ('4', ValueType::Primitive(PrimitiveType::Number)),
            ('5', ValueType::Primitive(PrimitiveType::Number)),
            ('6', ValueType::Primitive(PrimitiveType::Number)),
            ('7', ValueType::Primitive(PrimitiveType::Number)),
            ('8', ValueType::Primitive(PrimitiveType::Number)),
            ('9', ValueType::Primitive(PrimitiveType::Number)),
            ('-', ValueType::Primitive(PrimitiveType::Number)),
            ('f', ValueType::Primitive(PrimitiveType::Bool)),
            ('t', ValueType::Primitive(PrimitiveType::Bool)),
        ]); // TODO: move hash map to lazy static or const (or match or table)

        let first = value.chars().next().unwrap();
        let Some(value_type) = parse_map.get(&first) else {
            panic!("unknown value type"); // TODO: move all panics to errors
        };

        let rest = match value_type {
            ValueType::Object => Self::parse_object(value),
            ValueType::Array => Self::parse_array(value),
            ValueType::Primitive(primitive_type) => match primitive_type {
                PrimitiveType::String => {
                    let (s, rest) = Self::parse_string(value);
                    (Value::Primitive(Primitive::String(s)), Some(rest))
                }
                PrimitiveType::Number => Self::parse_number(value),
                PrimitiveType::Bool => Self::parse_bool(value),
            },
        };

        rest
    }

    fn parse_object(object: &str) -> Rest {
        let mut input = Self::skip_one(object); // skip '{'
        let mut fields: Vec<Field> = vec![];

        // Check if empty
        if let Some(rest) = input.strip_prefix('}') {
            return (Value::Object(vec![]), Some(rest));
        }

        loop {
            let (field, rest) = Self::parse_field(input);
            fields.push(field);

            let rest = rest.expect("object field must be followed by ',' or '}'");
            let mut chars = rest.chars();
            match chars.next() {
                Some(',') => {
                    input = chars.as_str(); // next char after comma
                }
                Some('}') => {
                    return (Value::Object(fields), Some(chars.as_str())); // next char after end bracket
                }
                other => panic!("unexpected character after object field: {:?}", other),
            }
        }
    }

    fn parse_array(array: &str) -> Rest {
        let mut input = Self::skip_one(array); // skip '['
        let mut array: Vec<Value> = vec![];

        // Check if empty
        if let Some(rest) = input.strip_prefix(']') {
            return (Value::Array(vec![]), Some(rest));
        }

        loop {
            let (value, rest) = Self::parse_value(input);
            array.push(value);

            let rest = rest.expect("array element must be followed by ',' or ']'");
            let mut chars = rest.chars();
            match chars.next() {
                Some(',') => {
                    input = chars.as_str(); // next char after comma
                }
                Some(']') => {
                    return (Value::Array(array), Some(chars.as_str())); // next char after end bracket
                }
                other => panic!("unexpected character after array element: {:?}", other),
            }
        }
    }

    fn parse_field(field: &str) -> (Field, Option<&str>) {
        let (key, rest) = Self::parse_key(field);
        let (value, rest) = Self::parse_value(rest);
        let field: Field = (key, value);

        (field, rest)
    }

    fn parse_key(field: &str) -> (String, &str) {
        let field = field.trim_start();
        let (s, rest) = Self::parse_string(field);
        let Some(rest) = rest.strip_prefix(':') else {
            panic!("not a colon");
        };

        (s, rest)
    }

    fn parse_string(string: &str) -> (String, &str) {
        // TODO: move to non-owned string impl
        let Some(next) = string.strip_prefix(Self::QUOTES) else {
            panic!("not a string");
        };

        let mut result = String::new();
        let mut chars = next.chars();

        loop {
            match chars.next() {
                Some('\\') => match chars.next() {
                    Some('"') => result.push('"'),
                    Some('\\') => result.push('\\'),
                    Some('/') => result.push('/'),
                    Some('n') => result.push('\n'),
                    Some('t') => result.push('\t'),
                    Some('r') => result.push('\r'),
                    Some('b') => result.push('\u{8}'),
                    Some('f') => result.push('\u{c}'),
                    Some('u') => {
                        let hex: String = (0..4)
                            .map(|_| chars.next().expect("incomplete unicode escape"))
                            .collect();
                        let code = u32::from_str_radix(&hex, 16).expect("invalid unicode escape");
                        result.push(char::from_u32(code).expect("invalid unicode codepoint"));
                    }
                    other => panic!("invalid escape sequence: {:?}", other),
                },
                Some('"') => return (result, chars.as_str()), // real end
                Some(c) => result.push(c),
                None => panic!("no quotes ending"),
            }
        }
    }

    fn parse_number(number: &str) -> Rest {
        // Find first non-digit char
        let end = number
            .find(|c: char| !c.is_ascii_digit() && c != '-')
            .unwrap_or(number.len());

        let (num_str, rest) = number.split_at(end);
        let value = num_str.parse::<i64>().expect("invalid number");

        (Value::Primitive(Primitive::Number(value)), Some(rest))
    }

    fn parse_bool(bool: &str) -> Rest {
        if let Some(next) = bool.strip_prefix("true") {
            return (Value::Primitive(Primitive::Bool(true)), Some(next));
        } else if let Some(next) = bool.strip_prefix("false") {
            return (Value::Primitive(Primitive::Bool(false)), Some(next));
        } else {
            panic!("not a bool");
        }
    }

    fn skip_one(s: &str) -> &str {
        let mut s = s.chars();
        s.next(); // skip
        s.as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::{JsonIter, Primitive, Value};

    trait IntoIteratorMy<'a> {
        fn iter(self) -> impl Iterator<Item = Value>;
    }

    impl<'a> IntoIteratorMy<'a> for &'static str {
        fn iter(self) -> impl Iterator<Item = Value> {
            JsonIter::new(self)
        }
    }

    #[test]
    fn arrays() {
        let mut iter = "[123, [false, []], \"str\"]".iter();
        let value: Value = iter.next().unwrap();
        let expected = Value::Array(vec![
            Value::Primitive(Primitive::Number(123)),
            Value::Array(vec![
                Value::Primitive(Primitive::Bool(false)),
                Value::Array(vec![]),
            ]),
            Value::Primitive(Primitive::String(String::from("str"))),
        ]);
        assert_eq!(value, expected);
    }

    #[test]
    fn objects() {
        let mut iter = "{\"o\\\"n:e\": 1, \"two\": {\"three\": {}}}".iter();
        let value: Value = iter.next().unwrap();
        let expected = Value::Object(vec![
            (
                String::from("o\"n:e"), // TODO: move escaping to separate test
                Value::Primitive(Primitive::Number(1)),
            ),
            (
                String::from("two"),
                Value::Object(vec![(String::from("three"), Value::Object(vec![]))]),
            ),
        ]);
        assert_eq!(value, expected);
    }
}
