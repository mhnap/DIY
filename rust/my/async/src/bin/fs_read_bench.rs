// https://users.rust-lang.org/t/unexpected-performance-results-comparing-async-vs-sync/46156

use std::future::Future;
use std::io::Read;
use std::path::Path;
use std::pin::Pin;
use std::time::Instant;
use tokio::io::AsyncReadExt;

const READ_BUF_SIZE: usize = 1024;

fn read_dir_tokio<'a>(
    dir: &'a std::path::Path,
    count: &'a mut usize,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        if dir.is_dir() {
            let mut entries = tokio::fs::read_dir(dir).await.unwrap();
            while let Some(entry) = entries.next_entry().await.unwrap() {
                *count += 1;
                let path = entry.path();
                if path.is_dir() {
                    read_dir_tokio(&path, count).await;
                } else {
                    let mut file = tokio::fs::File::open(&path).await.unwrap();
                    let mut buf = vec![0; READ_BUF_SIZE];
                    file.read(&mut buf).await.unwrap();
                }
            }
        }
    })
}

fn read_dir_tokio_uring<'a>(
    dir: &'a std::path::Path,
    count: &'a mut usize,
) -> Pin<Box<dyn Future<Output = ()> + 'a>> {
    Box::pin(async move {
        if dir.is_dir() {
            let mut entries = tokio::fs::read_dir(dir).await.unwrap();
            while let Some(entry) = entries.next_entry().await.unwrap() {
                *count += 1;
                let path = entry.path();
                if path.is_dir() {
                    read_dir_tokio_uring(&path, count).await;
                } else {
                    let file = tokio_uring::fs::File::open(&path).await.unwrap();
                    let buf = vec![0; READ_BUF_SIZE];
                    file.read_at(buf, 0).await.0.unwrap();
                }
            }
        }
    })
}

fn read_dir_sync(dir: &Path, count: &mut usize) {
    if dir.is_dir() {
        let entries = std::fs::read_dir(dir).unwrap();
        for entry in entries {
            *count += 1;
            let path = entry.unwrap().path();
            if path.is_dir() {
                read_dir_sync(&path, count);
            } else {
                let mut file = std::fs::File::open(&path).unwrap();
                let mut buf = vec![0; READ_BUF_SIZE];
                file.read(&mut buf).unwrap();
            }
        }
    }
}

fn main() {
    let path = Path::new("../../../");
    let mut sync_count = 0;
    let mut tokio_count = 0;
    let mut tokio_uring_count = 0;

    {
        let now = Instant::now();
        read_dir_sync(path, &mut sync_count);
        println!("Sync: {} sec", now.elapsed().as_secs());
    }

    {
        let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();
        let now = Instant::now();
        rt.block_on(read_dir_tokio(path, &mut tokio_count));
        println!("Async: {} sec", now.elapsed().as_secs());
    }

    {
        let rt_uring = tokio_uring::Runtime::new(&tokio_uring::builder()).unwrap();
        let now = Instant::now();
        rt_uring.block_on(read_dir_tokio_uring(path, &mut tokio_uring_count));
        println!("Async uring: {} sec", now.elapsed().as_secs());
    }

    assert_eq!(sync_count, tokio_count);
    assert_eq!(sync_count, tokio_uring_count);
    println!("Total files count: {sync_count}");
}
