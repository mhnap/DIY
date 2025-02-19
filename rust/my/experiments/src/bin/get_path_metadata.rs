use chrono::{DateTime, Utc};
use std::{
    env, error, fs,
    io::{self, Read},
    os,
    path::Path,
};

#[cfg(unix)]
use os::unix::prelude::*;

#[cfg(windows)]
use os::windows::prelude::*;

fn main() -> Result<(), Box<dyn error::Error>> {
    if let Some(path) = env::args().skip(1).next() {
        let full_path = fs::canonicalize(path)?;
        print_file_metadata(&full_path)?;

        // Try to read one byte if this is a file.
        if full_path.is_file() {
            dbg!(try_read_one_byte(&full_path)?);
        }
    } else {
        if cfg!(unix) {
            print_file_metadata("/sys/kernel/mm/hugepages/hugepages-2048kB/resv_hugepages")?;
        } else if cfg!(windows) {
            print_file_metadata(r#"C:\Windows\System32\LogFiles\WMI\RtBackup"#)?;
        } else {
            unreachable!();
        }
    }

    Ok(())
}

fn print_file_metadata(path: impl AsRef<Path>) -> Result<(), io::Error> {
    dbg!(path.as_ref());
    let metadata = fs::metadata(path)?;

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
        use os::windows::fs::FileTypeExt;
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

    dbg!(metadata.modified(), metadata.modified().map(DateTime::<Utc>::from));
    dbg!(metadata.accessed(), metadata.accessed().map(DateTime::<Utc>::from));
    dbg!(metadata.created(), metadata.created().map(DateTime::<Utc>::from));

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
        dbg!(metadata.atime(), DateTime::from_timestamp(metadata.atime(), 0));
        dbg!(metadata.mtime(), DateTime::from_timestamp(metadata.mtime(), 0));
        dbg!(metadata.ctime(), DateTime::from_timestamp(metadata.ctime(), 0));
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

fn try_read_one_byte(path: &Path) -> Result<u8, io::Error> {
    let mut file = fs::File::open(path)?;
    let mut byte = [0; 1];
    file.read(&mut byte)?;
    Ok(byte[0])
}
