use std::fs::{self, File};
use std::io::ErrorKind;
use std::io::{self, Read};

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    //
    // v[99];

    // let greeting_file_result = File::open("hello.txt");
    //
    // let greeting_file = match greeting_file_result {
    //     Ok(file) => file,
    //     Err(error) => match error.kind() {
    //         ErrorKind::NotFound => match File::create("hello.txt") {
    //             Ok(fc) => fc,
    //             Err(e) => panic!("Problem creating the file: {e:?}"),
    //         },
    //         other_error => {
    //             panic!("Problem opening the file: {error:?}");
    //         }
    //     },
    // };
}

// the propagated error will be of the same type in the return statement
fn read_username_from_file() -> Result<String, io::Error> {
    // long way //
    // let username_file_result = File::open("hello.txt");
    // let mut username_file = match username_file_result {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    //
    // short way //
    // let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();

    // long way //
    // match username_file.read_to_string(&mut username) {
    //     Ok(_) => Ok(username),
    //     Err(e) => Err(e),
    // }
    //
    // short way //
    // username_file.read_to_string(&mut username)?;
    // Ok(username)

    // shorter way //
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)

    // shortest way //
    // fs::read_to_string("hello.txt")
}
