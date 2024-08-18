use std::collections::HashMap;

fn main(){
    let arr = [1, 1, 2, 2, 3, 4, 5, 6, 6, 6, 6, 6, 7, 3, 4, 3, 12, 13];
    let mut vec: Vec<i32> = Vec::new();
    let mut number_count: HashMap<i32, i32> = HashMap::new();
    for num in arr {
        vec.push(num);
        let count = number_count.entry(num).or_insert(0);
        *count += 1;
    }
    vec.sort();
    let mut key_for_mode: i32 = 0;
    let mut mode: i32 = 0;
    for (k, v) in &number_count {
        if *v > mode {
            mode = *v;
            key_for_mode = *k;
        }
    }

    println!("The median is {}", vec.get(vec.len() / 2).expect("Whoops"));
    println!("The mode is {}", key_for_mode);
}