use std::io;

fn main() {
    println!("GUESS DA NUMBER!!!");
    println!("What's your guess?");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("That's not a valid guess you wanker!");

    println!("You guessed: {guess}");
}
