fn main() {
    let turing = Some("Turing");
    let logicians = vec!["Curry", "Kleene", "Markov"];
    
    let processed = extend_vec(logicians, turing);
    
    println!("{:?}", processed);

    let logicians = vec!["Kleene", "Curry", "Markov"];

    let processed = equivalent_vec(logicians, turing);

    println!("{:?}", processed);

    let logicians = vec!["Kleene", "Curry", "Markov"];

    for logician in logicians.iter().chain(turing.iter()) {
        println!("{:?}", logician);
    }
}


fn extend_vec<'a>(mut list: Vec<&'a str>, element : Option<&'a str>) -> Vec<&'a str> {
    
    list.extend(element);

    list
} 

fn equivalent_vec<'a>(mut list: Vec<&'a str>, element : Option<&'a str>) -> Vec<&'a str> {

    if let Some(element_inner) = element {
        list.push(element_inner);
    }

    list
}