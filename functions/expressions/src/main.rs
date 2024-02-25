fn main() {
    let x =5; 
    let y = {
        let x = 3;
        x + 1
    };
    println!("The value of x = {}", x);
    println!("The value of y = {}", y);
}
