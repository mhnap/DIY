use chrono::{DateTime, Utc};

#[cfg(unix)]
use std::os::unix::prelude::*;

#[cfg(windows)]
use std::os::windows::prelude::*;

fn main() {
    if let Some(path) = std::env::args().skip(1).next() {
        let full_path = std::fs::canonicalize(path).unwrap();
        print_file_metadata(full_path).unwrap();
    } else {
        if cfg!(unix) {
            print_file_metadata("/sys/kernel/mm/hugepages/hugepages-2048kB/resv_hugepages")
                .unwrap();
        } else if cfg!(windows) {
            print_file_metadata(r#"C:\Windows\System32\LogFiles\WMI\RtBackup"#).unwrap();
        } else {
            unreachable!();
        }
    }
}

fn print_file_metadata(path: impl AsRef<std::path::Path>) -> Result<(), std::io::Error> {
    dbg!(path.as_ref());
    let metadata = std::fs::metadata(path)?;

    let file_type = metadata.file_type();
    dbg!(file_type.is_dir());
    dbg!(file_type.is_file());
    dbg!(file_type.is_symlink());

    #[cfg(unix)]
    {
        dbg!(file_type.is_block_device());
        dbg!(file_type.is_char_device());
        dbg!(file_type.is_fifo());
        dbg!(file_type.is_socket());
    }
    #[cfg(windows)]
    {
        dbg!(file_type.is_symlink_dir());
        dbg!(file_type.is_symlink_file());
    }

    dbg!(metadata.len());

    let permissions = metadata.permissions();
    dbg!(permissions.readonly());
    #[cfg(unix)]
    {
        dbg!(permissions.mode());
    }

    dbg!(
        metadata.modified(),
        metadata.modified().map(DateTime::<Utc>::from)
    );
    dbg!(
        metadata.accessed(),
        metadata.accessed().map(DateTime::<Utc>::from)
    );
    dbg!(
        metadata.created(),
        metadata.created().map(DateTime::<Utc>::from)
    );

    #[cfg(unix)]
    {
        dbg!(metadata.dev());
        dbg!(metadata.ino());
        dbg!(metadata.mode());
        dbg!(metadata.nlink());
        dbg!(metadata.uid());
        dbg!(metadata.gid());
        dbg!(metadata.rdev());
        dbg!(metadata.size());
        dbg!(
            metadata.atime(),
            DateTime::from_timestamp(metadata.atime(), 0)
        );
        dbg!(
            metadata.mtime(),
            DateTime::from_timestamp(metadata.mtime(), 0)
        );
        dbg!(
            metadata.ctime(),
            DateTime::from_timestamp(metadata.ctime(), 0)
        );
        dbg!(metadata.blksize());
        dbg!(metadata.blocks());
    }
    #[cfg(windows)]
    {
        dbg!(metadata.file_attributes());
        dbg!(metadata.creation_time());
        dbg!(metadata.last_access_time());
        dbg!(metadata.last_write_time());
        dbg!(metadata.file_size());
    }

    Ok(())
}
