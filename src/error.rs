use std::fs;
use std::io::ErrorKind;

use std::{
    fs::File,
    io::{self, Read},
};

pub fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(uname) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_file_contents() -> Result<String, io::Error> {
    let mut file = File::open("hedasdasllo.txt")?;

    let mut contents = String::new();

    file.read_to_string(&mut contents)?;

    println!("file content {}", contents);

    Ok(contents)
}

fn read_username_from_file_most_idiomatic() -> Result<String, io::Error> {
    // This single line does the exact same work as your original 10 lines
    fs::read_to_string("hello.txt")
}

pub fn demonstrate_unwrap_or_else() {
    // compiler infer the error itself when _ is present

    let bad_input: Result<i32, _> = "some_else".parse();

    let score = bad_input.unwrap_or_else(|err| {
        println!("parse error occured {}", err);
        return 0;
    });

    println!("The final score is: {}", score);
    // Output: The final score is: 0 (The program does NOT crash)
}

pub fn demonstrate_unwrap() {
    // Case A: Success (Ok)
    let good_input: Result<i32, _> = "42".parse();
    // No panic here, 'score' gets the value 42
    let score = good_input.unwrap();
    println!("Successfully parsed score: {}", score);

    // Case B: Failure (Err)
    let bad_input: Result<i32, _> = "forty-two".parse();

    // !!! THIS LINE WILL CAUSE THE PROGRAM TO PANIC AND CRASH !!!
    // let score_crashes = bad_input.unwrap();
    // println!("This line is never reached.");
}

pub fn read_or_create_file() {
    let hello_file_result = File::open("hello.txt");

    let hello_file = match hello_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file {}", e),
            },
            _ => {
                panic!("Error opening the file")
            }
        },
    };
}
