fn main() {
    //This is a warning because the irrefutable pattern doesnt require validation.
    if let x = 5 {
        println!("{}", x);
    };

    //This will not compile since an element is not option of T.
    //let Some(x) = 5;
}
