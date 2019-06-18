fn main() {
    println!("Hello, world!");

    //Primitive Data Types

    //Scalar
    let val1: u8 = 16;
    let _val1: i8 = -16;

    //These size integers reach until size 128 on 
    
    //u16
    //u32
    //u64
    //u128

    //Floating-Point Types
    let fl = 2.0;
    let xfl: f32 = 3.0;

    println!("This is float {} and this is float32 {}", fl , xfl);

    //Arithmetic operators
    //addition
    let _sum = 5 + 10;
    // subtraction
    let _difference = 95.5 - 4.3;
    // multiplication
    let _product = 4 * 30;
    // division
    let _quotient = 56.7 / 32.2;
    // remainder
    let _remainder = 43 % 5;

    //Boolean types
    let _t = true;
    let _f: bool = false; // with explicit type annotation

    //Character Type
    let _c = 'z';
    let _z = 'ℤ';
    let _heart_eyed_cat = '😻';

    //Compound Types

    //Tuples
    let tup = (500, 6.4, 1);

    //Destructuring
    let (_x, y, _z) = tup;

    println!("The value of y is: {}", y);

    //Accessing elements of the tuple

    let xtup: (i32, f64, u8) = (500, 6.4, 1);

    let _five_hundred = xtup.0;

    let _six_point_four = xtup.1;

    let _one = xtup.2;

    //Arrays

    let _months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    //Alternate declarations
    let _array_declaration: [i32; 5] = [1, 2, 3, 4, 5];

    //Concise declarations
    let _short_array_declaration = [3; 5]; // [3, 3, 3, 3, 3]
    
    let tiny_array = [1, 2, 3, 4, 5];
    let index = 4;

    let element = tiny_array[index];

    println!("The value of element is: {}", element);
}   
