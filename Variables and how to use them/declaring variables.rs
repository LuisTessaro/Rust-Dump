fn main() {
    //mut keyword allows mutability (if we hade let x = 5; and tried to x = 10; the program would not compile)
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    //asigning a constant variable: 
    //must have a type assigned and by naming convention it should be capitalized
    const Y: i32 = 15;
    
    println!("The value of y is: {}", Y);
}
