fn main() {
    //Using the slice type
    let s = String::from("hello world");

    let _hello = &s[0..5];
    let _world = &s[6..11];
    
    let s2 = String::from("hello");

    //If you want the length to start from the beginning, the value is optional.
    let _slice = &s2[..2];
    let _slice2 = &s2[3..];
    let _slice3 = &s2[..];

    let word = first_word(&s);

    println!("The initial word {}", s);
    println!("The word splice by method: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}