use std::path::Path;

use clap::{App, Arg, ArgMatches, crate_authors, crate_version, SubCommand};

use crate::init::Action;
use crate::init::settings;
use crate::init::settings::{ConfigFile, ApiConfig};

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

fn parse_settings(args: &ArgMatches, current_settings: &mut ConfigFile) -> Action {
    // lets the user load settings from a different file
    // for doing so the other file needs to be a completely valid settings file
    // we don't have to check if the path is valid. This is already done by a clap validator
    if let Some(path) = args.value_of("load") {
        settings::load_configuration(path)
            .expect("Could not load configuration!");
    }

    let action = match args.subcommand() {
        ("algorithm", Some(algorithm)) => parse_settings_algorithms(&algorithm, &mut *current_settings),
        ("api", Some(api)) => parse_settings_api(&api, &mut *current_settings),
        ("save", Some(save)) => parse_settings_save(&save, &mut *current_settings),
        _ => Action::Exit
    };

    // override the current settings
    // this won't change anything if the settings weren't changed
    settings::write_configuration(&current_settings)
        .expect("Could not update the configuration!");

    if args.is_present("show") {
        print!("{}", current_settings);
    }

    action
}

fn parse_settings_algorithms(args: &ArgMatches, current_settings: &mut ConfigFile) -> Action {
    let mut action = Action::Exit;

    // lets the user change the currently used default algorithm
    if let Some(algorithm_name) = args.value_of("change") {
        // check if there are algorithms defines yet
        // these algorithms must live in the settings::ALGORITHM_DIR
        match &mut current_settings.algorithm_config {
            Some(algorithm_config) => {
                // check if the algorithm is defined
                let mut updated = false;
                for algorithm in algorithm_config.algorithms.iter() {
                    if algorithm.name == algorithm_name {
                        algorithm_config.current_algorithm = Some((*algorithm).clone());
                        updated = true;
                    }
                }

                if !updated {
                    action = Action::Panic(format!("Could not find the algorithm {}!", algorithm_name));
                }
            }
            _ => action = Action::Panic("No algorithms defined yet!".to_string())
        }
    }

    if args.is_present("show") {
        println!("{:#?}", current_settings.algorithm_config);
    }

    action
}

fn parse_settings_api(args: &ArgMatches, current_settings: &mut ConfigFile) -> Action {
    let mut action = Action::Exit;

    // lets the user add a new api
    if let Some(api_name) = args.value_of("add") {
        // check weather or not the broker is supported
        if !BROKERS.contains(&api_name) {
            return Action::Panic(format!("The broker {} is currently not supported!", api_name))
        }

        // get all the access values of the api
        // if a access value is needed the field should be required, so we don't have to check for that
        let key = if let Some(key) = args.value_of("key") { Some(key.to_string()) } else { None };
        let secret = if let Some(secret) = args.value_of("secret") { Some(secret.to_string()) } else { None };
        let username = if let Some(username) = args.value_of("username") { Some(username.to_string()) } else { None };
        let password = if let Some(password) = args.value_of("password") { Some(password.to_string()) } else { None };

        let new_api = settings::BrokerApi {
            name: api_name.to_string(),
            key,
            secret,
            username,
            password,
        };

        // adds the new broker to the settings
        if let Some(ref mut api_config) = current_settings.api_config {
            api_config.apis.push(new_api);
        } else {
            let api_config = ApiConfig {
                current_api: None,
                apis: vec![new_api]
            };
            current_settings.api_config = Some(api_config);
        }
    }

    // lets the user change the currently used default api
    // TODO: currently it's not possible decide witch account to use when multiple of one broker are available
    if let Some(api_name) = args.value_of("change") {
        if let Some(ref mut apis) = current_settings.api_config {
            let mut updated = false;
            for api in apis.apis.iter() {
                if api.name == api_name {
                    apis.current_api = Some(api.clone());
                    updated = true;
                }
            }
            if !updated {
                action = Action::Panic(format!("Could not find the api {}", api_name));
            }
        } else {
            action = Action::Panic("No apis defined yet!".to_string())
        }
    }

    action
}

fn parse_settings_save(_args: &ArgMatches, _current_settings: &mut ConfigFile) -> Action {
    unimplemented!()
}

fn parse_start(_args: &ArgMatches, _current_settings: &mut ConfigFile) -> Action {
    unimplemented!()
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
                .conflicts_with("change")
            )
            .subcommand(SubCommand::with_name("algorithm")
                .about("A CLI for manually changing the algorithm")
                .arg(Arg::with_name("show")
                    .help("shows the available algorithms")
                    .short("s")
                    .long("show")
                )
                .arg(Arg::with_name("change")
                    .help("changes the current used algorithm")
                    .short("c")
                    .long("change")
                    .takes_value(true)
                    .possible_values(&ALGORITHMS)
                )
            )
            .subcommand(SubCommand::with_name("api")
                .about("A CLI for manually changing API settings")
                .arg(Arg::with_name("change")
                    .help("changes the used api")
                    .short("c")
                    .long("change")
                    .takes_value(true)
                )
                .arg(Arg::with_name("add")
                    .help("adds a new api access to the apis")
                    .short("a")
                    .long("add")
                    .takes_value(true)
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

        .subcommand(SubCommand::with_name("start")
            .about("Starts the trading algorithm")
            .arg(Arg::with_name("trading type")
                .help("determine weather you want to trade live (with real money!), paper (without money) or back (back tests you algorithm)")
                .takes_value(true)
                .possible_values(&["live", "paper", "back"])
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
                .possible_values(&SYMBOLS)
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
                .default_value("text trade")
            )
            .arg(Arg::with_name("save")
                .help("Weather or not data like trades should be saved\
                \nThis argument let's you decide how much data should be saved. \
                Please notice that this could have a little performance overhead. Still it's \
                absolutely recommended to save the data. Data saves will be asyncness and can \
                save your butt if one of the algorithms goes crazy. \
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

fn validate_start_output(value: &String) -> Result<(), String> {
    let values = value.split_whitespace();
    let mut text = false;
    let mut chart = false;
    let mut full = false;
    let mut none = false;

    for value in values {
        if value == "text" && !chart { text = true; } else if value == "text" && chart { return Err("text output cannot live side by side with chart".to_string()); } else if value == "chart" && !text { chart = true; } else if value == "chart" && text { return Err("chart output cannot live side by side with text".to_string()); } else if value == "full" && !none { full = true; } else if value == "full" && none { return Err("full output cannot live side by side with none".to_string()); } else if value == "none" && !full { none = true; } else if value == "none" && full { return Err("none output cannot live side by side with full".to_string()); } else if value != "text" && value != "chart" && full { return Err(format!("{} output cannot live side by side with full", value)); } else if value != "text" && value != "chart" && none { return Err(format!("{} output cannot live side by side with none", value)); }
    }
    Ok(())
}
