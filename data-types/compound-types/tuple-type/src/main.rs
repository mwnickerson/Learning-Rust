fn main() {
    //let tup: (i32, f64, u8) = (500, 6.4, 1);
    let tup = (500, 6.4, 1);
    let (x, y, z) = tup;
    println!("The value of y = {}", y);
    println!("The value of x = {}", tup.0);
    println!("The value of z = {}", tup.2);
}