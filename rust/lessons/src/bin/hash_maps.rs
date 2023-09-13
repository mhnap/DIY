// https://doc.rust-lang.org/book/ch08-03-hash-maps.html
// https://doc.rust-lang.org/std/collections/hash_map/struct.HashMap.html

use std::collections::HashMap;

fn main() {
    // The type HashMap<K, V> stores a mapping of keys of type K to values of type V using a hashing function, which determines how it places these keys and values into memory.

    // Creating hash map.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    dbg!(&scores);

    // Get value.
    let score = scores.get(&String::from("Blue")).unwrap();
    dbg!(&score);

    // Iteration over hash map.
    for (key, value) in &scores {
        dbg!(&key, &value);
    }
    // Note, the order is arbitrary.

    //

    // Ownership is moved into hash map.
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    dbg!(&map);
    // dbg!(&field_name, field_value);
    // error[E0382]: borrow of moved value: `field_name`
    //   --> src/lessons/collections/hash_maps.rs:30:10
    //    |
    // 25 |     let field_name = String::from("Favorite color");
    //    |         ---------- move occurs because `field_name` has type `String`, which does not implement the `Copy` trait
    // ...
    // 28 |     map.insert(field_name, field_value);
    //    |                ---------- value moved here
    // 29 |     dbg!(&map);
    // 30 |     dbg!(&field_name, field_value);
    //    |          ^^^^^^^^^^^ value borrowed here after move
    //    |
    // help: consider cloning the value if the performance cost is acceptable
    //    |
    // 28 |     map.insert(field_name.clone(), field_value);
    //    |                          ++++++++

    //

    // Replacing a value stored with a particular key.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    dbg!(&scores);

    // `insert` method return `None` if the map did not have the key present.
    // If the map did have the key present, the value is updated, and the old value is returned.
    let old_value = scores.insert(String::from("Blue"), 10);
    dbg!(&old_value);
    let old_value = scores.insert(String::from("NewBlue"), 10);
    dbg!(&old_value);

    //

    // Using the entry method to only insert if the key does not already have a value.
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    let entry = scores.entry(String::from("Yellow"));
    dbg!(&entry);
    entry.or_insert(50);

    let entry = scores.entry(String::from("Blue"));
    dbg!(&entry);
    // Returns a mutable reference to the value in the entry.
    let value = entry.or_insert(50);
    dbg!(&value);
    *value /= 2;

    // Another alternative to entry API would be such manual logic.
    let key = String::from("Blue");
    let value = scores.get(&key);
    if value.is_none() {
        scores.insert(key, 20);
    }

    dbg!(&scores);

    //

    // Counting occurrences of words using a hash map that stores words and counts.
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    dbg!(&map);

    //

    // By default, HashMap uses a hashing function called SipHash that can provide resistance to Denial of Service (DoS) attacks involving hash tables.
    // This is not the fastest hashing algorithm available, but the trade-off for better security that comes with the drop in performance is worth it.
    // If you profile your code and find that the default hash function is too slow for your purposes, you can switch to another function by specifying a different hasher.
}
