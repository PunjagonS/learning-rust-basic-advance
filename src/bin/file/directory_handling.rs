// --------------------------------------------
//     Directory and Path Related Functions
// --------------------------------------------

use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let path = Path::new(r"D:\Coding\Rust\basic-advance\src\bin\file\my_text.txt");
    println!("Folder containing the file: {:?}", path.parent().unwrap());

    println!("File name: {:?}", path.file_stem().unwrap());
    println!("File extension: {:?}", path.extension().unwrap());

    // Build path using push
    let mut path = PathBuf::from(r"D:\");
    path.push("Coding");
    path.push("Rust");
    path.push("basic-advance");
    path.push("src");
    path.push("bin");
    path.push("file");
    path.push("my_text");
    path.set_extension("txt");
    println!("The path is: {:?}", path);

    // Build path using iterators
    let path = [
        r"D:\",
        "Coding",
        "Rust",
        "basic-advance",
        "src",
        "bin",
        "file",
        "my_text.txt",
    ]
    .iter()
    .collect::<PathBuf>();
    println!("The path is: {:?}", path);

    // Check if file exists?
    let path = Path::new(r"D:\Coding\Rust\basic-advance\src\bin\file\my_text.txt");
    println!("Does the file exist? {:?}", path.exists());

    // Check if directory exists?
    let path = Path::new(r"D:\Coding\Rust\basic-advance\src\bin\file");
    println!("Does the directory exist? {:?}", path.exists());

    // Check metadata of file
    let data = path.metadata().unwrap();
    println!("type {:?}", data.file_type());
    println!("length {:?}", data.len());
    println!("Permissions {:?}", data.permissions());
    println!("Created {:?}", data.created().unwrap());
    println!("Accessed {:?}", data.accessed().unwrap());
    println!("Modified {:?}", data.modified().unwrap());

    // Loop through all files in a directory
    for file in path.read_dir().expect("Failed to read directory") {
        println!("file: {:?}", file.unwrap().path());
    }

    // Get current working directory
    let current_path = env::current_dir().expect("Failed to access current directory");
    println!("Current path: {:?}", current_path);

    println!(
        "Create a new directory: {:?}",
        fs::create_dir(r"D:\Coding\Rust\basic-advance\src\bin\file\test_create_directory")
    );

    println!(
        "Create a new directory with sub directories: {:?}",
        fs::create_dir_all(
            r"D:\Coding\Rust\basic-advance\src\bin\file\test_create_directory\sub1_directory\sub2_directory"
        )
    );

    println!(
        "Remove a specific directory: {:?}",
        fs::remove_dir(
            r"D:\Coding\Rust\basic-advance\src\bin\file\test_create_directory\sub1_directory\sub2_directory"
        )
    );

    println!(
        "Remove a specific directory when it is not empty: {:?}",
        fs::remove_dir(r"D:\Coding\Rust\basic-advance\src\bin\file\test_create_directory")
    );

    println!(
        "Remove everything from a directory: {:?}",
        fs::remove_dir_all(r"D:\Coding\Rust\basic-advance\src\bin\file\test_create_directory")
    );

    println!(
        "Copy a file: {:?}",
        fs::copy(
            r"D:\Coding\Rust\basic-advance\src\bin\file\my_text.txt",
            r"D:\Coding\Rust\basic-advance\src\bin\file\my_text_copy.txt"
        )
    );

    println!(
        "Remove a file: {:?}",
        fs::remove_file(r"D:\Coding\Rust\basic-advance\src\bin\file\my_text_copy.txt")
    );
}
