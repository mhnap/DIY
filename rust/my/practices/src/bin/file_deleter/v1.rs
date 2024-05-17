use std::path::PathBuf;

#[derive(Debug, Clone)]
pub struct FileDeleter {
    pub file_path: PathBuf,
}

impl FileDeleter {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

impl Drop for FileDeleter {
    fn drop(&mut self) {
        if let Err(err) = std::fs::remove_file(&self.file_path) {
            println!(
                "Failed to delete '{}' file: {}",
                self.file_path.display(),
                err
            );
        } else {
            println!("File '{}' deleted", self.file_path.display());
        }
    }
}
