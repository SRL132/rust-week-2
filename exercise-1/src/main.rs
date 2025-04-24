use std::fmt::{Display};
// src/main.rs
mod traits;
mod calculator;

use calculator::Calculator;
use traits::{AdditiveOperations, MultiplicativeOperations, BinaryOperations};
//TODO: - The “Calculator” can be printed through the following line of code `println!("calculator: {}", calculator);`
//TODO: - When printing the calculator, the result shows the result for each operation.
impl Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.num1, self.num2)
    }
}

impl Calculator {
    pub fn new() -> Self {
        Calculator {
            num1: 0,
            num2: 0,
        }
    }
//TODO: confirm Add a print_output function which needs a single input parameter with the 3 defined traits:  AdditiveOperations, MultiplicativeOperations , BinaryOperations 
//This function should print every operations’ results for the given input.
    pub fn print_output<T: AdditiveOperations + MultiplicativeOperations + BinaryOperations>(&self, calculator: &T) {
        println!("Additive Operations:");
        println!("  {} + {} = {}", self.num1, self.num2, calculator.add(self.num1, self.num2));
        println!("  {} - {} = {}", self.num1, self.num2, calculator.subtract(self.num1, self.num2));
        
        println!("\nMultiplicative Operations:");
        println!("  {} * {} = {}", self.num1, self.num2, calculator.multiply(self.num1, self.num2));
        match calculator.divide(self.num1, self.num2) {
            Some(result) => println!("  {} / {} = {}", self.num1, self.num2, result),
            None => println!("  {} / {} = Division by zero!", self.num1, self.num2),
        }
        
        println!("\nBinary Operations:");
        println!("  {} AND {} = {}", true, false, calculator.and(true, false));
        println!("  {} OR {} = {}", true, false, calculator.or(true, false));
        println!("  {} XOR {} = {}", true, false, calculator.xor(true, false));
    }
}

fn main() {
    println!("Hello, calculator!");
}


#[cfg(test)]
mod calculator_tests {
    use super::*;
    use std::io::{Cursor, Write};  // Add this import

    #[test]
    fn test_additive_operations() {
        let calc = Calculator::new();
        assert_eq!(calc.add(5, 3), 8);
        assert_eq!(calc.subtract(10, 4), 6);
    }

    #[test]
    fn test_multiplicative_operations() {
        let calc = Calculator::new();
        assert_eq!(calc.multiply(4, 3), 12);
        assert_eq!(calc.divide(10, 2), Some(5));
        assert_eq!(calc.divide(5, 0), None);
    }

    #[test]
    fn test_binary_operations() {
        let calc = Calculator::new();
        assert_eq!(calc.and(true, false), false);
        assert_eq!(calc.or(true, false), true);
        assert_eq!(calc.xor(true, true), false);
    }

    #[test]
    fn test_calculator_display() {
        let calculator = Calculator::new();
        let display_output = format!("{}", calculator);
        assert!(display_output.contains("Calculator Results:"));
        assert!(display_output.contains("Additive Operations:"));
        assert!(display_output.contains("Multiplicative Operations:"));
        assert!(display_output.contains("Binary Operations:"));
    }

    #[test]
    fn test_print_output() {
        let calculator = Calculator::new();
        
        // Create a buffer to capture the output
        let mut output = Vec::new();
        {
            let mut cursor = Cursor::new(&mut output);
            writeln!(cursor, "Testing print_output:").unwrap();
            calculator.print_output(&calculator);
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
        assert!(output_str.contains("true AND false = false"));
        assert!(output_str.contains("true OR false = true"));
        assert!(output_str.contains("true XOR false = true"));
    }
}