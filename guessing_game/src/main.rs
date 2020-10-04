use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess number");

    let secret_number = rand::thread_rng().gen_range(1,101);

    loop {
        let mut guess = String::new();
        println!("input guess:");

        io::stdin()
            .read_line(&mut guess)
            .expect("didnae read line ?? :/");

        let guessi: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("type an actual number dawg");
                continue;
            }
        };

        println!("Your guess was {}", guessi);

        match guessi.cmp(&secret_number) {
            Ordering::Less => println!("Nope, too low"),
            Ordering::Greater => println!("Nope, too big"),
            Ordering::Equal => {
                println!("You got it");
                break;
            }
        }
    }
}
