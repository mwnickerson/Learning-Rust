use std::io; // import the io library from std
use std::cmp::Ordering; // import the Ordering library from std::cmp
use rand::Rng; // import the rand library 


fn main() {
    println!("Guess the number!");

    // println!("Please input your guess.");

    let secret_number = rand::thread_rng().gen_range(1, 101); //generate the random number and store as immutable variable

    // println!("The secret number is: {}", secret_number); // print the secret number

    loop { // move everything into the loop so the user can keep guessing
        println!("Please input your guess.");

        let mut guess = String::new(); // store the guess in a mutable variable
    
        io::stdin().read_line(&mut guess) //read line from standard input & store as guess variable
            .expect("Failed to read line"); //handle error if no user input

        let guess: u32 = match guess.trim().parse() {// convert the guess to an unsigned 32 bit integer and store it as a shadow to the original guess
            //.expect("Please type a number!"); // handle error if user inputs something other than a number
            Ok(num) => num, // if the user inputs a number, return the number
            Err(_) => continue, // if the user inputs something other than a number, continue the loop to allow for more guesses
    };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) { //compare the guess and the secret number using the cmp from ordering library
            Ordering::Less => println!("Too small!"), 
            Ordering::Greater => println!("Too big!"), 
            Ordering::Equal => {
                println!("You win!");
                break; // break out of the loop if the user guesses correctly
            }
    }

    }

}
