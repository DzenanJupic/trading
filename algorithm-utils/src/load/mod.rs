pub use algorithm::*;
pub use algorithms::*;

pub mod algorithms;
pub mod algorithm;


#[macro_export]
macro_rules! global_allocator {
    () => {
        #[global_allocator]
        static ALLOCATOR: ::std::alloc::System = ::std::alloc::System;
    };
}
