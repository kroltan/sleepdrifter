//! A lazy-evaluation library
//!
//! This library revolves around the concept of _expressions_,
//! which are representations of a future computation. It is
//! entirely implemented with generic types, no macros involved.

use std::marker::PhantomData;

pub mod ops;
pub mod param;

/// Re-exports all necessary types for common usage
pub mod prelude {
    pub use super::{lazy, lazyf, Lazy, Expression};
    pub use super::param::{Parameter, ParameterContent};
}

/// Convenience method to create a `Value` expression
pub fn lazy<T>(value: T) -> Lazy<T, Value<T>> {
    Lazy::new(Value(value))
}

/// Convenience method to create a `Function` expression
pub fn lazyf<T, F: FnOnce() -> T>(f: F) -> Lazy<T, Function<T, F>> {
    Lazy::new(Function(f))
}

/// Wrapper type which delegates operators into expressions
#[derive(Debug, Clone)]
pub struct Lazy<T, E: Expression<T>>(E, PhantomData<T>);

impl<T, E: Expression<T>> Lazy<T, E> {
    /// Creates a new `Lazy` from the given expression
    pub fn new(expr: E) -> Self {
        Lazy(expr, PhantomData)
    }
}

impl<T, E: Expression<T>> Expression<T> for Lazy<T, E> {
    fn evaluate(self) -> T {
        self.0.evaluate()
    }
}

/// A known, unchanging, value expression
#[derive(Debug, Clone)]
pub struct Value<T>(T);

impl<T> Expression<T> for Value<T> {
    fn evaluate(self) -> T {
        self.0
    }
}

/// Wrapper for an argument-less function
///
/// Resolves the value with the function's return
pub struct Function<T, F: FnOnce() -> T>(F);

impl<T, F: FnOnce() -> T> Expression<T> for Function<T, F> {
    fn evaluate(self) -> T {
        self.0()
    }
}

/// Represents a future computation
///
/// An expression represents a operation that will be performed.
/// Expressions can be composed using their `map` method, or,
/// if their underlying type, allows, operators.
pub trait Expression<T> {
    /// Executes the expression.
    ///
    /// Consumes the expression, applying all operations and
    /// returning their value.
    fn evaluate(self) -> T;

    /// Transform the value of an expression.
    ///
    /// Analogous to `Iterator::map` Creates an expression which transforms a value and assumes
    /// the value of the return of the provided function.
    fn map<U, F: Fn(T) -> U>(self, f: F) -> Lazy<U, LazyMap<T, Self, U, F>>
        where Self: Sized
    {
        Lazy::new(LazyMap(self, f, PhantomData, PhantomData))
    }
}

/// Internal type returned by `Expression::<T>::map`.
///
/// See its documentation for details.
#[derive(Debug, Clone)]
pub struct LazyMap<T, E: Expression<T>, U, F: Fn(T) -> U>(E, F, PhantomData<T>, PhantomData<U>);

impl<T, E: Expression<T>, U, F: Fn(T) -> U> Expression<U> for LazyMap<T, E, U, F> {
    fn evaluate(self) -> U {
        let LazyMap(expr, f, _, _) = self;
        f(expr.evaluate())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn literal() {
        let a = lazy(1);
        assert_eq!(a.evaluate(), 1);
    }

    #[test]
    fn function() {
        let a = lazyf(|| "potatoland".to_string());
        assert_eq!(a.evaluate(), "potatoland");
    }

    #[test]
    fn map() {
        let a = lazy(2i32).map(|n| n.pow(3));
        assert_eq!(a.evaluate(), 8);
    }
}
