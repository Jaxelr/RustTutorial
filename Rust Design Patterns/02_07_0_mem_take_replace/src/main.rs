use std::mem;

#[derive(Debug)]
enum MyEnum {
    A { name: String, x: u8 },
    B { name: String }
}

#[derive(Debug)]
enum MultiVariateEnum {
    A { name: String },
    B { name: String },
    C,
    D
}

fn a_to_b(e: &mut MyEnum) {

    // we mutably borrow `e` here. This precludes us from changing it directly
    // as in `*e = ...`, because the borrow checker won't allow it. Therefore
    // the assignment to `e` must be outside the `if let` clause.
    *e = if let MyEnum::A { ref mut name, x: 0 } = *e {

        // this takes out our `name` and put in an empty String instead
        // (note that empty strings don't allocate).
        // Then, construct the new enum variant (which will
        // be assigned to `*e`, because it is the result of the `if let` expression).
        MyEnum::B { name: mem::take(name) }

    // In all other cases, we return immediately, thus skipping the assignment
    } else { return }
}

fn swizzle(e: &mut MultiVariateEnum) {
    use MultiVariateEnum::*;
    *e = match *e {
        // Ownership rules do not allow taking `name` by value, but we cannot
        // take the value out of a mutable reference, unless we replace it:
        A { ref mut name } => B { name: mem::take(name) },
        B { ref mut name } => A { name: mem::take(name) },
        C => D,
        D => C
    }
}

fn main() {

    let mut my_enum = MyEnum::A { name: String::from("abc"), x: 0};
    println!("{:?}",my_enum);

    a_to_b(&mut my_enum);
    println!("{:?}",my_enum);

    //Multivariateenum
    let mut my_multi = MultiVariateEnum::A { name: String::from("def")};

    swizzle(&mut my_multi);

    println!("{:?}", my_multi);
}
