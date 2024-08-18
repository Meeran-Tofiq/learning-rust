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
}
