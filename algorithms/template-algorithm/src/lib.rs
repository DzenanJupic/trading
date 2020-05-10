//! This is a template that can be used for creating your own algorithms
//! For using it you should have a look at the Cargo.toml and change the
//! package name, the package author and the dependency path
//!
//! All the important features used here are imported from algorithm_utils.
//! These utils provide you the basic traits, struct, enums and macros for
//! successfully creating an algorithm.
//!
//! To start building your algorithm the first thing you need to do is to
//! define a public struct (in this example `Context`). This trait will be
//! your data storage. It's completely custom. Just make sure it derives
//! Clone and Debug.
//!
//! The next important step is to implement AlgorithmInterface for your
//! struct. This will provide an interface to communicate with your
//! algorithm.
//!
//! The last step is to export your algorithm using the export_algorithm
//! macro. This macro takes Your algorithms name as first argument and
//! the initial state of your struct as second one. The value you pass as
//! initial state will be the one passed to your `init` function.
//!
//! # NOTICE:
//! Even if it's possible to manually export your algorithm it's highly
//! recommended to use the `export_algorithm` macro. It will take care of
//! some internal things like the rustc version to make sure nothing
//! blows up. Manually exporting your algorithm with wrong settings
//! could potentially lead to undefined behaviour while trading.
//!
//! # IMPORTANT:
//! Please write code than will not and can not panic under **any**
//! circumstance. This could lead to massive losses caused by positions
//! that were not closed before exiting.
//! If you really have a problem you can't solve your self just return
//! an `algorithm_utils::error::Error` with the error kind of
//! `algorithm_utils::error::ErrorKind::Panic`. This will lead to a
//! save and organized exit and user defined behaviour in terms of the
//! open positions left.

use algorithm_utils as utils;
use utils::{AlgorithmInterface, Error, export_algorithm};

export_algorithm!("Template Algorithm", Context::default());

#[derive(Clone, Default, Debug)]
pub struct Context {
    first_price: Option<f64>,
    last_price: Option<f64>,
}

impl AlgorithmInterface for Context {
    fn about(&self) -> &str {
        "This is a little template Algorithm\n\
        You can use it to build your own ones"
    }

    fn min_prices(&self) -> u64 { 200 }

    fn max_prices(&self) -> u64 { 1200 }

    fn init(&mut self) -> Result<(), Error> {
        Ok(())
    }

    fn load_data(&mut self, prices: &[f64]) -> Result<(), Error> {
        if self.first_price.is_none() {
            self.first_price = Some(prices[0]);
        }
        Ok(())
    }

    fn algorithm(&mut self, prices: &[f64]) -> Result<(), Error> {
        self.last_price = Some(*prices.iter().last().unwrap());
        Ok(())
    }

    fn shutdown(&mut self) -> Result<(), Error> {
        println!("first: {:?}, last: {:?}", self.first_price, self.last_price);
        Ok(())
    }
}
