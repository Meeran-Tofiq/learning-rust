use std::ops::RangeInclusive;
use rand::Rng;

fn main() {
    let tup: (i32, i32, i64) = (9119, 99, 123456789098765);
    let (x, y, z): (i32, i32, i64) = tup;
    println!("HELLO: {}", tup.2);
    println!("HELLO: {x}");
    println!("HELLO: {y}");
    println!("HELLO: {z}");

    let arr: [i32; 5] = [3; 5];
    println!("HELLO: {}", arr[0]);

    let abcd: i32 = random_number_generator(1..=100);
    println!("{} is your value", abcd);

    if abcd < 50 {
        println!("NO WAAAAAY it's a 50/50")
    } else {
        println!("WHAAAAAAT that's a 50/50");
    }

    let str: String = weird_function().to_string();
    println!("{}", str);

    println!("The value of the loop function is {}", loop_function());

    for num in arr {
        println!("The number in the array is {num}");
    }

    space_liftoff();
}

fn random_number_generator(range: RangeInclusive<i32>) -> i32 {
    let random_number: i32 = rand::thread_rng().gen_range(range);
    random_number
}

fn weird_function() -> i32 {
    if 1 == 1 { return 22 }
    33
}

fn loop_function() -> i32 {
    let mut counter: i32 = 0;
    
    let result: i32 = 'coutner_loop: loop {
        counter += 1;

        if counter > 9 {
            break 'coutner_loop counter * 2;
        }
    };

    return result;
}

fn space_liftoff() -> () {
    for number in (1 ..= 3).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
