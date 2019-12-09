use std::collections::HashMap;

fn main() {
    //Declaring Hashmaps
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    //Using Vectors as input
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    println!("Second declaration: {:?}", scores2);

    //Borrowed types are passed to the hash
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    //Inaccessible
    //println!("field_name should be invalid here {}", field_name);

    //Updating an existing value
    let mut scores3 = HashMap::new();

    scores3.insert(String::from("Blue"), 10);
    scores3.insert(String::from("Blue"), 25); //Updated the Blue key

    println!("{:?}", scores3); //Will print 25 on the Blue key

    //Inserting only if it doesnt exist
    let mut scores4 = HashMap::new();
    scores4.insert(String::from("Blue"), 10);

    scores4.entry(String::from("Yellow")).or_insert(50); //Since Yellow doesnt exist, insert it
    scores4.entry(String::from("Blue")).or_insert(50); //Since Blue exists, we dont update the value

    println!("{:?}", scores4);

    //Updating based on  existing value
    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); //We check the key, if it doesnt exist insert it
        *count += 1;                              //we sum 1 after
    }

    println!("{:?}", map);
}
