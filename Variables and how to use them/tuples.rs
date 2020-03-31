fn main() {
    //tuple with infered types
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("x:{}, y:{}, z:{}", x, y, z);

    //tuple with explicit types
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup;

    println!("x:{}, y:{}, z:{}", x, y, z);
}
