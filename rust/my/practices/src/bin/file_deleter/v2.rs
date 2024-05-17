use std::{ops::Deref, path::PathBuf, sync::Arc};

#[derive(Debug, Clone)]
pub struct FileDeleter {
    file_deleter_inner: Arc<FileDeleterInner>,
}

impl Deref for FileDeleter {
    type Target = FileDeleterInner;

    fn deref(&self) -> &Self::Target {
        self.file_deleter_inner.as_ref()
    }
}

#[derive(Debug, Clone)]
pub struct FileDeleterInner {
    pub file_path: PathBuf,
}

impl FileDeleter {
    pub fn new(file_path: impl Into<PathBuf>) -> Self {
        Self {
            file_deleter_inner: Arc::new(FileDeleterInner {
                file_path: file_path.into(),
            }),
        }
    }
}

impl Drop for FileDeleterInner {
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
