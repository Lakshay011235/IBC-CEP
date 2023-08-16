use std::io;

fn add(a: &f64, b: &f64) -> f64 {
    a + b
}

fn subtract(a: &f64, b: &f64) -> f64 {
    a - b
}

fn multiply(a: &f64, b: &f64) -> f64 {
    a * b
}

fn divide(a: &f64, b: &f64) -> Result<f64, &'static str> {
    if *b == 0.0 {
        Err("Cannot divide by zero!")
    } else {
        Ok(a / b)
    }
}

fn main() {
    println!("Simple Calculator");

    let mut input = String::new();
    println!("Enter the first number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num1: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Enter the second number:");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let num2: f64 = input.trim().parse().expect("Please enter a valid number");

    input.clear();
    println!("Select operation (+, -, *, /):");
    io::stdin().read_line(&mut input).expect("Failed to read input");
    let operation = input.trim();

    let result = match operation {
        "+" => add(&num1, &num2),
        "-" => subtract(&num1, &num2),
        "*" => multiply(&num1, &num2),
        "/" => match divide(&num1, &num2) {
            Ok(result) => result,
            Err(err) => {
                println!("{}", err);
                return;
            }
        },
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    println!("Result: {} {} {} = {}", num1, operation, num2, result);
}
