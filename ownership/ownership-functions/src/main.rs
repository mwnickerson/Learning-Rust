fn main() {
    let s = String::from("Hello");

    take_ownership(s);

    let x = 5;

    make_copy(x)
}

fn take_ownership(some_string: String) {
    println!("{}", some_string);
}
fn make_copy(some_integer: i32) {
    println!("{}", some_integer)
}
