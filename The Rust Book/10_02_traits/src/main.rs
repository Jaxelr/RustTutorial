fn main() {

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from("The Pittsburgh Penguins once again are the best
        hockey team in the NHL."),
    };

    println!("New article available! {}", article.summarize());   
    
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)") //We can optionally define default behavior
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

//This is how we define default behavior describe on the Trait
//impl Summary for NewsArticle {}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

//Trait as parameter
pub fn notify(item: impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

//Trait bound syntax
pub fn notify2<T: Summary>(item: T) {
    println!("Breaking news! {}", item.summarize());
}

//Trait bounds with the where clause
pub fn notify3<T>(item: T) 
where T: Summary
{
    println!("Breaking news! {}", item.summarize());
}

//Returns a struct that implements Summary, in this case a Tweet
pub fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("of course, as you probably already know, people"),
        reply: false,
        retweet: false,
    }
}

//Fixing the previous example on an elegant version
fn largest<T> (list:  &[T]) -> T 
where T: PartialOrd + Copy {

    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}