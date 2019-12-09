fn main() {
    //We can mutate the string type String
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() appends a literal to a String

    println!("{}", s); // This will print `hello, world!`

    let s1 = String::from("hello");
    let _s2 = s1;

    //Uncomment to create an error on unusability
    //println!("{}, world!", s1);

    //Using clone this works correctly
    
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4);

    //Succint explanation per the book
    let xs = String::from("hello");     // xs comes into scope

    takes_ownership(xs);                // xs's value moves into the function...
                                        // ... and so is no longer valid here

    let sx = 5;                         // sx comes into scope

    makes_copy(sx);                     // sx would move into the function,
                                        // but i32 is Copy, so it’s okay to still
                                        // use sx afterward

    let _s7 = gives_ownership();         // gives_ownership moves its return
                                        // value into s7

    let s8 = String::from("hello");     // s8 comes into scope

    let _s9 = takes_and_gives_back(s8);  // s8 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into _s9
    
    let s5 = String::from("hello");

    //Returning a tuple
    let (s6, len) = calculate_length(s5);

    println!("The length of '{}' is {}.", s6, len);

} // Here, sx goes out of scope, then xs. But because xs's value was moved, nothing
  // special happens.
  // Here, _s9 goes out of scope and is dropped. s8 goes out of scope but was
  // moved, so nothing happens. _s7 goes out of scope and is dropped.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}