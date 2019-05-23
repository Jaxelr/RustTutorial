fn main() {
    println!("Hello, world!");
    another_function(5, 6);

    let _x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y);    

    let xx = five();

    println!("The value of xx is: {}", xx);

    let yy = plus_one(xx);
    
    println!("The value of yy is: {}", yy);
}

fn another_function(x: i32, y: i32) {
    println!("The value of x is: {}",x);
    println!("The value of x is: {}",y);
    
}

fn five() ->  i32 { 5 }

fn plus_one(x: i32) -> i32 {
    x + 1
}