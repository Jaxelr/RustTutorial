fn main() {
    foo::r#try();
    foo::r#catch();
}

mod foo {
    // Even though try is a reserved word in rust 2018, 
    // we can use the try keyword by using the 
    // special r# clause.
    pub fn r#try() -> () {
        println!("Trying...");
    }

    pub fn r#catch() -> () {
        println!("...Catching!!");
    }
}