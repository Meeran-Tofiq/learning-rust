#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String)
}

#[derive(Debug)]
enum UsState {
    Alaska,
    Arkansas
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState)
}

fn main() {
    let home_ip = IpAddrKind::V4("127.0.0.1".to_string());
    let office_ip = IpAddrKind::V6("::1".to_string());
    let coin: Coin = Coin::Quarter(UsState::Alaska);

    println!("{:?}", home_ip);
    filter_coin(&coin);
}

fn filter_coin(coin: &Coin) -> i32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("The state of the coin is {state:?}");
            25
        }
    }
}
