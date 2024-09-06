fn main() {
    {
        // Using `rust-ini`.
        let mut conf = ini::Ini::new();
        conf.with_section(None::<String>).set("encoding", "utf-8");
        conf.with_section(Some("User"))
            .set("given_name", "Tommy")
            .set("unicode", "Raspberry树莓");
        conf.with_section(Some("Book")).set("name", "Rust cool");
        conf.write_to_file("conf1.ini").unwrap();
    }

    {
        // Using `configparser`.
        let mut config = configparser::ini::Ini::new();
        // config.set(None, "encoding", "utf-8".to_owned().into()); // Not possible.
        config.set("User", "given_name", "Tommy".to_owned().into());
        config.set("User", "unicode", "Raspberry树莓".to_owned().into());
        config.set("Book", "name", "Rust cool".to_owned().into());
        config.set("Book", "surname", None); // Generates invalid `ini` format.
        config.write("conf2.ini").unwrap();
    }

    {
        // Using `serde_ini`.
        #[derive(serde::Deserialize, serde::Serialize)]
        #[serde(rename_all = "PascalCase")]
        struct Config {
            encoding: &'static str,
            user: User,
            book: Book,
        }

        #[derive(serde::Deserialize, serde::Serialize)]
        struct User {
            given_name: String,
            unicode: String,
        }

        #[derive(serde::Deserialize, serde::Serialize)]
        struct Book {
            name: String,
        }

        let c = Config {
            encoding: "utf-8",
            user: User {
                given_name: "Tommy".to_owned(),
                unicode: "Raspberry树莓".to_owned(),
            },

            book: Book {
                name: "Rust cool".to_owned(),
            },
        };

        std::fs::write("conf3.ini", serde_ini::to_string(&c).unwrap()).unwrap();
    }
}
