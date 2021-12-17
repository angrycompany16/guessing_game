// io - input/output library
use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    // gen_range() inclusive on upper bound but exclusive on lower bound (aka 1 to 101)
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        // creates an empty string
        let mut guess = String::new();
    
        // makes an input handle, calls read_line() function and calls .expect() on return value of read_line().
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // using match on the guess value. Checking if match.guess.trim().parse() enum is Ok or Err.
        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
    
        // use match to compare the Result enum from our guess to the secret number if it matches arm 1, do what arm 1 says, etc.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }       
        }
    }
}
