fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        println!("counter {}", counter);

        if counter == 10 {
            println!("breaking {}", counter);
            break counter * 2;
        }
    };


    assert_eq!(result, 20);
}
