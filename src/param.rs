//! Parameters, values which can be filled after the expression's creation.

use std::cell::Cell;
use std::rc::Rc;

use super::Expression;

/// A handle to assign the parameter's value.
///
/// Can be used to set the value after the expression itself
/// has been consumed, but before it is evaluated.
#[derive(Clone)]
pub struct ParameterContent<T>(Rc<Cell<Option<T>>>);

impl<T> ParameterContent<T> {
    pub fn set(&mut self, value: T) {
        self.0.replace(Some(value));
    }
}

/// Parameter which can be defined later.
///
/// A value that is unknown at the time of the expression's definition,
/// but will be known before it is evaluated.
#[derive(Clone)]
pub struct Parameter<T>(Rc<Cell<Option<T>>>);

impl<T> Parameter<T> {
    fn create_with(value: Option<T>) -> (Self, ParameterContent<T>)  {
        let param = Parameter(Rc::new(Cell::new(value)));
        let handle = param.0.clone();
        (param, ParameterContent(handle))
    }

    /// Creates a parameter with no initial value.
    pub fn empty() -> (Self, ParameterContent<T>)  {
        Self::create_with(None)
    }

    /// Creates a parameter with an initial value.
    ///
    /// You can still change its value through the returned `Content` handle.
    pub fn new(value: T) -> (Self, ParameterContent<T>) {
        Self::create_with(Some(value))
    }

    fn take(&mut self) -> Option<T> {
        self.0.replace(None)
    }
}

impl<T> Expression<T> for Parameter<T> {
    /// Yields the value of the parameter.
    ///
    /// # Panics
    /// When evaluated without an undefined value.
    fn evaluate(mut self) -> T {
        self.take().expect("Parameter value not provided")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_usage_empty() {
        let (param, mut setter) = Parameter::<u32>::empty();
        let expr = param.map(|n| n.pow(3));
        setter.set(10u32);
        assert_eq!(expr.evaluate(), 1000);
    }

    #[test]
    fn correct_usage_prefilled() {
        let (param, _) = Parameter::new(10u32);
        let expr = param.map(|n| n.pow(3));
        assert_eq!(expr.evaluate(), 1000);
    }

    #[test]
    fn correct_usage_override() {
        let (param, mut setter) = Parameter::new(10u32);
        let expr = param.map(|n| n.pow(3));
        setter.set(2u32);
        assert_eq!(expr.evaluate(), 8);
    }

    #[test]
    #[should_panic]
    fn incorrect_usage() {
        let (param, _) = Parameter::<u32>::empty();
        let expr = param.map(|n| n.pow(3));
        expr.evaluate();
    }
}
