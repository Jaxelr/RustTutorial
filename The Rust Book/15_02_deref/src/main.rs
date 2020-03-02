use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

//Trait of Dereference for smart pointers
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    //Store on stack aka normal usage 
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5,*y);

    //Store on heap, but behaves the same way
    let x2 = 5;
    let y2 = Box::new(x2);

    assert_eq!(5, x2);
    assert_eq!(5,*y2);

    //Store on heap, but behaves the same way
    let x3 = 5;
    let y3 = MyBox::new(x3);

    assert_eq!(5, x3);
    assert_eq!(5,*y3);    

    let m = MyBox::new(String::from("Rust"));

    //Thanks to deref coercion we dont have to do an explicit deref with *
    hello(&m);

    //Like this
    hello(&(*m)[..]);
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}