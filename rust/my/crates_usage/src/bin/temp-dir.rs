use temp_dir::TempDir;

fn main() {
    // Create new temp dir.
    let temp_dir = TempDir::new().unwrap();
    dbg!(temp_dir.path());

    // Create child file.
    let file_path = temp_dir.child("file1");
    dbg!(&file_path);

    // Write to file.
    std::fs::write(&file_path, b"abc").unwrap();
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "abc");

    // Temp dir will be deleted on drop.
    drop(temp_dir);

    // File is already deleted.
    assert!(matches!(std::fs::read_to_string(&file_path), Err(_)));
}
