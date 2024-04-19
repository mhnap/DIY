// https://www.lurklurk.org/effective-rust/use-types.html
// Item 1: Use the type system to express your data structures
// who called them programers and not type writer

fn main() {
    // The true power of Rust's enum feature comes from the fact that each variant can have data that comes along with it, making it into an algebraic data type (ADT).
    // This is less familiar to programmers of mainstream languages;
    // in C/C++ terms it's like a combination of an enum with a union – only type-safe.

    // This means that the invariants of the program's data structures can be encoded into Rust's type system;
    // states that don't comply with those invariants won't even compile.
    // A well-designed enum makes the creator's intent clear to humans as well as to the compiler:
    // pub enum SchedulerState {
    //     Inert,
    //     Pending(HashSet<Job>),
    //     Running(HashMap<CpuId, Vec<Job>>),
    // }

    // Just from the type definition, it's reasonable to guess that Jobs get queued up in the Pending state until the scheduler is fully active, at which point they're assigned to some per-CPU pool.
    // This highlights the central theme of this Item, which is to use Rust's type system to express the concepts that are associated with the design of your software.

    // A dead giveaway for when this is not happening is a comment that explains when some field or parameter is valid:
    struct DisplayProps {
        x: u32,
        y: u32,
        monochrome: bool,
        // `fg_colour` must be (0, 0, 0) if `monochrome` is true.
        fg_colour: RgbColour,
    }
    struct RgbColour(u8, u8, u8);

    // This is a prime candidate for replacement with an enum holding data:
    enum Colour {
        Monochrome,
        Foreground(RgbColour),
    }

    struct DisplayProperties {
        x: u32,
        y: u32,
        colour: Colour,
    }

    // This small example illustrates a key piece of advice: **make invalid states inexpressible in your types**.
    // Types that only support valid combinations of values mean that whole classes of error are rejected by the compiler, leading to smaller and safer code.
}
