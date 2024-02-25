//fn main() {
//    let s = String::from("hello");
//    println!("{}", s)
//}
fn main() {
    let mut s = String::from("hello");
    s.push_str(" world");
    println!("{}", s)
}