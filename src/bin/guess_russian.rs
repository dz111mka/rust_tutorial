use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Введите число");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Ожидается ввод числа");

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Ваше число {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Ваше число меньше загаданного"),
            Ordering::Greater => println!("Ваше число больше загаданного"),
            Ordering::Equal => {
                println!("Вы победили");
                break;
            }
        }
    }
}