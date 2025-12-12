use std::io; //import io -  input/output from std - the standard library
use rand::Rng; //import Rng - random number generator from rand - the random library
use std::cmp::Ordering; //import Ordering - ordering from std - the standard library
use colored::*; //import colored - colored text for terminal output

fn main() {
    let machine_guess = rand::thread_rng()
        .gen_range(1..=100); // generate a random number between 1 and 100
    
    println!("GUESS DA NUMBER!!!"); //println! - print a string to the console
    
    loop {
        println!("What's your guess?");
        
        let mut human_guess = String::new(); // create a mutable variable called guess that is a new empty string
        
        io::stdin() // read input from the standard input (console)
            .read_line(&mut human_guess) // read a line from the standard input and store it in the guess variable
            .expect("That's not a valid guess you wanker!"); // if the read_line fails, print the error message
        let human_guess: u32 = human_guess.trim().parse().expect("That's not a valid guess you wanker!"); // parse the human_guess variable to a u32 number

        match human_guess.cmp(&machine_guess) {
            Ordering::Less => println!("{}", "your guess is too low!".truecolor(255, 165, 0)),
            Ordering::Greater => println!("{}", "your guess is too high!".yellow()),
            Ordering::Equal => {
                println!("you win!");
                break; // exit the loop when the guess is correct
            }
        } // match the human_guess variable to the machine_guess variable
    }
}
