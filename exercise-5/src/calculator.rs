use crate::traits::{AdditiveOperations, MultiplicativeOperations, BinaryOperations};
use std::ops::{Add, Sub, Mul, Div, BitAnd, BitOr, BitXor};
use std::fmt::Display;
use std::default::Default;
use std::cmp::PartialEq;
//advise: Use a concrete integer for the type for the calculator to work on (e.g. i32) don't worry about it working for multiple types until Exercise 5.
pub struct Calculator<T> {
    pub num1: T,
    pub num2: T,
}

impl<T: Default> Calculator<T> {
    pub fn new() -> Self {
        Calculator {
            num1: T::default(),
            num2: T::default(),
        }
    }
}

impl<T: Add<Output = T> + Sub<Output = T> + Copy + Display> AdditiveOperations<T> for Calculator<T> {
    fn add(&self, a: T, b: T) -> T {
        a + b
    }

    fn subtract(&self, a: T, b: T) -> T {
        a - b
    }
}

impl<T: Mul<Output = T> + Div<Output = T> + Copy + Display + Default + PartialEq> MultiplicativeOperations<T> for Calculator<T> {
    fn multiply(&self, a: T, b: T) -> T {
        a * b
    }
//TODO: what to do with decimals?
    fn divide(&self, a: T, b: T) -> Option<T> {
        if b == T::default() {
            None
        } else {
            Some(a / b)
        }
    }
}

impl<T: BitAnd<Output = T> + BitOr<Output = T> + BitXor<Output = T> + Copy + Display> BinaryOperations<T> for Calculator<T> {
    fn and(&self, a: T, b: T) -> T {
        a & b  // Bitwise AND
    }

    fn or(&self, a: T, b: T) -> T {
        a | b  // Bitwise OR
    }

    fn xor(&self, a: T, b: T) -> T {
        a ^ b  // Bitwise XOR
    }
}
