const CONST_TEMPERATURE: f64 = 32.0;

fn main() {
    println!("Введите температуру");
    let mut temperature = String::new();
    std::io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");

    let temperature: f64 = temperature.trim().parse().expect("Нужно было ввести число");

    loop {
        println!("\nВыберите действие:");
        println!("1 - Цельсий → Фаренгейт");
        println!("2 - Фаренгейт → Цельсий");
        println!("0 - Выход");

        let mut choise = String::new();
        std::io::stdin()
            .read_line(&mut choise)
            .expect("Failed to read line");

        let choise: i8 = choise.trim()
            .parse()
            .expect("Нужно ввести число");

        if  choise == 1 {
            let f = celcium_to_fahrenheit(temperature);
            println!("{} °C = {:.2} °F", temperature, f);
        } else if choise == 2 {
            let c = fahrenheit_to_celcium(temperature);
            println!("{} °F = {:.2} °C", temperature, c);
        } else { break; }
    }



}

fn celcium_to_fahrenheit(celcium: f64) -> f64 {
    (celcium * 1.8) + CONST_TEMPERATURE
}

fn fahrenheit_to_celcium(fahrenheit: f64) -> f64 {
    (fahrenheit - CONST_TEMPERATURE) / 1.8
}

