use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    println!("Guess the number!");

    // 使用外部crate
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret_number);
    
    loop {
        println!("Please input your guess.");

        // guess 可变
        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        // guess 隐藏
        // let guess:u32 = guess.trim().parse()
        //     .expect("Please type a number!");
        
        // 异常输入不崩溃
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };


        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("you win");
                break;
            }
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
        }
    }
    


}