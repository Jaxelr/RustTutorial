#![allow(unused_variables)]
#![allow(unused_must_use)]
#![allow(dead_code)]

use std::rc::Rc;

fn main() {

    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);
    let closure = {
        // `num1` is moved
        let num2 = num2.clone();  // `num2` is cloned
        let num3 = num3.as_ref();  // `num3` is borrowed
        move || {
            *num1 + *num2 + *num3;
        }
    };
}

//Instead of
fn instead_of_sample()  {
    let num1 = Rc::new(1);
    let num2 = Rc::new(2);
    let num3 = Rc::new(3);
    
    let num2_cloned = num2.clone();
    let num3_borrowed = num3.as_ref();
    let closure = move || {
        *num1 + *num2_cloned + *num3_borrowed;
    };
}