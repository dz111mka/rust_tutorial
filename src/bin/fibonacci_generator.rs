#![warn(clippy::all, clippy::pedantic)]

/*use std::io;

fn main() {
    let mut n = String::new();
    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: i128 = n.trim().parse().expect("Нужно ввести число");

    let i = fibonacci_generator(n);
    println!("Число Фибоначчи для коэффициента {n} равно {i}");
}

fn fibonacci_generator(n: i128) -> i128 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }
    fibonacci_generator(n - 1) + fibonacci_generator(n - 2)
}*/

// вариант предложенный DeepSeek
use std::io;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: i128 = n.trim().parse().expect("Нужно ввести число");

    if n < 0 {
        println!("Число Фибоначчи определено только для неотрицательных чисел");
        return;
    }

    let result = fibonacci_generator(n);
    println!("Число Фибоначчи для коэффициента {n} равно {result}");
}

fn fibonacci_generator(n: i128) -> i128 {
    match n {
        0 => 0,
        1 => 1,
        _ => {
            let mut prev = 0;
            let mut curr = 1;

            for _ in 2..=n {
                let next = prev + curr;
                prev = curr;
                curr = next;
            }
            curr
        }
    }
}