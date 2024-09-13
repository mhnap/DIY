use std::collections::HashMap;

#[derive(Debug)]
struct Data {
    key: i32,
    val: i32,
}

fn main() {
    let vec = vec![Data { key: 1, val: 2 }, Data { key: 3, val: 4 }];

    // Compiler error without type annotation.
    // let map = vec.iter().map(|v| (v.key, v.val)).collect();
    // error[E0282]: type annotations needed
    //   --> experiments/src/bin/test.rs:14:9
    //    |
    // 14 |     let map = vec.iter().map(|v| (v.key, v.val)).collect();
    //    |         ^^^
    //    |
    // help: consider giving `map` an explicit type
    //    |
    // 14 |     let map: Vec<_> = vec.iter().map(|v| (v.key, v.val)).collect();
    //    |            ++++++++

    // Works with explicit type annotation.
    let _map: HashMap<_, _> = vec.iter().map(|v| (v.key, v.val)).collect();
    let _map = vec.iter().map(|v| (v.key, v.val)).collect::<HashMap<_, _>>();

    // Works without explicit type annotation when type is specified in the function signature.
    dbg_map(vec.iter().map(|v| (v.key, v.val)).collect());

    // Cannot infer type in this case.
    // dbg_into_map(vec.iter().map(|v| (v.key, v.val)).collect());
    // error[E0283]: type annotations needed
    //   --> experiments/src/bin/test.rs:36:53
    //    |
    // 36 |     dbg_into_map(vec.iter().map(|v| (v.key, v.val)).collect());
    //    |     ------------                                    ^^^^^^^ cannot infer type of the type parameter `B` declared on the method `collect`
    //    |     |
    //    |     required by a bound introduced by this call
    //    |
    //    = note: cannot satisfy `_: Into<HashMap<i32, i32>>`
    // note: required by a bound in `dbg_into_map`
    //   --> experiments/src/bin/test.rs:43:27
    //    |
    // 43 | fn dbg_into_map(map: impl Into<HashMap<i32, i32>>) {
    //    |                           ^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `dbg_into_map`
    // help: consider specifying the generic argument
    //    |
    // 36 |     dbg_into_map(vec.iter().map(|v| (v.key, v.val)).collect::<Vec<_>>());
    //    |                                                            ++++++++++

    // Works with explicit type annotation.
    dbg_into_map(vec.iter().map(|v| (v.key, v.val)).collect::<HashMap<_, _>>());
}

fn dbg_map(map: HashMap<i32, i32>) {
    dbg!(map);
}

fn dbg_into_map(map: impl Into<HashMap<i32, i32>>) {
    dbg!(map.into());
}
