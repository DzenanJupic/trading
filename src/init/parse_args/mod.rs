#![allow(unused)]

use std::path::Path;

use algorithm_utils::{AlgorithmInterface, Error, load::Algorithms};
use clap::{App, Arg, ArgMatches, crate_authors, crate_version, SubCommand};

use parse_settings::parse_settings;
use parse_start::parse_start;

use crate::init::Action;
use crate::init::settings;
use crate::init::settings::{ConfigFile, Settings};

mod parse_settings;
mod parse_start;

pub const ALGORITHM_DIR: &str = "./algorithms/";
const BROKERS: [&str; 1] = ["comdirect"];
const BROKER_REQUIREMENTS: [(&str, &str); 4] = [
    ("comdirect", "key"),
    ("comdirect", "secret"),
    ("comdirect", "username"),
    ("comdirect", "password"),
];
const TRADING_TYPES: [&str; 3] = ["live", "paper", "back"];
const OUTPUT: [&str; 6] = ["text", "chart", "full", "trade", "price", "none"];
const ON_OFF: [&str; 2] = ["on", "off"];


pub fn parse_args() -> Action {
    let matches = clap_parser();

    let mut current_settings: Settings = ConfigFile::from_config_file()
        .expect("Could not read configuration!")
        .into();

    let algorithms = match load_algorithms() {
        Ok(algorithms) => algorithms,
        Err(err) => return Action::Panic(err.msg().to_string()),
    };

    current_settings.set_algorithms(algorithms);

    match matches.subcommand() {
        ("settings", Some(settings)) => parse_settings(settings, current_settings),
        ("start", Some(start)) => parse_start(start, current_settings),
        _ => Action::Exit
    }
}

pub fn load_algorithms() -> Result<Algorithms, Error> {
    let mut algorithms = Algorithms::empty();
    algorithms.load_all(ALGORITHM_DIR)?;
    Ok(algorithms)
}

fn clap_parser<'a>() -> ArgMatches<'a> {
    App::new("Trading")
        .version(crate_version!())
        .author(crate_authors!())
        .about("A CLI for algorithmic trading\
        \nYou can either use existing parse-algorithms or develop some your own! \
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
            .arg(Arg::with_name("load") // TODO: allow partial load (also of broken files | `repair` maybe own argument?)
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
            .subcommand(SubCommand::with_name("algorithms")
                .about("A CLI for manually changing the algorithm")
                .arg(Arg::with_name("list")
                    .help("shows the available parse-algorithms")
                    .short("l")
                    .long("list")
                )
                .arg(Arg::with_name("change")
                    .help("changes the current used algorithm")
                    .short("c")
                    .long("change")
                    .takes_value(true)
                )
                .arg(Arg::with_name("about")
                    .help("Gives access to the algorithm descriptions")
                    .short("a")
                    .long("about")
                    .takes_value(true)
                )
            )
            .subcommand(SubCommand::with_name("apis") // TODO: load to load from different folders
                .about("A CLI for manually changing API settings")
                .arg(Arg::with_name("list")
                    .help("shows the available apis")
                    .short("l")
                    .long("list")
                )
                .arg(Arg::with_name("change")
                    .help("changes the used api based on the id")
                    .short("c")
                    .long("change")
                    .takes_value(true)
                )
                .subcommand(SubCommand::with_name("add")
                    .about("adds a new api access to the apis")
                    .arg(Arg::with_name("id")
                        .help("sets the api id")
                        .short("i")
                        .long("id")
                        .takes_value(true)
                        .env("API_ID")
                    )
                    .arg(Arg::with_name("broker")
                        .takes_value(true)
                        .env("API_BROKER")
                        .required(true)
                        .possible_values(&BROKERS)
                        .requires_ifs(&BROKER_REQUIREMENTS)
                    )
                    .arg(Arg::with_name("key")
                        .help("sets the api key")
                        .short("k")
                        .long("key")
                        .takes_value(true)
                        .env("API_KEY")
                    )
                    .arg(Arg::with_name("secret")
                        .help("sets the api secret")
                        .short("s")
                        .long("secret")
                        .takes_value(true)
                        .env("API_SECRET")
                    )
                    .arg(Arg::with_name("username")
                        .help("sets the api username")
                        .short("u")
                        .long("username")
                        .takes_value(true)
                        .env("API_USERNAME")
                    )
                    .arg(Arg::with_name("password")
                        .help("sets the api password")
                        .short("p")
                        .long("password")
                        .takes_value(true)
                        .env("API_PASSWORD")
                    )
                )
                .subcommand(SubCommand::with_name("remove")
                    .about("removes a existing api")
                    .arg(Arg::with_name("id")
                        .help("ID of the api to remove")
                        .takes_value(true)
                        .required(true)
                    )
                )
            )
        )
        .subcommand(SubCommand::with_name("start")
            .about("Starts the trading algorithm")
            .arg(Arg::with_name("trading type")
                .help("determine weather you want to trade live (with real money!), paper (without money) or back (back tests you algorithm)")
                .takes_value(true)
                .possible_values(&TRADING_TYPES)
            )
            .arg(Arg::with_name("ISIN")
                .help("the ISIN of the product you want to trade")
                .short("i")
                .long("isin")
                .takes_value(true)
                .required_unless_one(&["WKN", "SYMBOL"])
                .validator(|value| {
                    if value.len() == 12 { Ok(()) } else { Err("ISIN needs to be 12 chars long!".to_string()) }
                })
            )
            .arg(Arg::with_name("SYMBOL")
                .help("the symbol of the product you want to trade")
                .short("s")
                .long("symbol")
                .takes_value(true)
                .required_unless("ISIN")
                .conflicts_with("ISIN")
            )
            .arg(Arg::with_name("output")
                .help("Specifies the amount of date that should be displayed [default: trades]\
                \nThis argument let's you decide what amount of information should be displayed \
                while trading. Outputting  data will lead to a performance overhead. Still it's \
                recommended to output the trades, since this gives you the ability to check if \
                the algorithm goes crazy. full and none can't be specified together! \
                If full or none is set it overrides all other values. \
                Notice that you always have the option to look at the data afterwards if you save it.")
                .short("o")
                .long("output")
                .takes_value(true)
                .multiple(true)
                .possible_values(&OUTPUT)
                .default_value("text")
                .default_value("trade")
            )
            .arg(Arg::with_name("save")
                .help("Weather or not data like trades should be saved\
                \nThis argument let's you decide how much data should be saved. \
                Please notice that this could have a little performance overhead. Still it's \
                absolutely recommended to save the data. Data saves will be asyncness and can \
                save your butt if one of the parse-algorithms goes crazy. \
                Usually it also shouldn't be necessary to save the charts, since you can pull \
                them from the internet later.")
                .long("save")
                .takes_value(true)
                .multiple(true)
                .possible_values(&OUTPUT)
                .default_value("trade")
            )
        )
        .get_matches()
}
