use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // mutable
                                   // let apple = 5 // immutable

    io::stdin()
        .read_line(&mut guess) // it is important to add &mut to modify references; if you use &guess, it is immutable
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
