//! This is a template that can be used for creating your own algorithms
//! For using it you should have a look at the Cargo.toml and change the
//! package name, the package author and the dependency path
//!
//! All the important features used here are imported from algorithm_utils.
//! These utils provide you the basic traits, struct, enums and macros for
//! successfully creating an algorithm.
//!
//! To start building your algorithm the first thing you need to do is to
//! define a public struct (in this example `Context`). This struct will be
//! your data storage. It's completely custom.
//!
//! The next important step is to implement AlgorithmInterface for your
//! struct. This will provide an interface for trading-desk to communicate
//! with your algorithm.
//!
//! #### export_algorithm!
//! The last step is to export your algorithm using the export_algorithm
//! macro. It is indeed possible to export it manually but __not__ recommended.
//! this macro takes care of a lot of internal stuff that could lead to undefined
//! behavior.
//! Please note that you can just export one algorithm per crate.
//! ##### arguments
//! * __NAME__: The first argument to the export_algorithm is your algorithms name.
//! * __DESCRIPTION__: The second argument is a brief description of what your algorithm
//! does and how it works. This description is just for the user.
//! * __MIN_DATA_LENGTH__: This argument is optional (If you use it you also have
//! to use MAX_DATA_LENGTH) and describe the minimum amount of prices you need.
//! This signals trading-desk that your algorithm needs at least x prices for working
//! correctly. While the amount of collected prices is less then your minimum the
//! `collect_prices` method will be called (For more information about `collect_prices`
//! have a look at the algorithm-utils documentation).
//! (default = 0)
//! * __MAX_DATA_LENGTH__: This argument is optional (If you use it you also have
//! to use MIN_DATA_LENGTH) and describe the maximum amount of prices you can handle.
//! The maximum data length is an indicator for trading-desk for how many prices
//! have to be saved internally. Your algorithm will never get more prices then
//! this (what does not always mean it will ever get that amount).
//! Please note that max_data_length has to be greater or equal to min_data_length or 0.
//! Please note that a value of 0 corresponds to infinity.
//! (default = 0)
//! * __ALGORITHM__: Finally you have to pass in an instance of your algorithm.
//! Since trading-desk cannot call a constructor on your algorithm you have to help
//! it out here. This initial state will be the one received by `init`, so please do
//! heavy computations there if needed. Otherwise this will lower the startup performance
//! of trading-desk drastically. Calling the default constructor will solve this problem
//! in most cases.
//!
//! # NOTICE:
//! Even if it's possible to manually export your algorithm it's highly
//! recommended to use the `export_algorithm` macro. It will take care of
//! some internal things like the rustc version to make sure nothing
//! blows up. Manually exporting your algorithm with wrong settings
//! could potentially lead to undefined behaviour while trading.
//!
//! # IMPORTANT:
//! Please write code that will not and can not panic under __any__
//! circumstance. This could lead to massive losses caused by positions
//! that were not closed before exiting.
//! If you really have a problem you can't solve your self just return
//! an `algorithm_utils::error::Error` with the error kind of
//! `algorithm_utils::error::ErrorKind::Panic`. This will lead to a
//! save and organized exit and user defined behaviour in terms of the
//! open positions left.
//! # IMPORTANT
//! Your algorithm will be plugged in directly into the users trading-desk
//! application and will not experience any kind of sand-boxing. This means
//! you could potentially write malicious code that steals user data or plays
//! around with the file system. Please don't do it! Such things just suck!
//! If possible please provide the whole crate to the user so he can inspect
//! the code himself (precompiled crates are preferred so nobody has to install
//! a compiler).
//! ## What makes a safe algorithm
//! Here are some of the things you should prevent to write safe algorithms:
//!     * panicking
//!     * accessing the file system
//!     * connecting to the internet, opening ports, opening sockets, ...
//!     * blocking the whole application
//!     * burning money by purpose
//!
//! This list does not claim to be complete. Also there are algorithms where
//! some of these things might be necessary. Just keep in mind: In the end it's
//! your readability to deliver safe algorithms.


use trading_utils::{
    Derivative, Instruction, Position, Price,
    TradingErrorKind, AlgorithmInterface, Error, export_algorithm
};
use chrono::Duration;

export_algorithm!(
    "Template Algorithm",               // name
    "A little template description",    // description
    10,                                 // min data length
    20,                                 // max data length
    Context::default()                  // initial struct value
);
// export_algorithm!(
//      "Template Algorithm",           // name
//      "A little template description",// description
//                                      // < no min & max data length (results in (0, 0))
//       Context::default()             // initial struct value
// );

#[derive(Default)]
pub struct Context {
    first_init_price: Option<f64>,
    last_init_price: Option<f64>,
    first_algorithm_price: Option<f64>,
    last_algorithm_price: Option<f64>,
}

impl AlgorithmInterface for Context {
    fn init(&mut self, _derivative: &Derivative, _time_steps: Duration) -> Result<(), Error<TradingErrorKind>> {
        println!("init");
        Ok(())
    }

    fn collect_prices(&mut self, prices: &[Price]) -> Result<(), Error<TradingErrorKind>> {
        println!("collect");
        if self.first_init_price.is_none() {
            self.first_init_price = Some(prices[0])
        }
        self.last_init_price = Some(*prices.iter().last().unwrap());
        Ok(())
    }

    fn algorithm(&mut self, _positions: &[Position], prices: &[Price]) -> Result<&[Instruction<'_>], Error<TradingErrorKind>> {
        println!("algorithm");
        if self.first_algorithm_price.is_none() {
            self.first_algorithm_price = Some(prices[0])
        }
        self.last_algorithm_price = Some(*prices.iter().last().unwrap());
        Ok(&[])
    }

    fn shutdown(&mut self, _positions: &[Position], _prices: &[Price]) -> Result<&[Instruction<'_>], Error<TradingErrorKind>> {
        println!("shutdown");
        println!("first init price:\t{:?}", self.first_init_price);
        println!("last init price:\t{:?}", self.last_init_price);
        println!("first algorithm price:\t{:?}", self.first_algorithm_price);
        println!("last algorithm price:\t{:?}", self.last_algorithm_price);
        Ok(&[])
    }
}
