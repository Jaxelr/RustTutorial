//Lets reduce duplication by extracting the logic from the find largest sample to a fn
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item
        }
    }

    largest //We return the largest
}

fn main() {
    let number_list = vec![35, 50, 25, 100];

    let result = largest(&number_list);
    println!("The largest number from collection 1 is {}", result);

    let number_list = vec![28, 43, 67, 01];

    let result = largest(&number_list);
    println!("The largest number from collection 2 is {}", result);
}
