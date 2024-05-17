//! This module contains a small helper struct [`FileDeleter`] that will delete a file by its path on drop.

use std::{ops::Deref, path::PathBuf, sync::Arc};

/// Helps to delete a file by its path on drop.
/// [`FileDeleter`] is a wrapped [`FileDeleterInner`] in [`Arc`] for easier cloning,
/// and more importantly for deleting *only* when the last [`FileDeleter`] will drop.
#[derive(Debug, Clone)]
pub struct FileDeleter {
    file_deleter_inner: Arc<FileDeleterInner>,
}

impl FileDeleter {
    pub fn new(path: PathBuf, retain: bool) -> Self {
        Self {
            file_deleter_inner: Arc::new(FileDeleterInner { path, retain }),
        }
    }
}

impl Deref for FileDeleter {
    type Target = FileDeleterInner;

    fn deref(&self) -> &Self::Target {
        &self.file_deleter_inner
    }
}

/// Deletes a file by its path on drop.
/// Can specify whether to retain a file so it won't be deleted.
#[derive(Debug, Clone)]
pub struct FileDeleterInner {
    pub path: PathBuf,
    pub retain: bool,
}

impl Drop for FileDeleterInner {
    fn drop(&mut self) {
        if self.retain {
            println!("File '{}' will be retained", self.path.display());
        } else {
            if self.path.exists() {
                if let Err(err) = std::fs::remove_file(&self.path) {
                    println!("Failed to delete '{}' file: {}", self.path.display(), err);
                } else {
                    println!("Deleted '{}' file", self.path.display());
                }
            } else {
                println!(
                    "File '{}' won't be deleted as it does not exist",
                    self.path.display()
                );
            }
        }
    }
}
