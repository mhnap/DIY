#[derive(Debug)]
pub struct Asparagus;

pub fn get_garden_name() -> &'static str {
    // Need to specify an absolute path here.
    // crate::garden::GARDEN_NAME
    // Or use super to specify parent.
    super::GARDEN_NAME
}

// Even when the struct is declared pub, each struct field should be declared pub separately.
#[derive(Debug)]
pub struct Potato {
    pub weight: u32,
    kind: String,
}

impl Potato {
    pub fn new(weight: u32) -> Self {
        Potato { weight, kind: "homemade".into() }
    }
}

// All enum variants are pub if the enum is pub.
#[derive(Debug)]
pub enum Herb {
    Basil,
    Oregano,
    Mint,
}
