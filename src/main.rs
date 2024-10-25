use rand::Rng; // Rng trait is for random number generator methods
use std::cmp::Ordering; // enum with Less, Greater, Equal
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Input your guess:");

        // vars are immutable by default, must include mut here
        // String::new() creates new instance of String, provided by standard lib (growable, UTF-8)
        // :: says this is an associated function of the String type
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess) // returns a Result (an enum), can be ok or err
            .expect("Failed to read line"); // if Err, expect will cause a crash and display this message. will return value of Ok on Ok (num of bytes)

        // this is an example of "shadowing," rewriting the guess var as an int type instead of making a new one
        // parse() also returns a Result value (Ok/Err)
        // handle the error with a match instead of crashing with an .expect()
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess); // {} is placeholder that will print value of guess

        // takes a reference as an argument, need to use &
        // match - expression made of arms, kinda like a switch statement
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"), // example of an "arm" - a pattern to match against and run the code if matched
            Ordering::Greater => println!("Too Large"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
