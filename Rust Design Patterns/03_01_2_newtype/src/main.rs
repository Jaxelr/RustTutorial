struct Foo {
    field1: String,
    opaque_field1: String
}

impl Foo {
    fn print_secret(self) -> () {
        println!("Print field is {}", self.field1);
        println!("Secret is {}", self.opaque_field1);
    }
}

// The newtype.
pub struct Bar(Foo);

impl Bar {
    fn print_without_secret(self) -> () {
        println!("printing fields: {}", self.0.field1);
        println!("printing secret: {}", "********");
    }
}

fn main() {
    let foo = Foo { field1: String::from("Abcdef"), opaque_field1: String::from("secret secret")};

    foo.print_secret();
    
    let bar = Bar(Foo { field1: String::from("Abcdef"), opaque_field1: String::from("secret secret")});

    bar.print_without_secret();
}