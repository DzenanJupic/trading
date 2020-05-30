use algorithm_utils::AlgorithmInterface;
use clap::ArgMatches;

use crate::init::{Action, settings};
use crate::init::settings::{ApiConfig, BrokerApi, ConfigFile, Settings};

pub fn parse_settings(args: &ArgMatches, mut current_settings: Settings) -> Action {
    // lets the user load settings from a different file
    // for doing so the other file needs to be a completely valid settings file
    // we don't have to check if the path is valid. This is already done by a clap validator
    // if let Some(path) = args.value_of("load") { // todo
    //     *current_settings = settings::load_configuration(path)
    //         .expect("Could not load configuration!");
    // }

    let action = match args.subcommand() {
        ("save", Some(save)) => parse_save(&save, &mut current_settings),
        ("algorithms", Some(algorithms)) => parse_algorithms(&algorithms, &mut current_settings),
        ("apis", Some(apis)) => parse_apis(&apis, &mut current_settings),
        _ => Action::None
    };

    if args.is_present("show") {
        print!("{}", current_settings);
    }

    // override the current settings
    // this won't change anything if the settings weren't changed
    ConfigFile::from(current_settings)
        .to_config_file()
        .expect("Could not update the configuration!");

    action
}

fn parse_save(args: &ArgMatches, current_settings: &mut Settings) -> Action {
    if let Some(order) = args.value_of("order") {
        current_settings.save_config.order = on_off_to_bool(order);
    }

    if let Some(price) = args.value_of("price") {
        current_settings.save_config.price = on_off_to_bool(price);
    }

    Action::None
}

fn parse_algorithms(args: &ArgMatches, current_settings: &mut Settings) -> Action {
    let mut action = Action::None;

    // lets the user change the currently used default algorithm
    if let Some(algorithm_name) = args.value_of("change") {
        if let Err(_) = current_settings.set_current_algorithm(algorithm_name.to_string()) {
            action = Action::Panic(format!("Could not find the algorithm {}", algorithm_name));
        }
    }

    if let Some(algorithm_name) = args.value_of("about") {
        match current_settings.algorithms() {
            Some(ref algorithms) => {
                match algorithms.get(algorithm_name) {
                    Some(ref algorithm) => println!("{}", algorithm.about()),
                    None => action = Action::Panic(format!("Could not find the algorithm {}", algorithm_name))
                }
            }
            None => action = Action::Panic("No algorithms defined yet".to_string())
        }
    }

    if args.is_present("list") {
        match current_settings.algorithms() {
            Some(ref algorithms) => println!("{}", algorithms),
            None => println!("ALGORITHMS: None")
        }
    }

    action
}

fn parse_apis(args: &ArgMatches, current_settings: &mut Settings) -> Action {
    let mut action = match args.subcommand() {
        ("add", Some(add)) => parse_apis_add(add, current_settings),
        ("remove", Some(remove)) => parse_apis_remove(remove, current_settings),
        _ => Action::None
    };

    // lets the user change the currently used default api
    if let Some(api_id) = args.value_of("change") {
        if let Some(ref mut apis) = current_settings.api_config {
            let mut updated = false;
            for api in apis.apis.iter() {
                if api.id() == api_id {
                    apis.current_api = api.id().clone();
                    updated = true;
                }
            }
            if !updated {
                action = Action::Panic(format!("Could not find the api {}", api_id));
            }
        } else {
            action = Action::Panic("No apis defined yet".to_string())
        }
    }

    if args.is_present("list") {
        let api_config = match current_settings.api_config {
            Some(ref config) => config.to_string(),
            None => String::from("None")
        };
        println!("\n{}", api_config)
    }

    action
}

fn parse_apis_add(args: &ArgMatches, current_settings: &mut Settings) -> Action {
    let id = match args.value_of("id") {
        Some(id) => {
            if BrokerApi::id_exists(&current_settings, &id) {
                return Action::Panic("This id is already in use".to_string());
            }

            Some(id.to_string())
        }
        None => None
    };
    let broker = args
        .value_of("broker")
        .unwrap()
        .to_string();
    let key = args
        .value_of("key")
        .map(|key| key.to_string());
    let secret = args
        .value_of("secret")
        .map(|secret| secret.to_string());
    let username = args
        .value_of("username")
        .map(|username| username.to_string());
    let password = args
        .value_of("password")
        .map(|password| password.to_string());

    let broker_api = BrokerApi::builder(broker)
        .id(id)
        .key(key)
        .secret(secret)
        .username(username)
        .password(password)
        .build(current_settings);

    match &mut current_settings.api_config {
        Some(api_config) => api_config.apis.push(broker_api),
        None => {
            current_settings.api_config = Some(ApiConfig {
                current_api: broker_api.id().clone(),
                apis: vec![broker_api],
            });
        }
    }

    Action::None
}

fn parse_apis_remove(args: &ArgMatches, current_settings: &mut Settings) -> Action {
    let id = args.value_of("id").unwrap();
    let mut none = false;

    if let Some(ref mut api_config) = current_settings.api_config {
        let mut index = None;

        for (i, api) in api_config.apis.iter().enumerate() {
            if api.id() == id {
                index = Some(i);
            }
        }

        if let Some(index) = index {
            if api_config.apis.len() == 1 {
                none = true;
            } else {
                api_config.apis.remove(index);
                api_config.current_api = api_config.apis
                                                   .iter()
                                                   .last()
                                                   .unwrap()
                                                   .id().clone();
            }
        } else {
            return Action::Panic(format!("could not find {}", id));
        }
    } else { return Action::Panic("no saved apis to remove".to_string()); }

    if none {
        current_settings.api_config = None;
    }

    Action::None
}

fn on_off_to_bool(value: &str) -> bool {
    if value == "on" { true } else if value == "off" { false } else { unreachable!("tried to convert {} to bool", value); }
}