// https://doc.rust-lang.org/error_codes/E0509.html

struct FancyNum {
    num: usize,
}

struct DropStruct {
    fancy: FancyNum,
}

impl Drop for DropStruct {
    fn drop(&mut self) {
        // Destruct DropStruct, possibly using FancyNum
    }
}

struct NonDropStruct {
    fancy: FancyNum,
}

fn main() {
    // Can move from non drop struct.
    let non_drop_struct = NonDropStruct {
        fancy: FancyNum { num: 5 },
    };

    let fancy_field = non_drop_struct.fancy;
    println!("Fancy: {}", fancy_field.num);

    // Cannot move from drop struct.
    let drop_struct = DropStruct {
        fancy: FancyNum { num: 5 },
    };
    // let fancy_field = drop_struct.fancy;
    // println!("Fancy: {}", fancy_field.num);
    //     error[E0509]: cannot move out of type `DropStruct`, which implements the `Drop` trait
    //     --> my/experiments/src/bin/move_from_struct_with_drop.rs:34:23
    //      |
    //   34 |     let fancy_field = drop_struct.fancy;
    //      |                       ^^^^^^^^^^^^^^^^^
    //      |                       |
    //      |                       cannot move out of here
    //      |                       move occurs because `drop_struct.fancy` has type `FancyNum`, which does not implement the `Copy` trait
    //      |
    //   help: consider borrowing here
    //      |
    //   34 |     let fancy_field = &drop_struct.fancy;
    //      |                       +

    // This error occurs when an attempt is made to move out of a value whose type implements the Drop trait.

    // Here, we tried to move a field out of a struct of type DropStruct which implements the Drop trait.
    // However, a struct cannot be dropped if one or more of its fields have been moved.

    // Structs implementing the Drop trait have an implicit destructor that gets called when they go out of scope.
    // This destructor may use the fields of the struct, so moving out of the struct could make it impossible to run the destructor.
    // Therefore, we must think of all values whose type implements the Drop trait as single units whose fields cannot be moved.

    // This error can be fixed by creating a reference to the fields of a struct, enum, or tuple using the ref keyword.

    let drop_struct = DropStruct {
        fancy: FancyNum { num: 5 },
    };
    let ref fancy_field = drop_struct.fancy;
    println!("Fancy: {}", fancy_field.num);
}
