struct User {
    first_name: String,
    last_name: String,
    age: i8,
    active: bool,
    email: String
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

    println!("{}", user_b.first_name);
}
