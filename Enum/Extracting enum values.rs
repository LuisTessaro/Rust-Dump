#[derive(Debug)]
enum Coin {
    Penny(u8),
    Nickel(u8),
    Dime(u8),
    Quarter(u8),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny(amount) => 1 * amount,
        Coin::Nickel(amount) => 5 * amount,
        Coin::Dime(amount) => 10 * amount,
        Coin::Quarter(amount) => 25 * amount,
    }
}

fn main() {
    let coin = Coin::Quarter(6);

    println!("{}", value_in_cents(coin));
}
