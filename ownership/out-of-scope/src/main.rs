fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    //println!("{}", s1) //generates out of scope error due to s1 being dropped from memory when assigned to s2
    println!("{}", s2)
}
