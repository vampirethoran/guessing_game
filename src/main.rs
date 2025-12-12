use std::io; //import io -  input/output from std - the standard library
use rand::Rng; //import Rng - random number generator from rand - the random library
use std::cmp::Ordering; //import Ordering - ordering from std - the standard library

fn main() {
    let mut human_guess = u32::new(); // create a mutable variable called guess that is a new empty string
    let machine_guess = rand::thread_rng()
        .gen_range(1..=100); // generate a random number between 1 and 100
        //.to_string(); // convert the random number to a string but this method will have a bug. safer way is to have the human input ONLY numbers.
    
    println!("GUESS DA NUMBER!!!"); //println! - print a string to the console
    println!("What's your guess?");

    io::stdin() // read input from the standard input (console)
        .read_line(&mut human_guess) // read a line from the standard input and store it in the guess variable
        .expect("That's not a valid guess you wanker!"); // if the read_line fails, print the error message
    let human_guess: u32 = human_guess.trim().parse().expect("That's not a valid guess you wanker!"); // parse the human_guess variable to a u32 number

    println!("The machine guessed: {machine_guess}");
    //println!("You guessed: {human_guess}"); // print the human_guess variable to the console

    match human_guess.cmp(&machine_guess) {
        Ordering::Less => println!("your guess is too low!"),
        Ordering::Greater => println!("your guess is too high!"),
        Ordering::Equal => println!("you win!"),
    } // match the human_guess variable to the machine_guess variable
}
