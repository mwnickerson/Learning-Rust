//fn main() { 
    //repeating loop
    //loop {
        //println!("Again!")
    //}   
//}
//fn main() {
    //returning values from loops
    //let mut counter = 0;
    //let result = loop {
        //counter += 1;
        //if counter == 10 {
            //break counter * 2;
        //}
    //};
    //println!("The counter is {}", result)
//}
//fn main() {
    //conditional loops with while
    //let mut number = 3;
    //while number != 0 {
        //println!("{}!",number);
        //number = number -1
    //}
    //println!("LIFTOFF!")
//}
//fn main() {
    //Looping through a collection 
    //let a = [10, 20, 30, 40, 50];
    //let mut index = 0;
    //while index < 5 {
        //println!("{}", a[index]);
        //index = index + 1
    //}
    //for element in a.iter() {
        //println!("The value is: {}", element)
    //}
//}
fn main() {
    //consise liftoff
    //let a = [5, 4, 3, 2, 1];
    //for element in a.iter() {
    //    println!("{}!", element)
    //};
    //println!("LIFTOFF!")
    for i in (1..4).rev() {
        println!("{}!", i)
    }
    println!("LIFTOFF!")
}