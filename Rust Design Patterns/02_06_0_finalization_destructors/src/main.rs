struct Foo;

impl Drop for Foo {
    fn drop(&mut self) {
        println!("Goodbye, world!")
    }
}

fn main() {
    println!("Hello, world!");
    let _foo = Foo;
}
