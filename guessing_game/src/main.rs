use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    // println!("Secret number is: {secret_number}");

    loop {
        println!("Please input the number you want to guess:");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed read the line");
        println!("You guessed: {guess}");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("It's not a number, please check it.");
                continue;
            },
        };

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You win!");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too big"),
        }
    }
}
