// Similarly `mod inaccessible` and `mod nested` will locate the `nested.rs`
// and `inaccessible.rs` files and insert them here under their respective
// modules
mod inaccessible;
pub mod nested;

pub fn function() {
    println!("called `my::function()`");
}

fn private_function() {
    println!("called `my::private_function()`");
}

pub fn indirect_access() {
    print!("called `my::indirect_access()`, that > ");

    private_function();
}

//Added this to arrived to the inaccesible file, cannot called directly from the parent
pub fn called_inaccessible()
{
    inaccessible::public_function();

}