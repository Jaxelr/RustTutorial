fn main() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), //This is a new variable y, not the same as the scope above.
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);

    let x1 = 1;

    //Matching , multiple values
    match x1 {
        1 | 2 => println!("one or two!"),
        3 => println!("three"),
        _ => println!("Something Else!"),
    }

    //Matching range
    match x1 {
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let x2 = 'c';

    //Matching range for char datatype.
    match x2 {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }

    let p = Point { x: 0, y: 7 };
    //Destructuring
    let Point { x: a, y: b } = p;

    assert_eq!(0, a);
    assert_eq!(7, b);

    match p {
        Point { x, y: 0 } => println!("On the x axis at {}", x),
        Point { x: 0, y } => println!("On the y axis at {}", y),
        Point { x, y } => println!("On neither axis: ({}, {})", x, y),
    }

    let msg = Message::ChangeColor(Color::Hsv(0, 160, 255));
    let _rgb = Message::ChangeColor(Color::Rgb(0, 160, 255));
    let _quit = Message::Quit;
    let _write = Message::Write("Hello There!".to_string());
    let movy = Message::Move { x: 10, y: 20 };

    match msg {
        Message::ChangeColor(Color::Rgb(r, g, b)) => {
            println!(
                "Change the color to red {}, green {}, and blue {}",
                r,
                g,
                b
            )
        },
        Message::ChangeColor(Color::Hsv(h, s, v)) => {
            println!(
                "Change the color to hue {}, saturation {}, and value {}",
                h,
                s,
                v
            )
        },
        _ => ()
    }

    match movy {
        Message::Move { x, y } => {
            println!("x and y {} {}", x, y);
        },
        _ => ()
    }

    //Complex destructuring
    let ((feet, inches), Point {x, y}) = ((3, 10), Point { x: 3, y: -10 });

    println!("Feet and inches {} {}", feet, inches);
    println!("Points x y {} {}", x, y);

    //Ignore Pattern
    foo(3, 4);

    //More complex ignore patterns
    let mut setting_value = Some(5);
    let new_setting_value = Some(10);
    
    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("Can't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }
    
    println!("setting is {:?}", setting_value);

    //Ignoring values in an array    
    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {}, {}, {}", first, third, fifth)
        },
    }

    //Ignoring multiple values using ..
    match numbers {
        (first, .., last) => {
            println!("Some numbers: {}, {}", first, last);
        },
    }

    let origin = Point2 { x: 0, y: 0, z: 0 };

    match origin {
        Point2 { x, .. } => println!("x is {}", x),
    }

    let num = Some(4);

    //Match guard logic
    match num {
        Some(x) if x < 5 => println!("less than five: {}", x),
        Some(x) => println!("{}", x),
        None => (),
    }

    //@ Binding usage
    let msg = Message2::Hello { id: 5 };

    match msg {
        Message2::Hello { id: id_variable @ 3..=7 } => {
            println!("Found an id in range: {}", id_variable)
        },
        Message2::Hello { id: 10..=12 } => {
            println!("Found an id in another range")
        },
        Message2::Hello { id } => {
            println!("Found some other id: {}", id)
        },
    }
}

fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

struct Point {
    x: i32,
    y: i32,
}

struct Point2 {
    x: i32,
    y: i32,
    z: i32,
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(Color),
}

enum Message2 {
    Hello { id: i32 },
}

enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}