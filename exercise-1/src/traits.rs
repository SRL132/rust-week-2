pub trait AdditiveOperations {
    fn add(&self, a: i32, b: i32) -> i32;
    fn subtract(&self, a: i32, b: i32) -> i32;
}

pub trait MultiplicativeOperations {
    fn multiply(&self, a: i32, b: i32) -> i32;
    fn divide(&self, a: i32, b: i32) -> Option<i32>;  // Option to handle division by zero
}

pub trait BinaryOperations {
    fn and(&self, a: bool, b: bool) -> bool;
    fn or(&self, a: bool, b: bool) -> bool;
    fn xor(&self, a: bool, b: bool) -> bool;
}