use std::io;

fn main() {
    println!("Guess the Number!");
    println!("input Guess. ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read");

    println!("you guessed : {guess}");
}
