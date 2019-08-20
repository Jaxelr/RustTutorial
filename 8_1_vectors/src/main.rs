enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String)
}

fn main() {
    //Types of vector initialization
    //using new()
    let mut v: Vec<i32> = Vec::new();

    //pushing values after, must be mut
    v.push(1);
    v.push(2);
    v.push(3);

    //assigning with macro, in this case is immutable
    let v2 = vec![1, 2, 3];

    let third: &i32 = &v2[2]; //0 based like a sane language

    println!("The third element is {}", third);

    match v.get(1) {
        Some(second) => println!("The second element is {}", second),
        None => println!("There is no second element on the vector"),
    }

    //Iterating over vectors
    let v3 = vec![100, 32, 57];

    for i in &v3 {
        println!("{}", i);
    }

    //Mutating on each iteration while showing the result
    let mut v4 = vec![100, 32, 57];

    for i in &mut v4 {
        *i += 50;
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.11),
        SpreadsheetCell::Text(String::from("hellow"))
    ];

    for i in &row {
        match i {
            SpreadsheetCell::Text(text) => { println!("This is text {}", text); },
            SpreadsheetCell::Int(integer) => { println!("This is an integer {}", integer); },
            SpreadsheetCell::Float(float) => { println!("This is a float {}", float); },
        }
    }
}
