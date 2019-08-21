fn main() {
    let mut s = String::new();

    s = "initial contents".to_string(); //Verbosy way
    let _s2 = String::from("initial contents"); //succint way

    //Strings are UTF8
    s.push_str(" and bar");

    println!("s is {}", s);

    let s3 = "and bar";

    s.push_str(s3);

    println!("s3 is {}", s3);


    //Concatenating

    let s4 = String::from("Hello, ");
    let s5 = String::from("world!");
    let s6 = s4 + &s5; // note s4 has been moved here and can no longer be used
    
    println!("s6 is {}", s6);

    //Avoid doing concatenation, instead use format
    let s7 = String::from("tic");
    let s8 = String::from("tac");
    let s9 = String::from("toe");

    let s10 = format!("{}-{}-{}", s7, s8, s9);

    println!("s10 is {}", s10);

    //Strings are Vec<u8>
    let len = String::from("Hola").len();

    println!("length of Hola is: {}", len);

    //Indexing is not allowed on rust, but splicing is
    let my_phrase = "This is a long phrase that i have";

    let word = &my_phrase[0..4];

    println!("My phrase is {}", my_phrase);
    println!("word is {}", word);

    //Strings are hard (on UTF8 to work globally)
}
