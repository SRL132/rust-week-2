use crate::traits::{AdditiveOperations, MultiplicativeOperations, BinaryOperations};
//advise: Use a concrete integer for the type for the calculator to work on (e.g. i32) don't worry about it working for multiple types until Exercise 5.
pub struct Calculator {
    pub num1: i32,
    pub num2: i32,
}


impl AdditiveOperations for Calculator {
    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }
}

impl MultiplicativeOperations for Calculator {
    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }
//TODO: what to do with decimals?
    fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

impl BinaryOperations for Calculator {
    fn and(&self, a: i32, b: i32) -> i32 {
        a & b  // Bitwise AND
    }

    fn or(&self, a: i32, b: i32) -> i32 {
        a | b  // Bitwise OR
    }

    fn xor(&self, a: i32, b: i32) -> i32 {
        a ^ b  // Bitwise XOR
    }
}
