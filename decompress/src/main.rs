#![deny(clippy::all)]
//when working with files
use std::fs; // The fs module is used for file system interactions, like reading from or writing to files.
use std::io; // The io module provides Rust's basic input and output functionality.

fn main() {
    //help you exit cleanly from the actual logic 
    std::process::exit(real_main());
}

//It exits the program with the status code returned by real_main
//This separation allows real_main to handle all the logic and return a status code indicating success (0) or failure (non-zero), which is a common practice in command-line applications.
fn real_main() -> i32{
    //put args in a vec
let args : Vec<_> = std::env::args().collect();

if args.len() <2{
    println!("Usage: {} <filename>",args[0]);
    return 1; 
}

let fname=std::path::Path::new(&args[1]);
//kan momken fo2 fs::File wa akteb hena file 3alatol
let file = fs::File::open(&fname).unwrap();
//use zip archive reader
let mut archive= zip::ZipArchive::new(file).unwrap();

}