use std::fmt::{Display};
use std::io;
use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor};
use std::default::Default;
use std::cmp::PartialEq;

// src/main.rs
mod traits;
mod calculator;

use calculator::Calculator;
use traits::{AdditiveOperations, MultiplicativeOperations, BinaryOperations};
//TODO: - The "Calculator" can be printed through the following line of code `println!("calculator: {}", calculator);`
//TODO: - When printing the calculator, the result shows the result for each operation.
impl<T: Display> Display for Calculator<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.num1, self.num2)
    }
}

impl<T: Display + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + 
    BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy + Default + PartialEq> 
    Calculator<T> {
    pub fn print_output(&self) {
        println!("Additive Operations:");
        println!("  {} + {} = {}", self.num1, self.num2, self.add(self.num1, self.num2));
        println!("  {} - {} = {}", self.num1, self.num2, self.subtract(self.num1, self.num2));
        
        println!("\nMultiplicative Operations:");
        println!("  {} * {} = {}", self.num1, self.num2, self.multiply(self.num1, self.num2));
        match self.divide(self.num1, self.num2) {
            Some(result) => println!("  {} / {} = {}", self.num1, self.num2, result),
            None => println!("  {} / {} = Division by zero!", self.num1, self.num2),
        }
        
        println!("\nBinary Operations:");
        println!("  {} AND {} = {}", self.num1, self.num2, self.and(self.num1, self.num2));
        println!("  {} OR {} = {}", self.num1, self.num2, self.or(self.num1, self.num2));
        println!("  {} XOR {} = {}", self.num1, self.num2, self.xor(self.num1, self.num2));
    }
}

fn main() {
    let mut x: String = String::new();
    let mut y: String = String::new();
    let mut op: String = String::new();

    println!("Enter the first number: ");
    io::stdin().read_line(&mut x).expect("Invalid Input");

    let x: i32 = match x.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    println!("Enter the second number: ");
    io::stdin().read_line(&mut y).expect("Invalid Input");
    let y: i32 = match y.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    println!("Enter the operation: ");

    println!("List of operators:");
    println!("(1) Add");
    println!("(2) Subtract");
    println!("(3) Multiply");
    println!("(4) Divide");
    println!("(5) AND");
    println!("(6) OR");
    println!("(7) XOR");
    println!("(8) Print Output");
    println!("Select the number associated with the desired operation: ");

    io::stdin().read_line(&mut op).expect("Invalid Input");
    let op: i32 = match op.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input!");
            return;
        }
    };

    println!("Now, let's calculate!");

    let calculator: Calculator<i32> = Calculator::new();
    match op {
        1 => println!("{} + {} = {}", x, y, calculator.add(x, y)),
        2 => println!("{} - {} = {}", x, y, calculator.subtract(x, y)),
        3 => println!("{} * {} = {}", x, y, calculator.multiply(x, y)),
        4 => match calculator.divide(x, y) {
            Some(result) => println!("{} / {} = {}", x, y, result),
            None => println!("Error: Division by zero!"),
        },
        5 => println!("{} AND {} = {}", x, y, calculator.and(x, y)),
        6 => println!("{} OR {} = {}", x, y, calculator.or(x, y)),
        7 => println!("{} XOR {} = {}", x, y, calculator.xor(x, y)),
        8 => {
            let mut calc = Calculator::new();
            calc.num1 = x;
            calc.num2 = y;
            calc.print_output();
        },
        _ => println!("Invalid operation selected!"),
    }
}


#[cfg(test)]
mod calculator_tests {
    use super::*;
    use std::io::{Cursor, Write};

    #[test]
    fn test_additive_operations() {
        let calc: Calculator<i32> = Calculator::new();
        assert_eq!(calc.add(5, 3), 8);
        assert_eq!(calc.subtract(10, 4), 6);
    }

    #[test]
    fn test_multiplicative_operations() {
        let calc: Calculator<i32> = Calculator::new();
        assert_eq!(calc.multiply(4, 3), 12);
        assert_eq!(calc.divide(10, 2), Some(5));
        assert_eq!(calc.divide(5, 0), None);
    }

    #[test]
    fn test_binary_operations() {
        let calc: Calculator<i32> = Calculator::new();
        assert_eq!(calc.and(5, 3), 5 & 3);
        assert_eq!(calc.or(5, 3), 5 | 3);
        assert_eq!(calc.xor(5, 3), 5 ^ 3);
    }

    #[test]
    fn test_calculator_display() {
        let calculator: Calculator<i32> = Calculator::new();
        let display_output = format!("{}", calculator);
        assert!(display_output.contains("0 0"));
    }

    #[test]
    fn test_print_output() {
        let calculator: Calculator<i32> = Calculator::new();
        
        // Create a buffer to capture the output
        let mut output = Vec::new();
        {
            let mut cursor = Cursor::new(&mut output);
            writeln!(cursor, "Testing print_output:").unwrap();
            calculator.print_output();
        }
        
        let output_str = String::from_utf8(output).unwrap();
        
        // Test for presence of all operation sections
        assert!(output_str.contains("Additive Operations:"));
        assert!(output_str.contains("Multiplicative Operations:"));
        assert!(output_str.contains("Binary Operations:"));
        
        // Test for presence of operation results
        assert!(output_str.contains("0 + 0 = 0"));
        assert!(output_str.contains("0 - 0 = 0"));
        assert!(output_str.contains("0 * 0 = 0"));
        assert!(output_str.contains("0 AND 0 = 0"));
        assert!(output_str.contains("0 OR 0 = 0"));
        assert!(output_str.contains("0 XOR 0 = 0"));
    }
}