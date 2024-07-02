use typed_builder::TypedBuilder;

fn main() {
    #[derive(Debug, PartialEq)]
    pub struct PhoneNumberE164(pub String);

    #[derive(Debug, TypedBuilder)]
    pub struct Details {
        pub given_name: String,
        #[builder(default)]
        pub preferred_name: Option<String>,
        #[builder(default, setter(strip_option))]
        pub middle_name: Option<String>,
        pub family_name: String,
        #[builder(default)]
        pub mobile_phone: Option<PhoneNumberE164>,
    }

    // let details = Details::builder().build();
    //     error[E0061]: this method takes 1 argument but 0 arguments were supplied
    //     --> my/crates_usage/src/bin/typed-builder.rs:20:38
    //      |
    //   20 |     let details = Details::builder().build();
    //      |                                      ^^^^^-- an argument of type `DetailsBuilder_Error_Missing_required_field_given_name` is missing
    //      |

    // let details = Details::builder().given_name("Mike".into()).build();
    //     error[E0061]: this method takes 1 argument but 0 arguments were supplied
    //     --> my/crates_usage/src/bin/typed-builder.rs:35:64
    //      |
    //   35 |     let details = Details::builder().given_name("Mike".into()).build();
    //      |                                                                ^^^^^-- an argument of type `DetailsBuilder_Error_Missing_required_field_family_name` is missing
    //      |

    let details = Details::builder()
        .given_name("Mike".into())
        // .given_name("Mike".into())
        //         error[E0277]: the trait bound `DetailsBuilder_Error_Repeated_field_given_name: From<&str>` is not satisfied
        //   --> my/crates_usage/src/bin/typed-builder.rs:37:28
        //    |
        // 37 |         .given_name("Mike".into())
        //    |                            ^^^^ the trait `From<&str>` is not implemented for `DetailsBuilder_Error_Repeated_field_given_name`, which is required by `&str: Into<_>`
        .family_name("Hnap".into())
        .build();
    assert_eq!(details.given_name, "Mike");
    assert_eq!(details.family_name, "Hnap");

    let details = Details::builder()
        .given_name("Mike".into())
        .preferred_name(Some("Mykhailo".into()))
        .middle_name("Романович".into())
        .family_name("Hnap".into())
        .build();
    assert_eq!(details.given_name, "Mike");
    assert_eq!(details.preferred_name, Some("Mykhailo".into()));
    assert_eq!(details.middle_name, Some("Романович".into()));
    assert_eq!(details.family_name, "Hnap");
    assert_eq!(details.mobile_phone, None);

    #[derive(Debug, TypedBuilder)]
    pub struct Details2 {
        #[builder(setter(into))]
        pub given_name: String,
        #[builder(default, setter(into))]
        pub preferred_name: Option<String>,
        #[builder(default, setter(strip_option))]
        pub middle_name: Option<String>,
        #[builder(default = String::from("Hnap"))]
        pub family_name: String,
        #[builder(setter(strip_bool))]
        pub has_mobile_phone: bool,
    }

    let details = Details2::builder()
        .given_name("Mike")
        .preferred_name("Mykhailo".to_owned())
        .middle_name("Романович".into())
        .has_mobile_phone()
        .build();
    assert_eq!(details.given_name, "Mike");
    assert_eq!(details.preferred_name, Some("Mykhailo".into()));
    assert_eq!(details.middle_name, Some("Романович".into()));
    assert_eq!(details.family_name, "Hnap");
    assert_eq!(details.has_mobile_phone, true);
}
