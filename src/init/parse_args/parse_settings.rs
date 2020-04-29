use clap::ArgMatches;

use crate::init::{Action, settings};
use crate::init::settings::{ApiConfig, ConfigFile};

use super::BROKERS;

pub fn parse_settings(args: &ArgMatches, current_settings: &mut ConfigFile) -> Action {
    // lets the user load settings from a different file
    // for doing so the other file needs to be a completely valid settings file
    // we don't have to check if the path is valid. This is already done by a clap validator
    if let Some(path) = args.value_of("load") {
        *current_settings = settings::load_configuration(path)
            .expect("Could not load configuration!");
    }

    let action = match args.subcommand() {
        ("save", Some(save)) => parse_settings_save(&save, &mut *current_settings),
        ("algorithm", Some(algorithm)) => parse_settings_algorithms(&algorithm, &mut *current_settings),
        ("api", Some(api)) => parse_settings_api(&api, &mut *current_settings),
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

fn parse_settings_save(args: &ArgMatches, current_settings: &mut ConfigFile) -> Action {
    if let Some(order) = args.value_of("order") {
        current_settings.save_config.order = on_off_to_bool(order);
    }

    if let Some(price) = args.value_of("price") {
        current_settings.save_config.price = on_off_to_bool(price);
    }

    Action::None
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
            return Action::Panic(format!("The broker {} is currently not supported!", api_name));
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
                apis: vec![new_api],
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

fn on_off_to_bool(value: &str) -> bool {
    if value == "on" { true } else if value == "off" { false } else { unreachable!("tried to convert {} to bool", value); }
}