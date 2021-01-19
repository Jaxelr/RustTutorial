use std::ops::Deref;

//Taken from the docs
struct DerefExample<T> {
    value: T
}

//Implement
impl<T> Deref for DerefExample<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

fn main() {
    let x = DerefExample { value: 'a' };
    assert_eq!('a', *x);

    println!("{}", x.deref());
}
