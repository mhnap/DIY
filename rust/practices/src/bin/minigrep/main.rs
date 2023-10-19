// https://doc.rust-lang.org/book/ch12-00-an-io-project.html

use std::env;
use std::process;

use minigrep::Config;

// Should be defined as lib, but for simplicity, use module.
mod minigrep;

fn main() {
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
