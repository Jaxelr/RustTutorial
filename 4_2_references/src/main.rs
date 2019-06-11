fn main() 
{
    //We can cheat with modifying scopes depending on the {}
    let mut s = String::from("yeppers");
    {
        let _r1 = &mut s;
    } // r1 goes out of scope here, so we can make a new reference with no problems.

    let r2 = &mut s;

    println!("this is allowed to be reference here: {}", r2);

    //More borrowing
    let mut s = String::from("no prob");

    let r3 = &s; // no problem
    let r4 = &s; // no problem
    println!("immutables: {} and {}", r3, r4);
    // r1 and r2 are no longer used after this point

    let r5 = &mut s; // no problem
    println!("After 2 immutable references,  i can still reference a mutable here: {}", r5);
}
