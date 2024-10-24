mod generated {
    include!(concat!(env!("OUT_DIR"), "/person.rs"));
}

use crate::generated::Person;
use prost::Message;

fn main() {
    let person =
        Person { name: "Alice".to_string(), id: 123, email: Some("alice@example.com".to_string()) };

    // Serialize the person message to bytes
    let bytes = person.encode_to_vec();
    dbg!(bytes.len());

    // Deserialize the bytes back into a Person message
    let deserialized_person: Person = Person::decode(bytes.as_slice()).unwrap();

    dbg!(&deserialized_person.name);
    dbg!(&deserialized_person.id);
    dbg!(&deserialized_person.email);
}
