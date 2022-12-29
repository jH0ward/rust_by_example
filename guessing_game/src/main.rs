use rand::Rng;
use std::cmp::Ordering;
use std::io;

// we added rand = "0.8.5" to the deps section of the toml
//  and then ran cargo build to get access to rand crate

// Ordering type has variants of Less Equal or Greater

fn main() {
    println!("You're playing... Guess the number!\n");

    // Generate secret number (equal sign means inclusive)
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Shh the secret number is {secret_number}");

    // we can use cargo doc --open to build the docs for this project and open in browser

    println!("Please input a guess (integer between 0 and 100 inclusive)");

    loop {
        // Initialize a new mutable string for the guess
        let mut guess = String::new();
        // I tried to leave the string initialization outside the loop. However,
        //  read_line() will append to the string so we end up with the first two
        //  guesses concatted with a space in between so the parse fails

        // stdin() returns a Stdin instance, representing a handle to stdin from terminal
        io::stdin()
            // &mut guess means we pass a reference to the mutable string guess
            .read_line(&mut guess)
            // read_line() returns a Result (enum) which can be Ok or Err
            // the expect() method of Result will raise if Result is Err
            // If not err, expect will take the return value that Ok is holding.
            // In our case it's the number of bytes in the user input
            .expect("Failed to read line");

        // we need to coerce this guess into an integer to compare
        let parse_result = guess.trim().parse::<u32>();
        //.expect("Failed to parse input guess to integer");

        let guess = match parse_result {
            Ok(v) => {
                println!("Good number!!");
                v
            }
            // I got this never bit from so https://stackoverflow.com/a/70518198
            // but it doesn't quite work
            Err(e) => {
                println!("Couldn't parse: {e:?}");
                //let _never = continue;
                continue;
                //never
            }
        };

        println!("Your guess is {guess}");

        // Here is how I would naturally write the checking logic
        //     if guess < secret_number {
        //         println!("Your guess is too small!");
        //     } else if guess > secret_number {
        //         println!("Your guess is too large!");
        //     } else {
        //         println!("you got it!!!!");
        //         break;
        //     }
        //     println!("Try another number")

        // Rust has pattern matching to encourage exhaustiveness checking

        let comparison = guess.cmp(&secret_number);
        match comparison {
            Ordering::Equal => {
                println!("You got it!!!!");
                break;
            }
            Ordering::Greater => {
                println!("Your guess is too big")
            }
            Ordering::Less => {
                println!("Your guess is too small")
            }
        }
        println!("Try another guess!");
    }
}
