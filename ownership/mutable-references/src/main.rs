fn main() {
    let mut s = String::from("Hello");
    println!("{}", s);
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: & mut String) {
    some_string.push_str(", world!")
}