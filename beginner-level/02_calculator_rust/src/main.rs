// Simple Calculator (Rust)
// Supports: + - * /

use std::io;

// Read and parse a number from the user
fn get_number(prompt: &str) -> f64 {

    loop {
        println!("{prompt}");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<f64>() {
            Ok(number) => return number,
            Err(_) => println!(" Please enter a valid number."),
        }
    }

}

// Read and validate an operator
fn get_operator() -> String {
    loop{

        println!("Enter an operator (+, -, *, /):");

        let mut operator = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read input");

        let operator = operator.trim().to_string();

        if ["+", "-", "*", "/"].contains(&operator.as_str()){
            return operator;
        }

        println!("Invalid operator. Please use +, -, *, or /.");

    }
}

// Perform the calculation
fn calculate(num1: f64, num2: f64, operator: &str) -> String{
    match operator {
        "+" => (num1 + num2).to_string(),
        "-" => (num1 - num2).to_string(),
        "*" => (num1 * num2).to_string(),
        "/" => {
            if num2 == 0.0 {
                "Error: Division by zero".to_string()
            } else {
                (num1 / num2).to_string()
            }
        }
        _ => "Unknown operator".to_string(),
    }

}

fn main() {
    println!("Simple Calculator(Rust)");
    println!("----------------------------");

    loop {
        let num1 = get_number("Enter first number:");
        let operator = get_operator();
        let num2 = get_number("Enter second number:");

        let result = calculate(num1, num2, &operator);

        println!("Result: {result}");

        println!("Do you want to calculate again? (y/n)");

        let mut again = String::new();

        io::stdin()
            .read_line(&mut again)
            .expect("Failed to read input");

        if again.trim().to_lowercase() != "y" {
            println!("Goodbye!");
            break;
        }
    }
}
