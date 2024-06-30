// https://www.lurklurk.org/effective-rust/newtype.html
// Item 6: Embrace the newtype pattern

fn main() {
    // Item 1 described tuple structs, where the fields of a struct have no names and are instead referred to by number (self.0).
    // This Item focuses on tuple structs that have a single entry of some existing type, thus creating a new type that can hold exactly the same range of values as the enclosed type.
    // This pattern is sufficiently pervasive in Rust that it deserves its own Item and has its own name: the newtype pattern.

    // The simplest use of the newtype pattern is to indicate additional semantics for a type, over and above its normal behavior.
    // To illustrate this, imagine a project that's going to send a satellite to Mars.
    // It's a big project, so different groups have built different parts of the project.
    // One group has handled the code for the rocket engines:

    #[derive(Copy, Clone)]
    struct Direction;
    let direction = Direction;

    {
        /// Fire the thrusters. Returns generated impulse in pound-force seconds.
        pub fn thruster_impulse(direction: Direction) -> f64 {
            // ...
            return 42.0;
        }

        // while a different group handles the inertial guidance system:

        /// Update trajectory model for impulse, provided in Newton seconds.
        pub fn update_trajectory(force: f64) {
            // ...
        }

        // Eventually these different parts need to be joined together:

        let thruster_force: f64 = thruster_impulse(direction);
        let new_direction = update_trajectory(thruster_force);

        // Ruh-roh.
    }

    // https://en.wikipedia.org/wiki/Mars_Climate_Orbiter#Cause_of_failure
    // The primary cause of this discrepancy was that one piece of ground software supplied by Lockheed Martin produced results in a United States customary unit, contrary to its Software Interface Specification (SIS), while a second system, supplied by NASA, expected those results to be in SI units, in accordance with the SIS.
    // Specifically, software that calculated the total impulse produced by thruster firings produced results in pound-force seconds.
    // The trajectory calculation software then used these results – expected to be in newton-seconds (incorrect by a factor of 4.45) – to update the predicted position of the spacecraft.

    // This is the point where the newtype pattern helps:

    /// Units for force.
    pub struct PoundForceSeconds(pub f64);

    /// Fire the thrusters. Returns generated impulse.
    pub fn thruster_impulse(direction: Direction) -> PoundForceSeconds {
        // ...
        return PoundForceSeconds(42.0);
    }

    /// Units for force.
    pub struct NewtonSeconds(pub f64);

    /// Update trajectory model for impulse.
    pub fn update_trajectory(force: NewtonSeconds) {
        // ...
    }

    // let thruster_force: PoundForceSeconds = thruster_impulse(direction);
    // let new_direction = update_trajectory(thruster_force);
    //     error[E0308]: mismatched types
    //     --> others/books/effective_rust/src/bin/item_6.rs:64:43
    //      |
    //   64 |     let new_direction = update_trajectory(thruster_force);
    //      |                         ----------------- ^^^^^^^^^^^^^^ expected `NewtonSeconds`, found `PoundForceSeconds`
    //      |                         |
    //      |                         arguments to this function are incorrect
    //      |

    // As described in Item 5, adding an implementation of the standard From trait:
    impl From<PoundForceSeconds> for NewtonSeconds {
        fn from(val: PoundForceSeconds) -> NewtonSeconds {
            NewtonSeconds(4.448222 * val.0)
        }
    }
    // allows the necessary unit—and type—conversion to be performed with .into():
    let thruster_force: PoundForceSeconds = thruster_impulse(direction);
    let new_direction = update_trajectory(thruster_force.into());

    // The same pattern of using a newtype to mark additional "unit" semantics for a type can also help to make purely Boolean arguments less ambiguous.
    // Revisiting the example from Item 1, using newtypes makes the meaning of arguments clear:
    struct DoubleSided(pub bool);
    struct ColorOutput(pub bool);
    fn print_page(sides: DoubleSided, color: ColorOutput) {
        // ...
    }
    print_page(DoubleSided(true), ColorOutput(false));

    // If size efficiency or binary compatibility is a concern, then the #[repr(transparent)] attribute ensures that a newtype has the same representation in memory as the inner type.

    //

    // The other common, but more subtle, scenario that requires the newtype pattern revolves around Rust's orphan rule.

    // This is a sufficiently common problem for serde that it includes a mechanism to help:
    // https://serde.rs/remote-derive.html
}
