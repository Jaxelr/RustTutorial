#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Person {
    name: String,
    age: u32,
}

impl Person {
    pub fn new(name: String, age: u32) -> Self {
        Person { name, age }
    }
}

fn main() {
    //Sort vector of integers
    let mut vec = vec![1, 5, 10, 2, 15];
    println!("Before {:?}", vec);
    vec.sort();
    println!("After {:?}", vec);
    assert_eq!(vec, vec![1, 2, 5, 10, 15]);

    //Sort vector of floats
    let mut vec = vec![1.1, 1.15, 5.5, 1.123, 2.0];
    println!("Before {:?}", vec);
    vec.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("After {:?}", vec);
    assert_eq!(vec, vec![1.1, 1.123, 1.15, 2.0, 5.5]);

    //Sort vector of structs
    let mut people = vec![
        Person::new("Zoe".to_string(), 25),
        Person::new("Al".to_string(), 60),
        Person::new("John".to_string(), 1),
    ];
    println!("Before {:?}", people);
    // Sort people by derived natural order (Name and age)
    people.sort();

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("John".to_string(), 1),
            Person::new("Zoe".to_string(), 25),
        ]
    );

    println!("After name and age {:?}", people);

    // Sort people by age
    people.sort_by(|a, b| b.age.cmp(&a.age));

    assert_eq!(
        people,
        vec![
            Person::new("Al".to_string(), 60),
            Person::new("Zoe".to_string(), 25),
            Person::new("John".to_string(), 1),
        ]
    );

    println!("After age {:?}", people);
}
