use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess.");
        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("请你猜一个数字: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了!"),
            Ordering::Greater => println!("大了!"),
            Ordering::Equal => {
                println!("你赢了!");
                break;
            }
        }
    }
}
