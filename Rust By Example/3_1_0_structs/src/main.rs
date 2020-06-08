#[derive(Debug)]
struct Person<'a> {
    // The 'a defines a lifetime
    name: &'a str,
    age: u8,
}

// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// A struct with two fields
struct Point {
    x: f32,
    y: f32,
}

// Structs can be reused as fields of another struct
#[allow(dead_code)]
struct Rectangle {
    // A rectangle can be specified by where the top left and bottom right
    // corners are in space.
    top_left: Point,
    bottom_right: Point,
}

fn rect_area(rectangle: &Rectangle) -> f32 {
    let Rectangle 
    {
        top_left: Point { x: tlx, y: tly },
        bottom_right: Point { x: brx, y: bry}
    } = rectangle;

     (brx - tlx) * (tly - bry)
}

fn square(point: Point, length: f32) -> Rectangle {

    Rectangle 
    {
        top_left: Point {x: point.x, y: length },
        bottom_right: Point {x: length, y: point.y }
    }
}

fn main() {
    // Create struct with field init shorthand
    let name = "Peter";
    let age = 27;
    let peter = Person { name, age };

   // Print debug struct
   println!("{:?}", peter);

    // Instantiate a `Point`
    let point: Point = Point { x: 10.3, y: 0.4 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);
    
    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y` will be the same as `point.y` because we used that field
    // from `point`
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let Point { x: top_edge, y: left_edge } = point;

    let rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Instantiate a tuple struct
    let pair = Pair(1, 0.1);

    // Access the fields of a tuple struct
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // Destructure a tuple struct
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);

    println!("rectangle area is {}", rect_area(&rectangle));

    let custom_point = Point { x: 10.0, y: 30.0};
    let sqr = square(custom_point, 20.0);

    println!("Squaring: Point1: {} {}, Point2: {} {}", sqr.top_left.x, sqr.top_left.y, sqr.bottom_right.x, sqr.bottom_right.y);
}
