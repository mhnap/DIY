use temp_file::TempFile;

fn main() {
    // Create new temp file.
    let temp_file = TempFile::new().unwrap();
    let file_path = temp_file.path().to_path_buf();
    dbg!(&file_path);

    // Write to file.
    std::fs::write(&file_path, b"abc").unwrap();
    assert_eq!(std::fs::read_to_string(&file_path).unwrap(), "abc");

    // Temp file will be deleted on drop.
    drop(temp_file);

    // File is already deleted.
    assert!(matches!(std::fs::read_to_string(&file_path), Err(_)));
}
