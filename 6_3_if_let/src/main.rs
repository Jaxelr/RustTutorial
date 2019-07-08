#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Arkansas,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn main() {

    //These 2 situations are the same.
    let some_u8_value = Some(0u8);
    match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
    }
        
    //More concise.
    if let Some(3) = some_u8_value {
        println!("three");
    }

    //Another example
    let my_coin = Coin::Penny;

    let mut count = 0;
    match my_coin {
        Coin::Quarter(state) => println!("State quarter from {:?}!", state),
        _ => count += 1,
    }

    println!("count : {}", count);

    let my_second_coin = Coin::Quarter(UsState::Arkansas);

    let mut count = 0;
    if let Coin::Quarter(state) = my_second_coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }

    println!("count : {}", count);
}