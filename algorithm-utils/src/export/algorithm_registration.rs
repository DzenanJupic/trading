use crate::algorithm_interface::AlgorithmInterface;

#[derive(Clone, Debug)]
pub struct AlgorithmRegistration {
    pub rustc_version: &'static str,
    pub utils_version: &'static str,
    pub name: &'static str,
    pub initial_algorithm_state_fn: unsafe extern fn() -> Box<dyn AlgorithmInterface>
}
