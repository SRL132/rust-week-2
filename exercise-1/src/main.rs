use std::fmt::{Display};
// src/main.rs
mod traits;
mod calculator;

use calculator::Calculator;
use traits::{AdditiveOperations, MultiplicativeOperations, BinaryOperations};

impl Display for Calculator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.num1, self.num2)
    }
}

fn main() {
    println!("Hello, world!");
}
