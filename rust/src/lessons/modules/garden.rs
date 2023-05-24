// When we bring a name into scope with the use keyword, the name available in the new scope is private.
// To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.
// This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
pub use vegetables::Herb;

pub mod vegetables;

const GARDEN_NAME: &str = "Mike's garden";
