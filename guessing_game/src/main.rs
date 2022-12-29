use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::num::ParseIntError;

// we added rand = "0.8.5" to the deps section of the toml
//  and then ran cargo build to get access to rand crate

// Ordering type has variants of Less Equal or Greater

fn print_banner() {
    println!("You're playing... Guess the number!\n");
    println!("Please input a guess (integer between 0 and 100 inclusive)");
}

fn gen_secret_number() -> u32 {
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Shh the secret number is {secret_number}");
    return secret_number;
}

/// Prompts user for an integer and returns Result<u32, ParseIntError>
///
/// # Panics
///  if Stdin.read_line() can't take the input
fn get_number_from_user() -> Result<u32, ParseIntError> {
    // Initialize a new mutable string for the guess
    let mut guess = String::new();

    // Read the user input and panic if can't get it

    // stdin() returns a Stdin instance, representing a handle to stdin from terminal
    io::stdin()
        // &mut guess means we pass a reference to the mutable string guess
        .read_line(&mut guess)
        // read_line() returns a Result (enum) which can be Ok or Err
        // the expect() method of Result will raise if Result is Err
        // If not err, expect will take the return value that Ok is holding.
        // In our case it's the number of bytes in the user input
        .expect("Failed to read line");

    //
    let parse_result = guess.trim().parse::<u32>();

    // use match to interpret the parse outcome. return Err if we got one or the Ok
    let guess = match parse_result {
        Ok(v) => v,
        Err(e) => {
            return Err(e);
        }
    };
    Ok(guess)
}

// fn process_iteration(secret_number: u32) -> Result<Ordering, Box<dyn std::error::Error>> {
fn process_iteration(secret_number: u32) -> Result<Ordering, ParseIntError> {
    // ? lets us automatically return the Error Result if we can't get a number
    let guess = get_number_from_user()?;
    let comparison = guess.cmp(&secret_number);
    match comparison {
        Ordering::Equal => {
            println!("You got it!!!!");
        }
        Ordering::Greater => {
            println!("Your guess is too big")
        }
        Ordering::Less => {
            println!("Your guess is too small")
        }
    }
    Ok(comparison)
}

fn main() {
    print_banner();
    let secret_number = gen_secret_number();

    // Loop forever while the user is prompted for a guess,
    //  the guess is converted to an integer and compared to the secret
    //  If processing the user input errors, print a message and continue

    // process_iteration returns a Result<Ordering, Box Error>
    // actually changed Box Error to the specific ParseIntError b/c that's all that can happen
    loop {
        match process_iteration(secret_number) {
            Ok(comparison) => {
                // This could be a nested match, but I really
                // only care about the Equal case b/c I've already checked
                if comparison == Ordering::Equal {
                    break;
                }
            }
            Err(err) => {
                println!("Uhoh had a problem {err}. Skipping");
            }
        }
        println!("Try another guess!");
    }
}
