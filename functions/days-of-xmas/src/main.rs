fn main() {
    let gifts = ["partridge in a pear tree", "doves", "pied pipers", "milkers milking", "golden rings", "chickens", "drummers drumming", "sports cars", "hamburgers", "xboxes", "poopers pooping", "hackers hacking"]; //tbc
    for day in 1..13{
        if day == 1 {
            println!("On the {}st day of christmas my true love gave to me {} {}", day, day, gifts[day-1]);
        } else if day == 2 {
            println!("On the {}nd day of christmas my true love gave to me {} {}", day, day, gifts[day-1]);
        } else if day ==3 {
            println!("On the {}rd day of christmas my true love gave to me {} {}", day, day, gifts[day-1]);
        } else {
            println!("On the {}th day of christmas my true love gave to me {} {}", day, day, gifts[day-1]);
        }       
    }
}

