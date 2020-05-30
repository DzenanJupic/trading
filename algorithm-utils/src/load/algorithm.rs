use std::fmt;
use std::fmt::Formatter;

use chrono::Duration;
use libloading::Library;

use crate::{AlgorithmInterface, Derivative, Error, Instruction, Position};

/// a wrapper around an extern AlgorithmInterface
///
/// This wrapper provides convenient access to a Box<dyn AlgorithmInterface.
/// This dynamic AlgorithmInterface usually is a dynamically loaded library that contains
/// an algorithm.
/// The _lib field is for the borrow checker to keep the library alive as long as an instace
/// of it is used.
pub struct Algorithm {
    name: &'static str,
    path: String,
    algorithm: Box<dyn AlgorithmInterface>,
    _lib: Library,
}

impl Algorithm {
    pub fn new(
        name: &'static str,
        path: String,
        algorithm: Box<dyn AlgorithmInterface>,
        _lib: Library,
    ) -> Self {
        Self {
            name,
            path,
            algorithm,
            _lib,
        }
    }

    pub fn name(&self) -> &'static str {&self.name}
    pub fn path(&self) -> &String {&self.path}
}

impl AlgorithmInterface for Algorithm {
    #[inline]
    fn about(&self) -> &'static str {
        self.algorithm.about()
    }

    #[inline]
    fn min_prices(&self) -> u64 {
        self.algorithm.min_prices()
    }

    #[inline]
    fn max_prices(&self) -> u64 {
        self.algorithm.max_prices()
    }

    #[inline]
    fn init(&mut self, derivative: &Derivative, time_steps: Duration) -> Result<(), Error> {
        self.algorithm.init(derivative, time_steps)
    }

    #[inline]
    fn collect_prices(&mut self, prices: &[f64]) -> Result<(), Error> {
        self.algorithm.collect_prices(prices)
    }

    #[inline]
    fn algorithm(&mut self, positions: &[Position], prices: &[f64]) -> Result<&[Instruction], Error> {
        self.algorithm.algorithm(positions, prices)
    }

    #[inline]
    fn shutdown(&mut self, positions: &[Position], prices: &[f64]) -> Result<&[Instruction], Error> {
        self.algorithm.shutdown(positions, prices)
    }
}

impl fmt::Display for Algorithm {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(formatter, "{} ({})", self.name, self.path)
    }
}
