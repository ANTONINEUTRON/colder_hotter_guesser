use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_num = rand::thread_rng().gen_range(1..=100);

    println!("\nA secret number has been generated\nGuess the number!\n\n");

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        
        println!("You guessed: {}", guess);  

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("HOTTER!"),
            Ordering::Greater => println!("COLDER!"),
            Ordering::Equal => {
                println!("You win!!!");
                break;
            },
        }
    }
}