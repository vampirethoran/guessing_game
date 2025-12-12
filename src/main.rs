use std::io; //import io -  input/output from std - the standard library

fn main() {
    println!("GUESS DA NUMBER!!!"); //println! - print a string to the console
    println!("What's your guess?");

    let mut guess = String::new(); // create a mutable variable called guess that is a new empty string

    io::stdin() // read input from the standard input (console)
        .read_line(&mut guess) // read a line from the standard input and store it in the guess variable
        .expect("That's not a valid guess you wanker!"); // expect("That's not a valid guess you wanker!") - if the read_line fails, print the error message

    println!("You guessed: {guess}"); // print the guess variable to the console
}
