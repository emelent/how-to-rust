use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=10);

    println!("Guess the number!");

    loop {
        println!("\nPlease input your guess");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("Too low"),
            cmp::Ordering::Greater => println!("Too high"),
            cmp::Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}
