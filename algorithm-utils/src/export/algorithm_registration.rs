use crate::algorithm_interface::AlgorithmInterface;

#[derive(Clone, Debug)]
pub struct AlgorithmRegistration {
    pub rustc_version: &'static str,
    pub utils_version: &'static str,
    pub name: &'static str,
    pub default: unsafe extern fn() -> Box<dyn AlgorithmInterface>
}
