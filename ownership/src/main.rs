fn main() {
    let mut s: String = String::from("Hello");
    s += " World";

    {
        let m: String = String::from("NO WAY");
        s = m;
        // println!("{m}"); // Cannot use this m because it's invalid now
    }

    takes_onwership(s);
    // println!("{s}"); // Cannot use this because s's value has changed owners

    let x: i32 = 23;
    makes_copy(x);
    println!("Number is {x}");

    let str: String = String::from("Hola, soy Dora!");
    let len = calculate_length(&str);

    println!("Str is {str}, and length is {len}")
}

fn takes_onwership(some_string: String) {
    println!("String is {some_string}");
}

fn makes_copy(num: i32) {
    println!("Number is {num}");
}

fn calculate_length(str: &String) -> usize {
    str.len()
}