use relative_path::RelativePathBuf;
use std::path::PathBuf;

macro_rules! nit_dbg {
    ($val:expr $(,)?) => {
        match $val {
            tmp => {
                println!("{} = {:?}", stringify!($val), &tmp);
                tmp
            }
        }
    };
    ($($val:expr),+ $(,)?) => {
        ($(nit_dbg!($val)),+,)
    };
}

macro_rules! dbg_components {
    ($path:expr) => {
        println!("{}", stringify!($path));
        for component in $path.components() {
            println!("{component:?}");
        }
        println!();
    };
}

macro_rules! test_path {
    ($path:expr) => {
        let path = nit_dbg!($path);
        let path = nit_dbg!(path.join("path"));
        let path = nit_dbg!(path.join("/path"));
        let path = nit_dbg!(path.join("//path"));
        let path = nit_dbg!(path.join("path/"));
        let path = nit_dbg!(path.join("path//"));
        nit_dbg!(path.join("/path"));
        println!();
    };
}

fn main() {
    // `RelativePathBuf` stores only UTF-8 paths, so it's easily convertible to str.
    let path = RelativePathBuf::from("some/long/path");
    nit_dbg!(path.as_str());
    println!();

    // Also it uses '/' as a separator on all platforms.
    dbg_components!(RelativePathBuf::from("some/long/path"));
    dbg_components!(PathBuf::from("some/long/path"));
    dbg_components!(RelativePathBuf::from("some/long\\path"));
    dbg_components!(PathBuf::from("some/long\\path"));

    // And it also differs from `PathBuf` because it won't replace the current path if a new path is absolute.
    test_path!(PathBuf::from("/"));
    test_path!(RelativePathBuf::from("/"));
}

// On Unix:

// path.as_str() = "some/long/path"

// RelativePathBuf::from("some/long/path")
// Normal("some")
// Normal("long")
// Normal("path")

// PathBuf::from("some/long/path")
// Normal("some")
// Normal("long")
// Normal("path")

// RelativePathBuf::from("some/long\\path")
// Normal("some")
// Normal("long\\path")

// PathBuf::from("some/long\\path")
// Normal("some")
// Normal("long\\path")

// PathBuf::from("/") = "/"
// path.join("path") = "/path"
// path.join("/path") = "/path"
// path.join("//path") = "//path"
// path.join("path/") = "//path/path/"
// path.join("path//") = "//path/path/path//"
// path.join("/path") = "/path"

// RelativePathBuf::from("/") = "/"
// path.join("path") = "/path"
// path.join("/path") = "/path/path"
// path.join("//path") = "/path/path//path"
// path.join("path/") = "/path/path//path/path/"
// path.join("path//") = "/path/path//path/path/path//"
// path.join("/path") = "/path/path//path/path/path//path"

//

// On Windows:

// path.as_str() = "some/long/path"

// RelativePathBuf::from("some/long/path")
// Normal("some")
// Normal("long")
// Normal("path")

// PathBuf::from("some/long/path")
// Normal("some")
// Normal("long")
// Normal("path")

// RelativePathBuf::from("some/long\\path")
// Normal("some")
// Normal("long\\path")

// PathBuf::from("some/long\\path")
// Normal("some")
// Normal("long")
// Normal("path")

// PathBuf::from("/") = "/"
// path.join("path") = "/path"
// path.join("/path") = "/path"
// path.join("//path") = "//path"
// path.join("path/") = "//path\\path/"
// path.join("path//") = "//path\\path/path//"
// path.join("/path") = "//path\\path/path"

// RelativePathBuf::from("/") = "/"
// path.join("path") = "/path"
// path.join("/path") = "/path/path"
// path.join("//path") = "/path/path//path"
// path.join("path/") = "/path/path//path/path/"
// path.join("path//") = "/path/path//path/path/path//"
// path.join("/path") = "/path/path//path/path/path//path"
