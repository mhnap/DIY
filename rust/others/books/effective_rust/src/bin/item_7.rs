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

    // The all-consuming nature of this style of builder leads to a couple of wrinkles.
    // The first is that separating out stages of the build process can't be done on its own:
    // let builder = DetailsBuilder::new("Robert", "Builder");
    // if true {
    //     builder.preferred_name("Bob");
    // }
    // let bob = builder.build();
    //     error[E0382]: use of moved value: `builder`
    //     --> others/books/effective_rust/src/bin/item_7.rs:104:15
    //      |
    //  100 |     let builder = DetailsBuilder::new("Robert", "Builder");
    //      |         ------- move occurs because `builder` has type `DetailsBuilder`, which does not implement the `Copy` trait
    //  101 |     if true {
    //  102 |         builder.preferred_name("Bob");
    //      |                 --------------------- `builder` moved due to this method call
    //  103 |     }
    //  104 |     let bob = builder.build();
    //      |               ^^^^^^^ value used here after move
    //      |

    // This can be worked around by assigning the consumed builder back to the same variable:
    let mut builder = DetailsBuilder::new("Robert", "Builder");
    if true {
        builder = builder.preferred_name("Bob");
    }
    let bob = builder.build();

    // The other downside to the all-consuming nature of this builder is that only one item can be built; trying to create multiple instances by repeatedly calling build() on the same builder falls foul of the compiler, as you'd expect:
    // let smithy = DetailsBuilder::new("Agent", "Smith");
    // let clones = vec![smithy.build(), smithy.build(), smithy.build()];
    //     error[E0382]: use of moved value: `smithy`
    //     --> others/books/effective_rust/src/bin/item_7.rs:133:39
    //      |
    //  129 |     let smithy = DetailsBuilder::new(
    //      |         ------ move occurs because `smithy` has type `DetailsBuilder`, which does not implement the `Copy` trait
    //  ...
    //  133 |     let clones = vec![smithy.build(), smithy.build(), smithy.build()];
    //      |                              -------  ^^^^^^ value used here after move
    //      |                              |
    //      |                              `smithy` moved due to this method call
    //      |
    //  note: `DetailsBuilder::build` takes ownership of the receiver `self`, which moves `smithy`
    //     --> others/books/effective_rust/src/bin/item_7.rs:87:22
    //      |
    //  87  |         pub fn build(self) -> Details {
    //      |                      ^^^^

    //  error[E0382]: use of moved value: `smithy`
    //     --> others/books/effective_rust/src/bin/item_7.rs:133:55
    //      |
    //  129 |     let smithy = DetailsBuilder::new(
    //      |         ------ move occurs because `smithy` has type `DetailsBuilder`, which does not implement the `Copy` trait
    //  ...
    //  133 |     let clones = vec![smithy.build(), smithy.build(), smithy.build()];
    //      |                                              -------  ^^^^^^ value used here after move
    //      |                                              |
    //      |                                              `smithy` moved due to this method call

    //

    // An alternative approach is for the builder's methods to take a &mut self and emit a &mut Self:

    pub struct DetailsBuilder2(Details);

    impl DetailsBuilder2 {
        /// Start building a new [`Details`] object.
        pub fn new(given_name: &str, family_name: &str) -> Self {
            DetailsBuilder2(Details {
                given_name: given_name.to_owned(),
                preferred_name: None,
                middle_name: None,
                family_name: family_name.to_owned(),
                mobile_phone: None,
            })
        }

        /// Set the preferred name.
        pub fn preferred_name(&mut self, preferred_name: &str) -> &mut Self {
            self.0.preferred_name = Some(preferred_name.to_owned());
            self
        }

        /// Set the middle name.
        pub fn middle_name(&mut self, middle_name: &str) -> &mut Self {
            self.0.middle_name = Some(middle_name.to_owned());
            self
        }

        /// Consume the builder object and return a fully built [`Details`]
        /// object.
        pub fn build(self) -> Details {
            self.0
        }
    }

    // This removes the need for self-assignment in separate build stages:

    let mut builder = DetailsBuilder2::new("Robert", "Builder");
    if true {
        builder.preferred_name("Bob"); // no `builder = ...`
    }
    let bob = builder.build();

    // However, this version makes it impossible to chain the construction of the builder together with invocation of its setter methods:
    // let builder = DetailsBuilder2::new("Robert", "Builder")
    //     .middle_name("the")
    //     .preferred_name("Bob");
    // let bob = builder.build();
    //     error[E0716]: temporary value dropped while borrowed
    //    --> others/books/effective_rust/src/bin/item_7.rs:202:19
    //     |
    // 202 |     let builder = DetailsBuilder2::new("Robert", "Builder")
    //     |                   ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
    // 203 |         .middle_name("the")
    // 204 |         .preferred_name("Bob");
    //     |                               - temporary value is freed at the end of this statement
    // 205 |     let bob = builder.build();
    //     |               ------- borrow later used here
    //     |

    // As indicated by the compiler error, you can work around this by letting the builder item have a name:

    let mut builder = DetailsBuilder2::new("Robert", "Builder");
    builder.middle_name("the").preferred_name("Bob");
    let bob = builder.build();

    // With any style of builder pattern, the boilerplate code is now confined to one place—the builder—rather than being needed at every place that uses the underlying type.
}
