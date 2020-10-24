use std::io::{self, Read, ErrorKind};
use std::fs::{self, File};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // open_hello_file();
    println!("*******************");
    // open_hello2_file();
    println!("*******************");
    // open_hello3_file();
    println!("*******************");
    // File::open("hell.txt")
    //     // .unwrap();
    //     // thread 'main' panicked at 'Failed to open hell.txt!: Os { code: 2, kind: NotFound, message: "No such file or directory" }', src/libcore/result.rs:997:5
    //     .expect("Failed to open hell.txt!");
    println!("*******************");
    let username = read_username_from_file();
    println!("username: {:?}", username);

    let f = File::open("user2.txt")?;
    Ok(())
}

fn open_hello_file() {
    let f = File::open("hello.txt");

    let f = match f {
        // File { fd: 3, path: "~/recoverable_errors_with_result/hello.txt", read: true, write: false }
        Ok(file) => file,
        // thread 'main' panicked at There was a problem opening the file: Os { code: 2, kind: NotFound, message: "No such file or directory" }, src/main.rs:9:13
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error)
        },
    };
    println!("{:?}", f);
}

fn open_hello2_file() {
    let f = File::open("hello2.txt");

    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello2.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Tried to create file but there was a problem: {:?}", e),
            },
            other_error => panic!("There was a problem opening the file: {:?}", other_error),
        },
    };
}

// closures
fn open_hello3_file() {
    File::open("hello3.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello3.txt").unwrap_or_else(|err| {
                panic!("Tried to create file but there was a problem: {:?}", err);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
}

fn read_username()  -> Result<String, io::Error> {
    let f = File::open("user.txt");
    let mut s = String::new();
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => return Err(e),
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    // let mut s = String::new();
    // // let mut f = File::open("user.txt")?;
    // // f.read_to_string(&mut s)?;
    // let mut f = File::open("user.txt")?.read_to_string(&mut s)?;
    // Ok(s)
    fs::read_to_string("user.txt")
}

// #[derive(Debug)]
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }