use std::f64::consts::PI;
use num::complex::Complex;

fn main() {
    complex_numbers();
    adding_complex_numbers();
    mathematical_functions();
}

fn complex_numbers() -> () {
    let complex_integer = num::complex::Complex::new(10, 20);
    let complex_float = num::complex::Complex::new(10.1, 20.1);

    println!("Complex integer: {}", complex_integer);
    println!("Complex float: {}", complex_float);
}

fn adding_complex_numbers() -> () {
    let complex_num1 = num::complex::Complex::new(10.0, 20.0); // Must use floats
    let complex_num2 = num::complex::Complex::new(3.1, -4.2);

    let sum = complex_num1 + complex_num2;

    println!("Sum: {}", sum);
}

fn mathematical_functions() -> () {
    let x = Complex::new(0.0, 2.0*PI);
    println!("e^(2i * pi) = {}", x.exp()); // =~1
}
