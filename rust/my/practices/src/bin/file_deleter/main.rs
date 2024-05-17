use std::{fs, path::Path, sync::Arc};

mod v1;
mod v2;
mod v3;

fn main() {
    pretty_print("Basic case");
    {
        let file_path = Path::new("example.txt");
        fs::File::create(file_path).expect("Failed to create file");
        println!("File '{}' created", file_path.display());
        let _file_deleter = v1::FileDeleter::new(file_path);
    }
    println!();
    // File will be deleted.

    pretty_print("Use `FileDeleter` twice");
    {
        let file_path = Path::new("example.txt");
        fs::File::create(file_path).expect("Failed to create file");
        println!("File '{}' created", file_path.display());
        let file_deleter_1 = v1::FileDeleter::new(file_path);
        assert_eq!(file_deleter_1.file_path, file_path);
        let file_deleter_2 = file_deleter_1.clone();
        assert_eq!(file_deleter_2.file_path, file_path);
    }
    println!();
    // File will be deleted by first `FileDeleter`.
    // And second `FileDeleter` will give error.

    pretty_print("Use `Arc` for deleting only once");
    {
        let file_path = Path::new("example.txt");
        fs::File::create(file_path).expect("Failed to create file");
        println!("File '{}' created", file_path.display());
        let file_deleter_1 = Arc::new(v1::FileDeleter::new(file_path));
        assert_eq!(file_deleter_1.file_path, file_path);
        let file_deleter_2 = file_deleter_1.clone();
        assert_eq!(file_deleter_2.file_path, file_path);
    }
    println!();

    pretty_print("Use `FileDeleter` that has `Arc` inside");
    {
        let file_path = Path::new("example.txt");
        fs::File::create(file_path).expect("Failed to create file");
        println!("File '{}' created", file_path.display());
        let file_deleter_1 = v2::FileDeleter::new(file_path);
        assert_eq!(file_deleter_1.file_path, file_path);
        let file_deleter_2 = file_deleter_1.clone();
        assert_eq!(file_deleter_2.file_path, file_path);
    }
    println!();

    pretty_print("Can be easily moved to different scopes");
    {
        let file_path = Path::new("example.txt");
        fs::File::create(file_path).expect("Failed to create file");
        println!("File '{}' created", file_path.display());
        let file_deleter_1 = v2::FileDeleter::new(file_path);
        assert_eq!(file_deleter_1.file_path, file_path);
        let file_deleter_3 = {
            let file_deleter_2 = file_deleter_1;
            assert_eq!(file_deleter_2.file_path, file_path);
            file_deleter_2.clone()
        };
        assert_eq!(file_deleter_3.file_path, file_path);
    }
    println!();

    pretty_print("Will not delete not exists file");
    {
        let file_path = Path::new("example.txt");
        let _file_deleter = v3::FileDeleter::new(file_path.into(), false);
    }
    println!();

    pretty_print("Retain file, so it won't be deleted");
    {
        let file_path = Path::new("example.txt");
        fs::File::create(file_path).expect("Failed to create file");
        println!("File '{}' created", file_path.display());
        let _file_deleter = v3::FileDeleter::new(file_path.into(), true);
    }
    println!();
}

fn pretty_print(str: &str) {
    println!("--- {str} ---");
}
