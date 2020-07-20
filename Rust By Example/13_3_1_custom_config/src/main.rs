//In order for this to work you must run the following command
// from the src folder
// rustc --cfg some_condition main.rs && ./main

#[cfg(some_condition)]
fn conditional_function() {
    println!("condition met!");
}

fn main() {
    conditional_function();
}