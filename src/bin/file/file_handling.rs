// --------------------------------------------
//            Basic File Handling
// --------------------------------------------

use std::fs::*;
use std::io::{BufRead, BufReader, Read, Result, Write};
use std::path::Path;

fn main() {
    let _ = basic_file_handling();
}

fn basic_file_handling() -> Result<()> {
    /*
        r"..." is used to create a raw string for
        for avoiding the need to escape backslashes(\)
        and escape colons(:) in Windows file paths.
    */
    let path_loc = r"D:\Coding\Rust\basic-advance\src\bin\file\my_text.txt";
    let path = Path::new(path_loc);

    // create file with new content
    // let mut file = File::create(path)?;
    // file.write_all(b"let's put this line1 in the file")?; // b"" is used to create a byte string
    // file.write_all("let's put this line2 in the file".as_bytes())?; // Alternative way to create a byte string

    // append the content to the existing file
    // let mut file = OpenOptions::new().append(true).open(path)?;
    // file.write_all(b"\n www.includehelp.con\n")?;

    // let str = "nouman";
    // file.write_all(str.as_bytes())?;

    // Write numbers to the file by formatting the array with combinator
    // let numbers = [1, 2, 3, 4, 5];
    // let str_from_vec: String = numbers
    //     .iter()
    //     .map(|n| n.to_string())
    //     .collect::<Vec<_>>()
    //     .join("\n");
    // file.write_all(str_from_vec.as_bytes())?;

    // Write formatted string to the file
    // let (name, age) = ("Joseph", 40);
    // let formatted_string = format!("I am {} and my age is {}", name, age);
    // file.write_all(formatted_string.as_bytes())?;

    // Read file v1
    // let mut file = File::open(path)?;
    // let mut contents = String::new();
    // file.read_to_string(&mut contents)?;
    // println!("The file contains: {}", contents);

    // Read file v2
    let file = File::open(path)?;
    let file_buffer = BufReader::new(file);
    for line in file_buffer.lines() {
        println!("{:?}", line?);
    }

    Ok(())
}
