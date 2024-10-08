use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The larger number is {}", self.x);
            return;
        }
        
        println!("The larger number is {}", self.y)
    }
}

pub trait Summary {
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.format_author())
    }

    fn format_author(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{} {}: {}", self.location, self.author, self.headline)
    }

    fn format_author(&self) -> String {
        format!("{}", self.author)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!{"{}: {}", self.username, self.content}
    }

    fn format_author(&self) -> String {
        format!("@{}", self.username)
    }
}

struct Point<T> {
    x: T,
    y: T
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {result}");

    let pair: Pair<i32> = Pair::new(17, 35);
    pair.cmp_display();
}
