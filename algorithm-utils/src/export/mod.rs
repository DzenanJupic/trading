pub use self::algorithm_interface::*;
pub use self::algorithm_registration::*;

pub mod algorithm_interface;
pub mod algorithm_registration;

#[macro_export]
macro_rules! export_algorithm {
    ($name:literal, $algorithm:expr) => {
        pub extern fn initial_algorithm_state() -> ::std::boxed::Box<dyn AlgorithmInterface> {
            ::std::boxed::Box::new($algorithm)
        }

        #[doc(hidden)]
        #[no_mangle]
        // needs to be static
        pub static ALGORITHM_REGISTRATION: $crate::AlgorithmRegistration = $crate::AlgorithmRegistration {
            rustc_version: $crate::RUSTC_VERSION,
            utils_version: $crate::UTILS_VERSION,
            name: $name,
            initial_algorithm_state_fn: initial_algorithm_state,
        };
    };
}
