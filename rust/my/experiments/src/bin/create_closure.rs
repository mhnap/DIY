fn main() {
    // Some data that should be captured inside the closure.
    let name = String::from("Mykhailo");

    // The closure that does some useful stuff.
    let with_surname = |surname: &str| format!("I'm {name} {surname}");

    // Can run closure with saved data.
    dbg!(with_surname("Hnap"));
    dbg!(with_surname("Anonym"));

    // Function for creating closures without duplication.
    fn get_with_surname(name: &str) -> impl Fn(&str) -> String + use<'_> {
        move |surname: &str| format!("I'm {name} {surname}")
    }

    // Can create and run closure with saved data.
    dbg!(get_with_surname(&name)("Hnap"));
    dbg!(get_with_surname(&name)("Anonym"));
}
