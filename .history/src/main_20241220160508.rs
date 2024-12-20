use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("来玩猜数字游戏吧!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {}", secret_number);

    loop {
        println!("请输入你的猜测:");
        let mut guess: String = String::new();

        io::stdin().read_line(&mut guess).expect("读取行失败");

        let guess: u32 = guess.trim().parse() {
          Ok(num) => num,
          Err(_) => continue,
        };

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
