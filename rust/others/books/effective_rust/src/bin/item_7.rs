// https://www.lurklurk.org/effective-rust/builders.html
// Item 7: Use builders for complex types

fn main() {
    // This Item describes the builder pattern, where complex data structures have an associated builder type that makes it easier for users to create instances of the data structure.

    // Rust insists that all fields in a struct must be filled in when a new instance of that struct is created.
    // This keeps the code safe by ensuring that there are never any uninitialized values but does lead to more verbose boilerplate code than is ideal.

    // For example, any optional fields have to be explicitly marked as absent with None:

    /// Phone number in E164 format.
    #[derive(Debug, Clone)]
    pub struct PhoneNumberE164(pub String);

    #[derive(Debug, Default)]
    pub struct Details {
        pub given_name: String,
        pub preferred_name: Option<String>,
        pub middle_name: Option<String>,
        pub family_name: String,
        pub mobile_phone: Option<PhoneNumberE164>,
    }

    // ...

    let dizzy = Details {
        given_name: "Dizzy".to_owned(),
        preferred_name: None,
        middle_name: None,
        family_name: "Mixer".to_owned(),
        mobile_phone: None,
    };

    // This boilerplate code is also brittle, in the sense that a future change that adds a new field to the struct requires an update to every place that builds the structure.

    // The boilerplate can be significantly reduced by implementing and using the Default trait, as described in Item 10:
    let dizzy = Details {
        given_name: "Dizzy".to_owned(),
        family_name: "Mixer".to_owned(),
        ..Default::default()
    };

    // Using Default also helps reduce the changes needed when a new field is added, provided that the new field is itself of a type that implements Default.

    // These ergonomics can be improved if you **implement the builder pattern for complex data structures**.

    // The simplest variant of the builder pattern is a separate struct that holds the information needed to construct the item.
    // For simplicity, the example will hold an instance of the item itself:
    pub struct DetailsBuilder(Details);

    impl DetailsBuilder {
        /// Start building a new [`Details`] object.
        pub fn new(given_name: &str, family_name: &str) -> Self {
            DetailsBuilder(Details {
                given_name: given_name.to_owned(),
                preferred_name: None,
                middle_name: None,
                family_name: family_name.to_owned(),
                mobile_phone: None,
            })
        }
    }

    // The builder type can then be equipped with helper methods that fill out the nascent item's fields.
    // Each such method consumes self but emits a new Self, allowing different construction methods to be chained:

    impl DetailsBuilder {
        /// Set the preferred name.
        pub fn preferred_name(mut self, preferred_name: &str) -> Self {
            self.0.preferred_name = Some(preferred_name.to_owned());
            self
        }

        /// Set the middle name.
        pub fn middle_name(mut self, middle_name: &str) -> Self {
            self.0.middle_name = Some(middle_name.to_owned());
            self
        }
    }

    // The final method to be invoked for the builder consumes the builder and emits the built item:

    impl DetailsBuilder {
        /// Consume the builder object and return a fully built [`Details`]
        /// object.
        pub fn build(self) -> Details {
            self.0
        }
    }

    // Overall, this allows clients of the builder to have a more ergonomic building experience:
    let also_bob = DetailsBuilder::new("Robert", "Builder")
        .middle_name("the")
        .preferred_name("Bob")
        .build();
}
