// A custom struct called vector, as a workable example
pub struct Vec {
    len: usize,
}

impl Vec {
    // Note this is a static method - no self.
    // This constructor doesn't take any arguments, but some might in order to
    // properly initialise an object
    pub fn new() -> Vec {
        Vec {
            len: 0
        }
    }
}

fn main() {
    let vec = Vec::new();

    println!("{}", vec.len);
}
