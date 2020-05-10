use chrono::Duration;
use libloading::Library;

use crate::{AlgorithmInterface, Error, Instruction, Position, Derivative};

pub struct Algorithm {
    algorithm: Box<dyn AlgorithmInterface>,
    _lib: Library,
}

impl Algorithm {
    pub fn new(algorithm: Box<dyn AlgorithmInterface>, _lib: Library) -> Self {
        Self {
            algorithm,
            _lib,
        }
    }
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
