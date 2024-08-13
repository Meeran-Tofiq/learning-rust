#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String)
}

fn main() {
    let home_ip = IpAddrKind::V4("127.0.0.1".to_string());
    let office_ip = IpAddrKind::V6("::1".to_string());

    println!("{:?}", home_ip);
}
