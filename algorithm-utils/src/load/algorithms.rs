use std::collections::HashMap;
use std::ffi::OsStr;
use std::path::Path;

use libloading::Library;

use crate::load::algorithm::Algorithm;
use crate::{Error, ErrorKind};

type AlgorithmDeclaration = *mut crate::algorithm_registration::AlgorithmRegistration;

#[derive(Default)]
pub struct Algorithms {
    algorithms: HashMap<&'static str, Algorithm>,
}

impl Algorithms {
    /// loads all algorithms of a directory
    pub fn load_all<P: AsRef<Path>>(&mut self, _path: P) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    /// loads a algorithm by path
    pub fn load<P: AsRef<OsStr>>(&mut self, path: P) -> Result<(), Error> {
        let lib = Library::new(path)?;

        let algorithm_declaration;
        unsafe {
            algorithm_declaration = lib
                .get::<AlgorithmDeclaration>(b"ALGORITHM_DECLARATION")?
                .read();
        }

        if algorithm_declaration.rustc_version != crate::RUSTC_VERSION
            || algorithm_declaration.utils_version != crate::UTILS_VERSION {
            return Err(Error::new(
                format!(
                    "The algorithm `{}` has a mismatched version!\n\
                    Algorithm version: [{}/{}]\nUtils version: [{}/{}]",
                    algorithm_declaration.name,
                    algorithm_declaration.rustc_version,
                    algorithm_declaration.utils_version,
                    crate::RUSTC_VERSION,
                    crate::UTILS_VERSION
                ),
                ErrorKind::MisMatchedVersion
            ))
        }

        let algorithm_name = algorithm_declaration.name;
        let algorithm_default = unsafe { (algorithm_declaration.default)() };

        let algorithm = Algorithm::new(
            algorithm_default,
            lib,
        );

        self.algorithms.insert(
            algorithm_name,
            algorithm,
        );

        Ok(())
    }

    /// returns a reference to a `Algorithm`
    pub fn get(&self, algorithm: &str) -> Option<&Algorithm> {
        self.algorithms.get(algorithm)
    }

    /// returns a mutable reference to a `Algorithm`
    pub fn get_mut(&mut self, algorithm: &str) -> Option<&mut Algorithm> {
        self.algorithms.get_mut(algorithm)
    }
}
