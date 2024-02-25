//fn main() { // if statement
//    let number = 9;
//
//    if number < 5 {
//        println!("The condition was met!")
//    } else {
//        println!("The condition was not met :(")
//   }
//}
//fn main() { //error with bool
 //   let number = 0;
//
  //  if number != 0 {
    //    println!("The number is not zero")
    //}; 
//}
//fn main() { //handling multiple conditions
//    let number = 6;
//
  //  if number % 4 == 0 {
    //    println!("The number is divisible by 4");
    //} else if number % 3 == 0 {
    //    println!("The Number is divisible by 3");
    //} else if number % 2 == 0 { 
    //    println!("The number is divisible by 2");
    //} else {
    //    println!("The number is not divisible by 4, 3, 2, 1");
    //}
//}
fn main() { //using an if statement in a let statement
    let condition = false;

    let number = if condition {
        5
    } else {
        6
    };
    println!("The value of the number is {}", number)
}