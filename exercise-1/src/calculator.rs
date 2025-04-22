use crate::traits::{AdditiveOperations, MultiplicativeOperations, BinaryOperations};

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

    fn divide(&self, a: i32, b: i32) -> Option<i32> {
        if b == 0 {
            None
        } else {
            Some(a / b)
        }
    }
}

impl BinaryOperations for Calculator {
    fn and(&self, a: bool, b: bool) -> bool {
        a && b
    }

    fn or(&self, a: bool, b: bool) -> bool {
        a || b
    }

    fn xor(&self, a: bool, b: bool) -> bool {
        a ^ b
    }
}
