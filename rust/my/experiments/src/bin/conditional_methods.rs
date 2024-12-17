// https://doc.rust-lang.org/book/ch10-02-traits.html#using-trait-bounds-to-conditionally-implement-methods
// https://users.rust-lang.org/t/conditional-implementation-on-trait-existence/42489

fn main() {
    let empty_snap = EmptySnap;
    let real_snap = RealSnap;

    let snapshot_pair = SnapshotPair { current: empty_snap, previous: real_snap };
    assert!(snapshot_pair.current_is_empty());
    assert!(snapshot_pair.prev_is_real());
    // assert!(snapshot_pair.current_is_real()); // does not exist
    // assert!(snapshot_pair.prev_is_empty()); // does not exist
    assert_eq!(snapshot_pair.current(), "empty");

    let snapshot_pair = SnapshotPair { current: real_snap, previous: empty_snap };
    // assert!(snapshot_pair.current_is_empty()); // does not exist
    // assert!(snapshot_pair.prev_is_real()); // does not exist
    assert!(snapshot_pair.current_is_real());
    assert!(snapshot_pair.prev_is_empty());
    assert_eq!(snapshot_pair.current(), "real");

    let snapshot_pair = SnapshotPair { current: real_snap, previous: real_snap };
    // assert!(snapshot_pair.current_is_empty()); // does not exist
    assert!(snapshot_pair.prev_is_real());
    assert!(snapshot_pair.current_is_real());
    // assert!(snapshot_pair.prev_is_empty()); // does not exist
    assert_eq!(snapshot_pair.current(), "real");

    let snapshot_pair = SnapshotPair { current: empty_snap, previous: empty_snap };
    assert!(snapshot_pair.current_is_empty());
    // assert!(snapshot_pair.prev_is_real()); // does not exist
    // assert!(snapshot_pair.current_is_real()); // does not exist
    assert!(snapshot_pair.prev_is_empty());
    assert_eq!(snapshot_pair.current(), "empty");
}

trait Snapshot {}

#[derive(Copy, Clone)]
struct EmptySnap;

impl Snapshot for EmptySnap {}

#[derive(Copy, Clone)]
struct RealSnap;

impl Snapshot for RealSnap {}

struct SnapshotPair<Curr: Snapshot, Prev: Snapshot> {
    #[expect(dead_code)]
    current: Curr,
    #[expect(dead_code)]
    previous: Prev,
}

impl<Prev: Snapshot> SnapshotPair<EmptySnap, Prev> {
    fn current_is_empty(&self) -> bool {
        true
    }
}

impl<Curr: Snapshot> SnapshotPair<Curr, EmptySnap> {
    fn prev_is_empty(&self) -> bool {
        true
    }
}

impl<Prev: Snapshot> SnapshotPair<RealSnap, Prev> {
    fn current_is_real(&self) -> bool {
        true
    }
}

impl<Curr: Snapshot> SnapshotPair<Curr, RealSnap> {
    fn prev_is_real(&self) -> bool {
        true
    }
}

// But not such cases:
//
// impl<Prev: Snapshot> SnapshotPair<EmptySnap, Prev> {
//     fn has_empty(&self) -> bool {
//         true
//     }
// }
// impl<Curr: Snapshot> SnapshotPair<Curr, EmptySnap> {
//     fn has_empty(&self) -> bool {
//         true
//     }
// }
//
// error[E0592]: duplicate definitions with name `has_empty`
//   --> my/experiments/src/bin/test.rs:77:5
//    |
// 77 |     fn has_empty(&self) -> bool {
//    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ duplicate definitions for `has_empty`
// ...
// 82 |     fn has_empty(&self) -> bool {
//    |     --------------------------- other definition for `has_empty`
//
// As they can be intersected: `SnapshotPair<EmptySnap, EmptySnap>`,
// And the compiler would not know which one to use.

// But such cases are possible:

impl<Prev: Snapshot> SnapshotPair<EmptySnap, Prev> {
    fn current(&self) -> &str {
        "empty"
    }
}

impl<Prev: Snapshot> SnapshotPair<RealSnap, Prev> {
    fn current(&self) -> &str {
        "real"
    }
}
