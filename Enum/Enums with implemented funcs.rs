#[derive(Debug)]
enum IpAddr {
    V4(String),
    V6(String),
}

impl IpAddr {
    fn print_self(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let home = IpAddr::V4(String::from("127.0.1.15"));

    let loopback = IpAddr::V6(String::from("::1"));

    home.print_self();
    loopback.print_self();
}