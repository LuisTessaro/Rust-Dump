#[derive(Debug)]
struct User {
    name: String,
    email: String,
    login_count: u64,
    active: bool,
}

impl User {
    fn create_user(name: String, email: String) -> User {
        return User {
            name,
            email,
            login_count: 0,
            active: false,
        };
    }

    fn login(&mut self) {
        println!("User {} Loged in", self.name);
        self.login_count = self.login_count + 1;
    }

    fn print_user(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let mut luis = User::create_user(
        String::from("Luís Tessaro"),
        String::from("lh.pedrosot@hotmail.com"),
    );

    luis.login();
    luis.login();
    luis.login();
    luis.login();

    luis.print_user();
}


