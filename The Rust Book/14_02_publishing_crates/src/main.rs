//! # Additions
//! 
//! `additions` is a crate i just created as an example.

fn main() {

    //Run cargo doc to see all this goodie documentation generated by itself.
    let x = 5;

    let result = add_one(x);

    println!("This is the result of adding 1 to {}: {}", x, result);
}


/// Adds one to the number given.
///
/// # Examples
///
/// ```
/// let arg = 5;
/// let answer = my_crate::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}