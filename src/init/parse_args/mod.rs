use std::path::Path;

use clap::{App, Arg, ArgMatches, crate_authors, crate_version, SubCommand};

use parse_settings::parse_settings;
use parse_start::parse_start;

use crate::init::Action;
use crate::init::settings;
use crate::init::settings::{ApiConfig, ConfigFile};

mod parse_settings;
mod parse_start;

const BROKERS: [&str; 1] = ["comdirect"];
const BROKER_REQUIREMENTS: [(&str, &str); 4] = [
    ("comdirect", "key"),
    ("comdirect", "secret"),
    ("comdirect", "username"),
    ("comdirect", "password"),
];
// #[get_algorithms] // TODO: create attribute macro that populates the ALGORITHMS array
// get_algorithms!();
// maybe it's better to use a function like macro that creates the ALGORITHM array from nothing
const ALGORITHMS: [&str; 0] = [];
const SYMBOLS: [&str; 5] = ["USD_EUR", "GOLD", "DAX", "SP500", "NASDAQ"];
const OUTPUT: [&str; 6] = ["text", "chart", "full", "trade", "price", "none"];
const ON_OFF: [&str; 2] = ["on", "off"];


pub fn parse_args() -> Action {
    let matches = clap_parser();

    let mut current_settings = settings::read_configuration()
        .expect("Could not read configuration!");

    match matches.subcommand() {
        ("settings", Some(settings)) => parse_settings(settings, &mut current_settings),
        ("start", Some(start)) => parse_start(start, &mut current_settings),
        _ => Action::Exit
    }
}

fn clap_parser<'a>() -> ArgMatches<'a> {
    App::new("Trading")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A CLI for algorithmic trading\
        \nYou can either use existing algorithms or develop some your own! \
        \nIf your brokers api currently is not supported, please open a issue on GitHub with \
        a link to your brokers API documentation. It would awesome if you could create the rust api \
        for you brokers api your self! Each contribution makes this CLI a great amount better.")

        .subcommand(SubCommand::with_name("settings")
            .about("Gives you the option to change settings")
            .arg(Arg::with_name("show")
                .help("shows the current settings")
                .short("s")
                .long("show")
            )
            .arg(Arg::with_name("load")
                .help("loads settings from a file")
                .short("l")
                .long("load")
                .takes_value(true)
                .validator(|path| {
                    let path = Path::new(&path);
                    if path.is_file() { Ok(()) } else { Err(String::from("load takes a valid path to a file")) }
                })
            )
            .subcommand(SubCommand::with_name("save")
                .about("A CLI for manually changing the save behavior while trading")
                .arg(Arg::with_name("order")
                    .help("Defines the saving behavior when a order is made")
                    .short("o")
                    .long("order")
                    .takes_value(true)
                    .possible_values(&ON_OFF)
                    .default_value("on")
                )
                .arg(Arg::with_name("price")
                    .help("Defines the saving behavior for the price of products you trad")
                    .short("p")
                    .long("price")
                    .takes_value(true)
                    .possible_values(&ON_OFF)
                    .default_value("off")
                )
            )
        )
        .get_matches()
}

/*fn validate_start_output(value: &String) -> Result<(), String> {
    let values = value.split_whitespace();
    let mut text = false;
    let mut chart = false;
    let mut full = false;
    let mut none = false;

    for value in values {
        if value == "text" && !chart { text = true; } else if value == "text" && chart { return Err("text output cannot live side by side with chart".to_string()); } else if value == "chart" && !text { chart = true; } else if value == "chart" && text { return Err("chart output cannot live side by side with text".to_string()); } else if value == "full" && !none { full = true; } else if value == "full" && none { return Err("full output cannot live side by side with none".to_string()); } else if value == "none" && !full { none = true; } else if value == "none" && full { return Err("none output cannot live side by side with full".to_string()); } else if value != "text" && value != "chart" && full { return Err(format!("{} output cannot live side by side with full", value)); } else if value != "text" && value != "chart" && none { return Err(format!("{} output cannot live side by side with none", value)); }
    }
    Ok(())
}*/