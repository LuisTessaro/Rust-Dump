fn main() {
    println!("sum: {}", sum(5, 5));
    print_this("hey!");
    println!(
        "{}: size {}",
        trim_string("               hey!               "),
        trim_string("               hey!               ").len()
    );
}

// name (var: type) -> return type
fn sum(x: i32, y: i32) -> i32 {
    return x + y;
}

fn print_this(text: &str) {
    println!("{}", text);
}

fn trim_string(text: &str) -> &str {
    return text.trim();
}
