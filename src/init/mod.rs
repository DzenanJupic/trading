use std::time::Duration;

mod parse_args;
mod settings;

pub fn init() -> Action {
    let parse_args = parse_args::parse_args();

    if parse_args.is_exit() || parse_args.is_none() {
        std::process::exit(0);
    } else if let Action::Panic(msg) = parse_args {
        panic!(msg);
    }

    parse_args
}

pub struct Start {
    isins: Vec<String>,
    workers: u32,
    interval: Duration,
    api: settings::ApiConfig,
    algorithms: Vec<settings::AlgorithmConfig>,
}

pub enum Action {
    Start(Start),
    Exit,
    None,
    Panic(String),
}

impl Action {
    pub fn is_none(&self) -> bool {
        if let Action::None = self {
            true
        } else { false }
    }
    pub fn is_exit(&self) -> bool {
        if let Action::Exit = self {
            true
        } else { false }
    }
    pub fn is_panic(&self) -> bool {
        if let Action::Panic(_) = self {
            true
        } else { false }
    }
}