// File struct
use std::fs::File;
// Helps perform read and write operations to file
use std::io::prelude::*;

/**
 * Testing out file import
 */
pub fn experiment_files() {
    let open_file_url = "./files/test_string.txt";
    let create_file_url = "./files/new_file.txt";
    // Open a specific file
    let mut opened_file = File::open(open_file_url)
        .expect("Error: when opening file 1!");

    let mut created_file = File::create(create_file_url)
        .expect("Error: when creating file 2!");

    // Creating new string to hold contents of file
    let mut contents_f1 = String::new();
    let mut contents_f2 = String::new();

    // reading file contents and storing them in a string, expect is used to handle errors
    opened_file.read_to_string(&mut contents_f1)
        .expect("Error: file 1 reading to string");
    
    println!("Contents of File 1:\n{}", contents_f1);

    let new_contents = format!("Testing write operation:\n{}", &contents_f1);

    created_file.write_all(new_contents.as_bytes())
        .expect("Error: writing string to file.");

    // Need to reopen the file to access the data
    created_file = File::open(create_file_url)
                        .expect("Error: when opening file 2!");
    
    created_file.read_to_string(&mut contents_f2)
        .expect("Error: file 2 reading to string");
        
    println!("Contents of File 2:\n{}", contents_f2);
}

