use regex::Regex;
use std::io;

fn main() {
    let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    println!("Did our date match? {}", re.is_match("2020-01-02"));

    //Added by Me
    println!("Please Input your date: ");

    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    match re.is_match(&input.trim()) {
            true => println!("Its a match, your date is {}", input),
            false => println!("This is an unmatched date yo!")
    };

}
