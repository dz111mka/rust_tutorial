use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Угадайте число");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("Секретное число {secret_number}");

    loop {
        println!("Введите плз ваше число");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number!");

        println!("Вы ввели: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}
