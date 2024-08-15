use std::collections::{BTreeMap, HashMap};

fn main() {
    let vec = vec![
        ("key-1", "val-1"),
        ("key-2", "val-2"),
        ("key-1", "val-11"),
        ("key-3", "val-3"),
        ("key-1", "val-111"),
    ];
    assert_eq!(vec.len(), 5);

    // The same key will have the last value.
    let map: HashMap<_, _> = vec.clone().into_iter().map(|tuple| tuple).collect();
    assert_eq!(map.len(), 3);
    assert_eq!(map["key-1"], "val-111");
    assert_eq!(map["key-2"], "val-2");
    assert_eq!(map["key-3"], "val-3");

    // The same key will have the last value.
    let map: BTreeMap<_, _> = vec.into_iter().map(|tuple| tuple).collect();
    assert_eq!(map.len(), 3);
    assert_eq!(map["key-1"], "val-111");
    assert_eq!(map["key-2"], "val-2");
    assert_eq!(map["key-3"], "val-3");
}
