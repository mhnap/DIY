fn main() {
    let v: Vec<i32> = vec![];
    // let chunks: Vec<Vec<_>> = v.chunks(0).map(|s| s.to_vec()).collect();
    // panic: chunk size must be non-zero

    let chunks: Vec<Vec<_>> = v.chunks(1).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 0);

    let chunks: Vec<Vec<_>> = v.chunks(2).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 0);

    //

    let v: Vec<i32> = vec![1];

    let chunks: Vec<Vec<_>> = v.chunks(1).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], vec![1]);

    let chunks: Vec<Vec<_>> = v.chunks(2).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], vec![1]);

    //

    let v: Vec<i32> = vec![1, 2];

    let chunks: Vec<Vec<_>> = v.chunks(1).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], vec![1]);
    assert_eq!(chunks[1], vec![2]);

    let chunks: Vec<Vec<_>> = v.chunks(2).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], vec![1, 2]);

    let chunks: Vec<Vec<_>> = v.chunks(3).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], vec![1, 2]);

    //

    let v: Vec<i32> = vec![1, 2, 3];

    let chunks: Vec<Vec<_>> = v.chunks(1).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 3);
    assert_eq!(chunks[0], vec![1]);
    assert_eq!(chunks[1], vec![2]);
    assert_eq!(chunks[2], vec![3]);

    let chunks: Vec<Vec<_>> = v.chunks(2).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 2);
    assert_eq!(chunks[0], vec![1, 2]);
    assert_eq!(chunks[1], vec![3]);

    let chunks: Vec<Vec<_>> = v.chunks(3).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], vec![1, 2, 3]);

    let chunks: Vec<Vec<_>> = v.chunks(4).map(|s| s.to_vec()).collect();
    assert_eq!(chunks.len(), 1);
    assert_eq!(chunks[0], vec![1, 2, 3]);
}
