#[derive(Debug)]
struct User {
    name: String,
    email: String,
    login_count: u64,
    active: bool,
}

fn main() {
    let mut luis = create_user(
        String::from("Luís Tessaro"),
        String::from("lh.pedrosot@hotmail.com"),
    );
    luis.login_count = login(&luis);
    luis.login_count = login(&luis);
    luis.login_count = login(&luis);
    luis.login_count = login(&luis);
    luis.login_count = login(&luis);

    print_user(&luis);
}

fn create_user(name: String, email: String) -> User {
    return User {
        name,
        email,
        login_count: 0,
        active: false,
    };
}

fn login(user: &User) -> u64 {
    println!("User {} Loged in", user.name);
    return user.login_count + 1;
}

// {:#?} can be used to print the object formatted
fn print_user(user: &User) {
    println!(
        "{:?}",
        user
    );
}
