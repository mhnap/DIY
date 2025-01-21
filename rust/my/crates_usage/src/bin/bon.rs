use bon::Builder;

fn main() {
    #[derive(Debug, PartialEq)]
    pub struct PhoneNumberE164(pub String);

    #[derive(Debug, Builder)]
    pub struct Details {
        pub given_name: String,
        pub preferred_name: Option<String>,
        pub middle_name: Option<String>,
        pub family_name: String,
        pub mobile_phone: Option<PhoneNumberE164>,
    }

    // let details = Details::builder().build();
    // error[E0277]: the member `Unset<given_name>` was not set, but this method requires it to be set
    //   --> my/crates_usage/src/bin/bon.rs:16:38
    //    |
    // 16 |     let details = Details::builder().build();
    //    |                                      ^^^^^ the member `Unset<given_name>` was not set, but this method requires it to be set
    //    |
    //    = help: the trait `IsSet` is not implemented for `Unset<given_name>`, which is required by `details_builder::Empty: IsComplete`
    //    = help: the trait `IsSet` is implemented for `Set<Name>`

    // let details = Details::builder().given_name("Mike".into()).build();
    // error[E0277]: the member `Unset<family_name>` was not set, but this method requires it to be set
    //   --> my/crates_usage/src/bin/bon.rs:26:64
    //    |
    // 26 |     let details = Details::builder().given_name("Mike".into()).build();
    //    |                                                                ^^^^^ the member `Unset<family_name>` was not set, but this method requires it to be set
    //    |
    //    = help: the trait `IsSet` is not implemented for `Unset<family_name>`, which is required by `SetGivenName: IsComplete`
    //    = help: the trait `IsSet` is implemented for `Set<Name>`

    let details = Details::builder()
        .given_name("Mike".into())
        // .given_name("Mike".into())
        // error[E0277]: the member `Set<given_name>` was already set, but this method requires it to be unset
        //   --> my/crates_usage/src/bin/bon.rs:38:10
        //    |
        // 38 |         .given_name("Mike".into())
        //    |          ^^^^^^^^^^ the member `Set<given_name>` was already set, but this method requires it to be unset
        //    |
        //    = help: the trait `IsUnset` is not implemented for `Set<given_name>`
        //    = help: the trait `IsUnset` is implemented for `Unset<Name>`
        .family_name("Hnap".into())
        .build();
    assert_eq!(details.given_name, "Mike");
    assert_eq!(details.family_name, "Hnap");

    let details = Details::builder()
        .given_name("Mike".into())
        .maybe_preferred_name(Some("Mykhailo".into()))
        .middle_name("Романович".into())
        .family_name("Hnap".into())
        .build();
    assert_eq!(details.given_name, "Mike");
    assert_eq!(details.preferred_name, Some("Mykhailo".into()));
    assert_eq!(details.middle_name, Some("Романович".into()));
    assert_eq!(details.family_name, "Hnap");
    assert_eq!(details.mobile_phone, None);

    #[derive(Debug, Builder)]
    pub struct Details2 {
        #[builder(into)]
        pub given_name: String,
        #[builder(into)]
        pub preferred_name: Option<String>,
        pub middle_name: Option<String>,
        #[builder(default = "Hnap".into())]
        pub family_name: String,
        #[builder(with = || true)]
        pub has_mobile_phone: bool,
    }

    let details = Details2::builder()
        .given_name("Mike")
        .preferred_name("Mykhailo")
        .middle_name("Романович".into())
        .has_mobile_phone()
        .build();
    assert_eq!(details.given_name, "Mike");
    assert_eq!(details.preferred_name, Some("Mykhailo".into()));
    assert_eq!(details.middle_name, Some("Романович".into()));
    assert_eq!(details.family_name, "Hnap");
    assert_eq!(details.has_mobile_phone, true);
}
