//fn main_old() {
    //let y = 6; // good statement
    //let x = (let y = 6); // bad statement
//    let x = 5;
//    let y = { 
 //       let x = 3;
//        x + 1
//    };
//   println!("the value of  y  is: {}", y)
//}

//fn five() -> i32 {
 //   5
//}
//fn main_old2() {
//    let x = five();
//    println!("The value of x is: {}", x)
//}

fn main() {
    let x = plus_one(5);
    println!("The valuen of x is: {}", x)
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}