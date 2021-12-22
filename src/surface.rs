use crate::function::{Compute, Derivative, Function};
use std::collections::HashMap;

pub struct Surface<T> {
    pub variables: Vec<char>,
    pub functions: Vec<Function<T>>,
    pub bounds: Option<HashMap<char, (T, T)>>,
    pub color: [u8; 3],
}

impl Compute<f64> for Function<f64> {
    fn compute(&self, point: &HashMap<char, f64>) -> f64 {
        match self {
            Function::Additive(a, b) => a.compute(point) + b.compute(point),
            Function::Product(a, b) => a.compute(point) * b.compute(point),
            Function::Power(a, p) => (a.compute(point)).powf(*p),
            Function::Constant(a) => *a,
            Function::Variable(a) => point[a],
            Function::Cosine(a) => (a.compute(point)).cos(),
            Function::Sine(a) => (a.compute(point)).sin(),
        }
    }
}

impl Derivative for Function<f64> {
    fn derivative(&self) -> Self
    where
        Self: Clone,
    {
        match self {
            Function::Constant(_) => Function::Constant(0.0),
            Function::Variable(_) => Function::Constant(1.0),
            Function::Power(f, p) => Function::Product(
                Box::new(Function::Constant(*p)),
                Box::new(Function::Product(
                    Box::new(Function::Power((*f).clone(), p - 1.0)),
                    Box::new(f.derivative()),
                )),
            ),
            Function::Sine(f) => Function::Product(
                Box::new(f.derivative()),
                Box::new(Function::Cosine((*f).clone())),
            ),
            Function::Cosine(f) => Function::Product(
                Box::new(f.derivative()),
                Box::new(Function::Product(
                    Box::new(Function::Constant(-1.0)),
                    Box::new(Function::Sine((*f).clone())),
                )),
            ),
            Function::Product(u, v) => Function::Additive(
                Box::new(Function::Product(Box::new((*u).derivative()), v.clone())),
                Box::new(Function::Product(Box::new((*v).derivative()), u.clone())),
            ),
            Function::Additive(u, v) => {
                Function::Additive(Box::new((*u).derivative()), Box::new((*v).derivative()))
            }
        }
    }
}

impl Surface<f64> {
    pub fn new(
        variables: Vec<char>,
        functions: Vec<Function<f64>>,
        bounds: Option<HashMap<char, (f64, f64)>>,
        color: [u8; 3],
    ) -> std::io::Result<Self> {
        Ok(Surface {
            variables,
            functions,
            bounds,
            color,
        })
    }

    pub fn at(&self, t: &HashMap<char, f64>) -> Option<Vec<f64>> {
        let mut within_bounds: bool = true;
        if let Some(h) = &self.bounds {
            for (var, bounds) in h {
                within_bounds = t[var] >= bounds.0 && t[var] <= bounds.1;
            }
        }
        match within_bounds {
            true => Some(
                self.functions
                    .iter()
                    .map(|f| f.compute(t))
                    .collect::<Vec<f64>>(),
            ),
            false => None,
        }
    }

    pub fn derivative(&self) -> Self {
        Surface {
            variables: self.variables.clone(),
            functions: self
                .functions
                .clone()
                .iter()
                .map(|f| f.derivative())
                .collect(),
            bounds: self.bounds.clone(),
            color: self.color,
        }
    }
}
