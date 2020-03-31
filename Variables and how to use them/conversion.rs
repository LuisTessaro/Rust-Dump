use std::any::type_name;

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
}

fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    println!("{}: {}", guess, type_of(&guess));

    let guess: String = 42.to_string();
    println!("{}: {}", guess, type_of(&guess));
}
