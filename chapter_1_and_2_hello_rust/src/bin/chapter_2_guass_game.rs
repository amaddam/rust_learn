use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    //生成一个随机数
    let secret_number = rand::thread_rng().gen_range(1..=100);

    //打印出这个随机数
    println!("The secret number is: {}", secret_number);

    //循环猜测
    loop {
        print!("Please input your guess: ");

        //创建一个可变的变量来存储用户输入的值, mut的全称是mutable(可变的)
        let mut guess = String::new();

        //读取用户输入的值并存储到guess变量中
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        //打印出用户输入的值
        println!("You guessed: {}", guess);

        // 将用户输入的值转换为数字类型, 如果转换成功, 则将值赋给guess变量, 否则继续循环读取输入的值
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //比较用户输入的值和随机数的大小
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            // 如果用户输入的值和随机数相等, 则打印出You win!并退出循环
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
