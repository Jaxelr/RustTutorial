struct User {
    username: String,
    email: String,
    sign_in_count:  u64,
    active: bool,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 3,
    };

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    let user3 = build_user(String::from("third@example.com"), String::from("functionuser123"));

    println!("This is my user1 data: {} {} {} {}", user1.username, user1.email, user1.sign_in_count, user1.active);
    println!("This is my user2 data: {} {} {} {}", user2.username, user2.email, user2.sign_in_count, user2.active);
    println!("This is my user3 data: {} {} {} {}", user3.username, user3.email, user3.sign_in_count, user3.active);
}

//This is a shorthand notation of the struct creation.
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}