use std::fmt::{Display, Debug};

#[derive(Debug)]
struct Rectangle { length: f64, height: f64 }

//#[derive(Debug)]
#[allow(dead_code)]
struct Triangle  { length: f64, height: f64 }

// Define a function `printer` that takes a generic type `T` which
// must implement trait `Display`.
#[allow(dead_code)]
fn printer<T: Display>(t: &T) {
    println!("{}", t);
}

// The generic `T` must implement `Debug`. Regardless
// of the type, this will work properly.
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

trait HasArea {
    fn area(&self) -> f64;
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 { self.length * self.height }
}

// `T` must implement `HasArea`. Any type which meets
// the bound can access `HasArea`'s function `area`.
fn area<T: HasArea>(t: &T) -> f64 { t.area() }

fn main() {

    printer(&String::from("Hello world, printed using my custom printer"));

    let rectangle = Rectangle { length: 3.0, height: 4.0 };
    let _triangle = Triangle  { length: 3.0, height: 4.0 };

    print_debug(&rectangle);
    println!("Area: {}", area(&rectangle));

    //print_debug(&_triangle);
    // println!("Area: {}", area(&_triangle));
    // ^ TODO: Try uncommenting these.
    // | Error: Does not implement either `Debug` or `HasArea`. 
}
