// This program will convert temperatures between Fahrenheit and Celsius

use std::io;

fn main() {
    println!("\nThis program will convert temperatures between Fahrenheit and Celsius\n");
    println!("Choose an option: ");
    println!("1. Fahrenheit to Celsius ");
    println!("2. Celsius to Fahrenheit ");

    let mut option = String::new();
    io::stdin()
        .read_line(&mut option)
        .expect("Failed to read line");

    let option: i8 = match option.trim().parse() {
        Ok(num) => num,
        Err(_) => { -1 }
    };

    if option != 1 && option != 2 {
        println!("\nPlease enter a valid option");
        return
    }


    if option == 1 {
        println!("\nFahrenheit: ");
        let input: f64 = get_input_and_parse();
        if input == -0.0 { println!("Please enter a valid number"); return }
        println!("Celsius equivalent: {}", f_to_c(input));
    } else {
        println!("\nCelsius: ");
        let input: f64 = get_input_and_parse();
        if input == -0.0 { println!("Please enter a valid number"); return }
        println!("Fahrenheit equivalent: {}", c_to_f(input));

    }
}

fn get_input_and_parse() -> f64 {
    let mut value = String::new();
    io::stdin()
        .read_line(&mut value)
        .expect("Failed to read line");

    let value: f64 = match value.trim().parse() {
        Ok(num) => num,
        Err(_) => { -0.0 }
    };

    value
}

fn c_to_f(x: f64) -> f64 {
    x * (9.0 / 5.0) + 32.0
}

fn f_to_c(x: f64) -> f64 {
    (x - 32.0) * 5.0 / 9.0
}
