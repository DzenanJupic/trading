use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs::read_dir;
use std::path::Path;

use libloading::Library;

use crate::{Error, ErrorKind};
use crate::load::algorithm::Algorithm;
use std::fmt;
use std::fmt::Formatter;

type AlgorithmRegistration = *mut crate::algorithm_registration::AlgorithmRegistration;

#[cfg(target_os = "windows")]
const DYL_EXTENSION: &str = "dll";
#[cfg(target_os = "linux")]
#[cfg(target_os = "macos")]
const DYL_EXTENSION: &str = "so";

#[derive(Default)]
pub struct Algorithms {
    algorithms: HashMap<&'static str, Algorithm>,
}

impl Algorithms {
    pub fn empty() -> Self {
        Self {
            algorithms: HashMap::new()
        }
    }

    /// loads all algorithms of a directory
    ///
    /// This method provides a convenient way to search for all dynamically loaded library's in that
    /// directory. It searches for these library's in a specific pattern:
    ///     1. All dynamically loaded library's directly places inside the directory (./)
    ///     2. All dynamically loaded library's in the release directory of a crate (./<crate>/target/release/)
    ///     3. All dynamically loaded library's in the debug directory of a crate (./<crate>/target/debug/)
    ///
    /// Library's compiled in debug mode cannot be loaded
    /// Duplicated algorithm names will be ignored
    pub fn load_all<P: AsRef<Path>>(&mut self, path: P) -> Result<(), Error> {
        for entry in read_dir(path)? {
            if let Ok(entry) = entry {
                let path = entry.path();

                if path.is_dir() {
                    let release_path = path.join("target/release");
                    let debug_path = path.join("target/debug");

                    let target_path = if release_path.is_dir() {
                        release_path
                    } else if debug_path.is_dir() {
                        debug_path
                    } else {
                        return Err(Error::new(format!("No algorithm in this directory {:?}", entry.path().file_name().unwrap()), ErrorKind::IO));
                    };

                    for entry in read_dir(target_path)? {
                        if let Ok(entry) = entry {
                            let path = entry.path();

                            if path.is_file() {
                                if let Some(extension) = path.extension() {
                                    if extension == DYL_EXTENSION {
                                        self.load(path)?;
                                    }
                                }
                            }
                        }
                    }
                } else if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == DYL_EXTENSION {
                            self.load(path)?;
                        }
                    }
                } else { unreachable!("Path is neither directory nor file") }
            }
        }

        Ok(())
    }

    /// loads a algorithm by path
    ///
    /// This method requires the path to lead to a dynamically loaded library. There's no searching
    /// happening in the background.
    ///
    /// Library's compiled in debug mode cannot be loaded
    /// Duplicated algorithm names will be ignored
    pub fn load<P: AsRef<OsStr>>(&mut self, path: P) -> Result<(), Error> {
        let lib = Library::new(path.as_ref().clone())?;

        let algorithm_registration;
        unsafe {
            algorithm_registration = lib
                .get::<AlgorithmRegistration>(b"ALGORITHM_REGISTRATION\0")? // todo: maybe \0 at the end?
                .read();
        }

        if algorithm_registration.rustc_version != crate::RUSTC_VERSION
            || algorithm_registration.utils_version != crate::UTILS_VERSION {
            return Err(Error::new(
                format!(
                    "The algorithm `{}` has a mismatched version!\n\
                    Algorithm version: [{}/{}]\nUtils version: [{}/{}]",
                    algorithm_registration.name,
                    algorithm_registration.rustc_version,
                    algorithm_registration.utils_version,
                    crate::RUSTC_VERSION,
                    crate::UTILS_VERSION
                ),
                ErrorKind::MisMatchedVersion,
            ));
        }

        let algorithm_name = algorithm_registration.name;
        if self.get(&algorithm_name).is_some() {
            return Err(Error::new("Algorithm already exists".to_string(), ErrorKind::LibLoading));
        }

        // call the initial_algorithm_state_fn which returns the initial algorithm state
        let initial_algorithm_state = unsafe { (algorithm_registration.initial_algorithm_state_fn)() };

        let algorithm = Algorithm::new(
            algorithm_name,
            path
                .as_ref()
                .to_str()
                .expect("path contained unexpected characters")
                .to_string(),
            initial_algorithm_state,
            lib,
        );

        self.algorithms.insert(
            algorithm_name,
            algorithm,
        );

        Ok(())
    }

    /// returns a reference to a `Algorithm`
    /// can be used to trade using this algorithm
    pub fn get(&self, algorithm: &str) -> Option<&Algorithm> {
        self.algorithms.get(algorithm)
    }

    /// returns a mutable reference to a `Algorithm`
    /// can be used to trade using this algorithm
    pub fn get_mut(&mut self, algorithm: &str) -> Option<&mut Algorithm> {
        self.algorithms.get_mut(algorithm)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.get(name).is_some()
    }
}

impl fmt::Display for Algorithms {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let algorithms = self.algorithms
            .iter()
            .fold(
                String::from("\t"),
                |mut prev, (_ , curr)| {
                    prev.push_str("\n\t");
                    prev.push_str(&curr.to_string());
                    prev
                }
            );

        write!(formatter, "ALGORITHMS:{}", algorithms)
    }
}
