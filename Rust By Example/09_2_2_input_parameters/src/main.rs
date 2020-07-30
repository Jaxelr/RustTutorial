use std::mem;

// A function which takes a closure as an argument and calls it.
// <F> denotes that F is a "Generic type parameter"
fn apply<F>(f: F) where
    // The closure takes no input and returns nothing.
    F: FnOnce() {
    // ^ TODO: Try changing this to `Fn` or `FnMut`.

    f();
}

// A function which takes a closure and returns an `i32`.
fn apply_to_3<F>(f: F) -> i32 where
    // The closure takes an `i32` and returns an `i32`.
    F: Fn(i32) -> i32 {

    f(3)
}

fn main() {
    let greeting = "hello";

    let mut farewell = "goodbye".to_owned();

    let diary = || {
        // `greeting` is by reference: requires `Fn`.
        println!("I said {}", greeting);

        farewell.push_str("!!!");

        println!("Then i screamed {}.", farewell);
        println!("Now i can sleep. zzzzz");

        mem::drop(farewell);
    };

    // Call the function which applies the closure.
    apply(diary);

    // `double` satisfies `apply_to_3` trait bound
    let double = |x| 2 * x;

    println!("3 doubled: {}", apply_to_3(double));
}
