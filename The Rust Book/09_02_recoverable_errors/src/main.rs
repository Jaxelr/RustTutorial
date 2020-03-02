use std::fs::File;
use std::io::ErrorKind;
use std::io::Read;
use std::io;

fn main() {
    let f = File::open("hello.txt");

    //Syntax with match
    let _f = match f {
        Ok(file) =>  file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) =>  fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    //Unwrap
    let f = File::open("hello.txt").unwrap();

    let f = File::open("hello.txt")
            .expect("Failed to open hello.txt"); //This will 
}

fn rustacean_approach() {
    //more Rustacean approach
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}

//This is the verbose way 
fn read_username_from_file() -> Result<String, io::Error> {
    let f  = File::open("hello.txt");
 
    let mut f = match f {
        Ok(file) =>  file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) =>  Ok(s),
        Err(e) => Err(e),
    }
}

//This is an implementation using the ? Operator
fn read_username_from_file_brief() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

//Even briefer implementation using the chaining of Result<T, E>
fn read_username_from_file_briefer() ->Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;
    
    Ok(s)
}