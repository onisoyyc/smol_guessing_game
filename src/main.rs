use std::io; //input/output library

fn main() {
    //ask for user input
    println!("Guess the number!");
    println!("Please input your guess.");

    //take an integer from a user

    let mut guess = String::new(); //store user input into mutable variable
    //check the input is an integer
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    println!("You guessed: {guess}");
}
