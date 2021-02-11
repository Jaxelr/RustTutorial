mod a {
    // Public struct.
    #[derive(Debug)]
    pub struct S {
        pub foo: i32,
        // Private field.
        bar: i32,
    }
}

fn main() {
    let s: a::S;

    // Because S::bar is private, it cannot be named here and we must use `..`
    // in the pattern.
    let a::S { foo: _, ..} = s;
}
