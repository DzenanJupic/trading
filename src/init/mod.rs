use std::time::Duration;

use algorithm_utils::Derivative;

mod parse_args;
mod settings;

pub fn init() -> Action {
    let parse_args = parse_args::parse_args();

    match parse_args {
        Action::Exit(msg) => {
            println!("\n{}", msg);
            std::process::exit(0);
        }
        Action::Panic(msg) => {
            eprintln!("\n{}", msg);
            std::process::exit(1);
        }
        Action::None => std::process::exit(0),
        Action::Start(_) => return parse_args
    }
}

#[allow(unused)] // todo
pub struct Start {
    isins: Vec<Derivative>,
    interval: Duration,
    api: settings::ApiConfig,
    algorithm: String,
}

#[allow(unused)] // todo
pub enum Action {
    Start(Start),
    Exit(String),
    None,
    Panic(String),
}

#[allow(unused)]
impl Action {
    pub fn is_none(&self) -> bool {
        if let Action::None = self {
            true
        } else { false }
    }
    pub fn is_exit(&self) -> bool {
        if let Action::Exit(_) = self {
            true
        } else { false }
    }
    pub fn is_panic(&self) -> bool {
        if let Action::Panic(_) = self {
            true
        } else { false }
    }
}