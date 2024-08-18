use std::collections::HashMap;

fn main() {
    let mut x: Vec<i32> = Vec::new();
    x.push(1);
    x.push(3);
    x.push(5);

    println!("The number is {}", x[2]);

    x.push(99);

    match x.get(3) {
        Some(val) => println!("The fourth number is {}", val),
        None => println!("There is no number at the fourth spot.") 
    }

    for i in &x {
        print!("{}\t", i);
    }

    println!("Vector is {:?}", x);

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {s2}");

    let t1: String = String::from("tic");
    let t2: String = String::from("tac");
    let t3: String = String::from("toe");
    let t = format!("{}-{}-{}", t1, t2, t3);
    println!("{t}");

    let mut map: HashMap<String, i32> = HashMap::new();
    map.insert("Banana".to_string(), 99);
    map.insert("Apples".to_string(), 73);
    map.insert("Oranges".to_string(), 112);
    println!("{:?}", map);
    let banana_sales = map.get("Banana").copied().unwrap_or(-1);
    let melon_sales = map.get("Melon").copied().unwrap_or(-1);
    println!("{}", banana_sales);
    println!("{}", melon_sales);
    let j = map.entry("Strawberry".to_string()).or_insert(999).clone();
    println!("{:?}", map);
    println!("Stawberries is {}", j);
}
