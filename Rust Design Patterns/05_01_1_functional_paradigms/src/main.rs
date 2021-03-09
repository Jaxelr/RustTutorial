fn main() {
    println!("Equivalent!");

    //Imperative
    let mut sum = 0;
    for i in 1..11 {
        sum += i;
    }
    println!("{}", sum);

    //Functional
    println!("{}", (1..11).fold(0, |a, b| a + b));
}
