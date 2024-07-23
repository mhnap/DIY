use std::{
    env,
    fs::File,
    io::{Read, Write},
    os::{
        fd::{AsRawFd, IntoRawFd},
        unix::io::FromRawFd,
    },
};

fn main() {
    // Create a file and write something to it.
    let path = env::temp_dir().join("file");
    File::create(&path).unwrap().write(b"hello").unwrap();

    {
        // Wrong usage, `File` object will be dropped immediately and thus closed.
        let fd = File::open(&path).unwrap().as_raw_fd();
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(num) => println!("Successfully read '{num}' bytes: '{buf}'."),
            Err(err) => eprintln!("Cannot read file: '{err}'."),
        }
    }

    {
        // Valid usage, `File` object won't be dropped immediately.
        let file = File::open(&path).unwrap();
        let fd = file.as_raw_fd();
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(num) => println!("Successfully read '{num}' bytes: '{buf}'."),
            Err(err) => eprintln!("Cannot read file: '{err}'."),
        }
    }

    {
        // Correct usage, `File` object will be dropped immediately, but fd is moved.
        let fd = File::open(&path).unwrap().into_raw_fd();
        let mut file = unsafe { File::from_raw_fd(fd) };
        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(num) => println!("Successfully read '{num}' bytes: '{buf}'."),
            Err(err) => eprintln!("Cannot read file: '{err}'."),
        }
    }
}
