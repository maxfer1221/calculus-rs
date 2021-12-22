use std::collections::HashMap;

#[derive(Clone)]
pub enum Function<T> {
    Variable(char),
    Constant(T),
    Power(Box<Function<T>>, T),
    Sine(Box<Function<T>>),
    Cosine(Box<Function<T>>),
    Product(Box<Function<T>>, Box<Function<T>>),
    Additive(Box<Function<T>>, Box<Function<T>>),
}

pub trait Constants {
    fn one() -> Self;
    fn negative_one() -> Self;
    fn zero() -> Self;
    fn minus_one(&self) -> Self;
}

pub trait Derivative {
    fn derivative(&self) -> Self
    where
        Self: Clone;
}

pub trait Compute<T> {
    fn compute(&self, point: &HashMap<char, T>) -> T;
}
