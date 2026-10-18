#![allow(unused)]
#![allow(mismatched_lifetime_syntaxes)]

mod iterator;
mod parser;

fn main() {
    // iterator
    use iterator::Value;
    let value = Value::Array(vec![
        Value::Number(1),
        Value::Number(2),
        Value::Array(vec![Value::Number(3), Value::Number(4)]),
    ]);

    for elem in value {
        dbg!(elem);
    }

    // parser
    let json = "[false, 1000, 2, {\"1\": [{}]}]\n{\"one\": 1}";
    let mut iter = parser::JsonIter::new(json);
    let value = iter.next();
    dbg!(value);
    let value = iter.next();
    dbg!(value);
}
