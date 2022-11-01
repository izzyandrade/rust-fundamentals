use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut file: File = File::open("files/readthisexample.txt").expect("Ooops! Couldn't open the file!");

    let mut file_content: String = String::new();
    file.read_to_string(&mut file_content).expect("Oops! Cannot write file contents");

    println!("File content: \n\n{}", file_content);
}