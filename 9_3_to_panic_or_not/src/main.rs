use std::net::IpAddr;

fn main() {
    //Technically valid scenario to call unwrap
    let _home: IpAddr = "127.0.0.1".parse().unwrap();

    // let guess_that_panics = Guess::new(101);

    // println!("Guess: {}", guess_that_panics.value());

    let guess = Guess::new(100);

    println!("Guess that doesnt panic: {}", guess.value());
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}