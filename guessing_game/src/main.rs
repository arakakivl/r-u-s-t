// io is a module inside the std library.
use std::io;
use std::io::Write;

// importing the Rng trait
use rand::Rng;
// importing the Ordering enum.
use std::cmp::Ordering;

fn main() {
    // Generating a random number passing a range expression (inclusive when with the '=' sign).
    let correct_answer = rand::rng().random_range(0..=100);
    println!("========== Guessing Game ==========");

    // Looping
    loop {

        // Mutable variable, of string type.
        // "String" is a type provided by the std library. The "new" function is associated with this type.
        let mut user_guess = String::new();

        print!("Which is your answer? ");

        // Flushing the stdout.
        io::stdout().flush().expect("Unhandled error at flush function.");

        // Reading a line w/ the read_line function. This function appends the user input to the "user_guess" variable,
        // and that's why a mutable reference of the "user_guess" variable is being passed as an argument.

        // The "read_line" function returns a "Result" enum -- a type of data that represents a value
        // that can be in one of many states. In the case of the "Result" enum, it can be either "Err"
        // and "Ok". In rust, enums can have associated methods; the "expect" is one for the "Result"
        // enum. It crashes the application if the state of "Result" is "Err", and returns an error. 
        // If it is "Ok", it can returns a value; in this case, the number of bytes inputted by the
        // user.
        io::stdin().read_line(&mut user_guess).expect("Unhandled error at read_line function.");

        // Shadowing the "user_guess" variable to an immutable, unsigned integer one.
        // Shadowing allows the reuse of a variable name, discarding the older one.
        // The parse function is responsible to convert the value type, to the explicited type
        // annotation. It returns a "Result" enum, containing either the error or the parsed value in
        // the "Ok" state.
        let user_guess: u32 = match user_guess.trim().parse() {
            Ok(guess) => guess,
            Err(_) => continue,
        };

        // Comparing the provided input with the correct answer, using the "match" expression.
        // Passing a reference of the value "correct_answer".
        match user_guess.cmp(&correct_answer) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You won!");
                break;
            }
        }
    }
}
