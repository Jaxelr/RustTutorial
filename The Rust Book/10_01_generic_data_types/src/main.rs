fn main() {
    let integer_struct = Point { x: 5, y: 10};
    let float_struct = Point { x: 1.0, y: 2.0};
    let mixed_struct = Point2 { x: 10, y: 1.0};
    let second_mixed_struct = Point2 { x: "Hellow", y: 'c'};


    println!("integer x {}", integer_struct.x());
    println!("integer y {}", integer_struct.y());
    
    println!("float x {}", float_struct.x());
    println!("float y {}", float_struct.y());
    println!("distance {}", float_struct.distance_from_origin());

    let result = mixed_struct.mixup(second_mixed_struct);

    println!("result of x {} and y {}", result.x, result.y);
}

//Generic struc
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl<T> Point<T> {
    fn y(&self) -> &T {
        &self.y
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

