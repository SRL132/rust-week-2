pub trait AdditiveOperations<T> {
    fn add(&self, a: T, b: T) -> T;
    fn subtract(&self, a: T, b: T) -> T;
}

pub trait MultiplicativeOperations<T> {
    fn multiply(&self, a: T, b: T) -> T;
    fn divide(&self, a: T, b: T) -> Option<T>;  // Option to handle division by zero
}

pub trait BinaryOperations<T> {
    fn and(&self, a: T, b: T) -> T;
    fn or(&self, a: T, b: T) -> T;
    fn xor(&self, a: T, b: T) -> T;
}