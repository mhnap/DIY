#[non_exhaustive]
pub struct Config {
    pub width: u16,
    pub height: u16,
}

pub fn create_config() -> Config {
    // Can create non-exhaustive struct using struct expression in own crate.
    Config { width: 640, height: 480 }
}

#[non_exhaustive]
pub enum Error {
    Message(String),
    Other,
}

#[non_exhaustive]
pub struct Foo;
