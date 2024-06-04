#![deny(clippy::all)]
use std::io::{self, Write};

enum Operation {
    //Add, Subtract, Multiply, and Divide
    Add(f64, f64),
    Subtract(f64, f64),
    Multiply(f64, f64),
    Divide(f64, f64),
}

impl Operation {
    fn calculate(self) -> f64 {
        match self {
            Operation::Add(x, y) => x + y,
            Operation::Subtract(x, y) => x - y,
            Operation::Multiply(x, y) => x * y,
            Operation::Divide(x, y) => {
                if y != 0.0 {
                    x / y
                } else {
                    println!("Cannot divide by zero!");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn prompt_for_number(prompt: &str) -> f64 {
    loop {
        //without loop error in match input (Err())
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        //turn it to f64
        match input.trim().parse::<f64>() {
            Ok(num) => return num,
            Err(_) => println!("Invalid number, please try again."),
        }
    }
}

fn prompt_for_operation(prompt: &str) -> String {
    loop {
        //loop bec i he enters smth wrong, it will ask again and not return gher number
        print!("{}", prompt);
        io::stdout().flush().unwrap();

        let mut op = String::new();
        io::stdin().read_line(&mut op).unwrap();

        let trim = op.trim();

        if trim.len() == 1 {
            //This converts the trimmed string into an iterator of characters and retrieves the first character using the .next() method.
            let char = trim.chars().next().unwrap();
            if "+-*/".contains(char) {
                return char.to_string();
            }
        }
    }
}

fn main() {
    let n1 = prompt_for_number("Enter number: ");
    let operation = prompt_for_operation("Enter the operation (+, -, *, /): ");
    let n2 = prompt_for_number("Enter number: ");

    let op = match operation.as_str() {
        "+" => Operation::Add(n1, n2),
        "-" => Operation::Subtract(n1, n2),
        "*" => Operation::Multiply(n1, n2),
        "/" => Operation::Divide(n1, n2),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let result = op.calculate();
    println!("The result is: {}", result);

    // let result = op.calculate();
    //println!("The result is: {}", result);

    println!("Hello, world!");
}
