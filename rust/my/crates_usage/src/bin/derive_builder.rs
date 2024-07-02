use derive_builder::Builder;

fn main() {
    #[derive(Debug, Clone)]
    pub struct PhoneNumberE164(pub String);

    #[derive(Debug, Default, Builder)]
    pub struct Details {
        pub given_name: String,
        pub preferred_name: Option<String>,
        pub middle_name: Option<String>,
        pub family_name: String,
        pub mobile_phone: Option<PhoneNumberE164>,
    }

    let mut details = DetailsBuilder::create_empty();

    // details.build().unwrap();
    // thread 'main' panicked at my/crates_usage/src/bin/derive_builder.rs:19:21:
    // called `Result::unwrap()` on an `Err` value: UninitializedField("given_name")

    // WTF?
    assert_eq!(details.given_name, None);

    details.given_name("Mike".into());
    details.family_name("Hnap".into());

    // WTF?
    // let details = details.build().unwrap();
    // thread 'main' panicked at my/crates_usage/src/bin/derive_builder.rs:29:35:
    // called `Result::unwrap()` on an `Err` value: UninitializedField("preferred_name")

    // Need to add clumsy attributes to gain normal behavior..
    #[derive(Debug, Default, Builder)]
    #[builder(setter(strip_option), default)]
    pub struct Details2 {
        pub given_name: String,
        pub preferred_name: Option<String>,
        pub middle_name: Option<String>,
        pub family_name: String,
        pub mobile_phone: Option<PhoneNumberE164>,
    }

    let mut details = Details2Builder::create_empty();
    details.given_name("Mike".into());
    details.family_name("Hnap".into());
    let details = details.build().unwrap();
    dbg!(details);

    // Oh no..
    let mut details = Details2Builder::create_empty();
    details.given_name("Mike".into());
    // details.family_name("Hnap".into());
    let details = details.build().unwrap();
    dbg!(details);
}
