#[derive(Debug)]
struct User {
    first_name: String,
    last_name: String,
    age: i8,
    active: bool,
    email: String
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn create_user(first_name: String, last_name: String, age: i8, email: String) -> User {
    User {
        first_name,
        last_name,
        age,
        active: true,
        email
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct Vector(Point, Point);

fn main() {
    let user_a = create_user(
        String::from("Meran"), 
        String::from("Tofiq"), 
        22, 
        "meran.saman2310@gmail.com".to_string());

    let user_b = User {
        email: "noway@gmail.com".to_string(),
        ..user_a
    };

    let red = Color(255, 0, 0);
    let origin = Point(0, 0, 0);
    let horizon = Point(255, 255, 255);
    let line = Vector(origin, horizon);

    println!("{}", user_b.first_name);
    println!("{}", line.0.0);

    println!("{user_b:#?}");
    dbg!(&user_b);

    println!("{}", user_b.first_name);

    let rect1 = Rectangle {
        width: 35,
        height: 45
    };

    print_rect_area(&rect1);
}

fn print_rect_area(rect: &Rectangle) {
    println!("The area of {rect:?} is {}", rect.area());
}
