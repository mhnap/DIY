use std::path::Path;

macro_rules! dbg_path {
    ($var:ident) => {
        dbg_path(stringify!($var), $var);
    };
}

macro_rules! compare_paths {
    ($path1:ident, $path2:ident) => {
        if $path1 == $path2 {
            print!(
                "'{}' is equal to '{}'",
                stringify!($path1),
                stringify!($path2)
            );
        } else {
            print!(
                "'{}' is not equal to '{}'",
                stringify!($path1),
                stringify!($path2)
            );
        }

        if $path1.to_string_lossy() == $path2.to_string_lossy() {
            println!(" (to_string_lossy is equal)");
        } else {
            println!(" (to_string_lossy is not equal)");
        }
    };
}

fn main() {
    let unix_style = Path::new(r"/Users/Administrator/projects");
    dbg_path!(unix_style);

    let windows_style = Path::new(r"\Users\Administrator\projects");
    dbg_path!(windows_style);

    let mixed_style = Path::new(r"/Users\Administrator\projects");
    dbg_path!(mixed_style);

    compare_paths!(unix_style, windows_style);
    compare_paths!(unix_style, mixed_style);
    compare_paths!(windows_style, mixed_style);
    println!();

    let relative_style_1 = Path::new(r"../some/path/.");
    dbg_path!(relative_style_1);

    let relative_style_2 = Path::new(r"./some/..\path\");
    dbg_path!(relative_style_2);

    let relative_style_3 = Path::new(r"some\../path");
    dbg_path!(relative_style_3);

    let with_characters = Path::new(r#"some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;'""#);
    dbg_path!(with_characters);
}

fn dbg_path(var: &str, path: &Path) {
    println!("var: '{var}'");
    println!("debug: '{path:?}'");
    println!("display: '{}'", path.display());
    println!("to_str: '{}'", path.to_str().unwrap_or("cannot convert"));
    for component in path.components() {
        println!("component: '{component:?}'");
    }
    println!();
}

// On Unix:

// var: 'unix_style'
// debug: '"/Users/Administrator/projects"'
// display: '/Users/Administrator/projects'
// to_str: '/Users/Administrator/projects'
// component: 'RootDir'
// component: 'Normal("Users")'
// component: 'Normal("Administrator")'
// component: 'Normal("projects")'

// var: 'windows_style'
// debug: '"\\Users\\Administrator\\projects"'
// display: '\Users\Administrator\projects'
// to_str: '\Users\Administrator\projects'
// component: 'Normal("\\Users\\Administrator\\projects")'

// var: 'mixed_style'
// debug: '"/Users\\Administrator\\projects"'
// display: '/Users\Administrator\projects'
// to_str: '/Users\Administrator\projects'
// component: 'RootDir'
// component: 'Normal("Users\\Administrator\\projects")'

// 'unix_style' is not equal to 'windows_style' (to_string_lossy is not equal)
// 'unix_style' is not equal to 'mixed_style' (to_string_lossy is not equal)
// 'windows_style' is not equal to 'mixed_style' (to_string_lossy is not equal)

// var: 'relative_style_1'
// debug: '"../some/path/."'
// display: '../some/path/.'
// to_str: '../some/path/.'
// component: 'ParentDir'
// component: 'Normal("some")'
// component: 'Normal("path")'

// var: 'relative_style_2'
// debug: '"./some/..\\path\\"'
// display: './some/..\path\'
// to_str: './some/..\path\'
// component: 'CurDir'
// component: 'Normal("some")'
// component: 'Normal("..\\path\\")'

// var: 'relative_style_3'
// debug: '"some\\../path"'
// display: 'some\../path'
// to_str: 'some\../path'
// component: 'Normal("some\\..")'
// component: 'Normal("path")'

// var: 'with_characters'
// debug: '"some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;\'\""'
// display: 'some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;'"'
// to_str: 'some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;'"'
// component: 'Normal("some")'
// component: 'Normal("!p@a#t$h%")'
// component: 'Normal("^w&i*t(h)")'
// component: 'Normal("-c=h_a+r[a]c{t}e,r.s;\'\"")'

//

// On Windows:

// var: 'unix_style'
// debug: '"/Users/Administrator/projects"'
// display: '/Users/Administrator/projects'
// to_str: '/Users/Administrator/projects'
// component: 'RootDir'
// component: 'Normal("Users")'
// component: 'Normal("Administrator")'
// component: 'Normal("projects")'

// var: 'windows_style'
// debug: '"\\Users\\Administrator\\projects"'
// display: '\Users\Administrator\projects'
// to_str: '\Users\Administrator\projects'
// component: 'RootDir'
// component: 'Normal("Users")'
// component: 'Normal("Administrator")'
// component: 'Normal("projects")'

// var: 'mixed_style'
// debug: '"/Users\\Administrator\\projects"'
// display: '/Users\Administrator\projects'
// to_str: '/Users\Administrator\projects'
// component: 'RootDir'
// component: 'Normal("Users")'
// component: 'Normal("Administrator")'
// component: 'Normal("projects")'

// 'unix_style' is equal to 'windows_style' (to_string_lossy is not equal)
// 'unix_style' is equal to 'mixed_style' (to_string_lossy is not equal)
// 'windows_style' is equal to 'mixed_style' (to_string_lossy is not equal)

// var: 'relative_style_1'
// debug: '"../some/path/."'
// display: '../some/path/.'
// to_str: '../some/path/.'
// component: 'ParentDir'
// component: 'Normal("some")'
// component: 'Normal("path")'

// var: 'relative_style_2'
// debug: '"./some/..\\path\\"'
// display: './some/..\path\'
// to_str: './some/..\path\'
// component: 'CurDir'
// component: 'Normal("some")'
// component: 'ParentDir'
// component: 'Normal("path")'

// var: 'relative_style_3'
// debug: '"some\\../path"'
// display: 'some\../path'
// to_str: 'some\../path'
// component: 'Normal("some")'
// component: 'ParentDir'
// component: 'Normal("path")'

// var: 'with_characters'
// debug: '"some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;\'\""'
// display: 'some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;'"'
// to_str: 'some/!p@a#t$h%/^w&i*t(h)/-c=h_a+r[a]c{t}e,r.s;'"'
// component: 'Normal("some")'
// component: 'Normal("!p@a#t$h%")'
// component: 'Normal("^w&i*t(h)")'
// component: 'Normal("-c=h_a+r[a]c{t}e,r.s;\'\"")'
