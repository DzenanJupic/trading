pub use self::algorithm_interface::*;
pub use self::algorithm_registration::*;

pub mod algorithm_interface;
pub mod algorithm_registration;

#[macro_export]
macro_rules! export_algorithm {
    ($name:literal, $algorithm:expr) => {
        pub extern fn default_algorithm() -> ::std::boxed::Box<dyn AlgorithmInterface> {
            ::std::boxed::Box::new($algorithm)
        }

        #[doc(hidden)]
        #[no_mangle]
        pub const ALGORITHM_DECLARATION: $crate::AlgorithmDeclaration = $crate::AlgorithmDeclaration {
            rustc_version: $crate::RUSTC_VERSION,
            utils_version: $crate::UTILS_VERSION,
            name: $name,
            default: default_algorithm,
        };
    };
}
