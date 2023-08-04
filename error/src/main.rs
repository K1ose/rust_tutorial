fn welcome_panic() {
    // panic!("Hello, error!");
    let v = vec![1, 2, 3];
    v[50];
}

use core::panic;
use std::{error, fs::File, io::ErrorKind, thread::panicking};

fn openfile() {
    let greeting_file_result = File::open("hello.txt");

    /*
    enum Result<T, E>{
        Ok(T),
        Err(E),
    }
     */
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(err) => panic!("Problem opening the file: {:?}", err),
    };
}

fn open_file_with_create() {
    let response_file_result = File::open("hi.txt");

    let response_file = match response_file_result {
        Ok(f) => f,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hi.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem other error in the file: {:?}", other_error);
            }
        },
    };

    let response = File::open("hi.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file {:?}", error);
            })
        } else {
            panic!("Problem opening the file {:?}", error);
        }
    });
}

fn open_file_with_unwrap() {
    // let res = File::open("hi.txt").unwrap();
    // let res = File::open("hello.txt").unwrap();
    let res_exp = File::open("hello.txt").expect("hello.txt should be in this directory.");
}

use std::io::{self, Read};
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_res = File::open("hi.txt");
    let mut usename_file = match username_file_res {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match usename_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}
fn main() {
    // openfile();
    // open_file_with_create();
    // open_file_with_unwrap();
    read_username_from_file();
}
