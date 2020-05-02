use std::time::Duration;

mod parse_args;
mod settings;

pub fn init() -> Action {
    let parse_args = parse_args::parse_args();

    if parse_args.is_exit() || parse_args.is_none() {
        std::process::exit(0);
    } else if let Action::Panic(msg) = parse_args {
        eprintln!("{}", msg);
        std::process::exit(1);
    }

    parse_args
}

#[allow(unused)] // todo
pub struct Start {
    isins: Vec<String>,
    workers: u32,
    interval: Duration,
    api: settings::ApiConfig,
    algorithms: Vec<settings::AlgorithmConfig>,
}

#[allow(unused)] // todo
pub enum Action {
    Start(Start),
    Exit,
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