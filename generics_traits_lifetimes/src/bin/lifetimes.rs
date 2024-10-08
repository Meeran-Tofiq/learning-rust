use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str, 
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }

    fn announce_and_return_part(&self, announcement: &str, x: &str) -> &str {
        println!("{announcement}");
        self.part
    }
}

fn main() {
    let string1: String = String::from("abcd");
    let string2: &str = "xyz";

    let mut result = longest(string1.as_str(), string2);
    println!("The longest string slice is {result}");

    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {result}");
    }

    // println!("{result}"); this line produces an error
}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x
    }

    y
}

fn longest_with_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where 
    T: Display
{
    println!("{ann}");
    
    if x.len() > y.len() {
        return x;
    }
    
    y
}