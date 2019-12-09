enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn main() {
    let my_coin = Coin::Penny;
    let mut other_coin = Coin::Dime;

    let cents = value_in_cents(&my_coin);
    let cents2 = value_in_cents(&other_coin);

    println!("My coin in cents {}", cents);
    println!("My coin in cents2 {}", cents2);

    other_coin = Coin::Nickel;
    let cents2 = value_in_cents(&other_coin);

    println!("My coin in cents2 {}", cents2);

    other_coin = Coin::Quarter;
    let cents2 = value_in_cents(&other_coin);

    println!("My coin in cents2 {}", cents2);

    
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);

    //using the placeholder of _
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }   

    println!("I am 0 {}", some_u8_value);
}

fn value_in_cents(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        },  
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}