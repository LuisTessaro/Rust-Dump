fn main() {
    //shadowing: reasigns the variable to what we need
    let x = 5;

    let x = x + 1;

    let x = x * 2;

    println!("The value of x is: {}", x);

    //can be used to completely change the type of the variable
    
    //will not work
    let mut spaces = "   ";
    spaces = spaces.len();

    //works
    let spaces = "   ";
    let spaces = spaces.len();
}
