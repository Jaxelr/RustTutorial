enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    //This logic doesnt work, because we cant share ownership of the Box<List>
    // let a = Cons(5,
    //     Box::new(Cons(10,
    //         Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a)); //This clone just ads a pointer counter to the same value
    println!("count after creating b = {}", Rc::strong_count(&a));
    let _c = Cons(4, Rc::clone(&a)); //It does not copy the value in memory
    println!("count after creating c = {}", Rc::strong_count(&a));
    {
        let _d = Cons(6, Rc::clone(&a));
        println!("count after creating d = {}", Rc::strong_count(&a));
    }
    println!("count after d goes out of scope = {}", Rc::strong_count(&a));
}
