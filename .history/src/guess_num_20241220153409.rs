use std::io;

fn guessNum() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess);
}
