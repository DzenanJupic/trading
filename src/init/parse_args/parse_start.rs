use clap::ArgMatches;

use crate::init::Action;
use crate::init::settings::ConfigFile;

pub fn parse_start(args: &ArgMatches, current_settings: &mut ConfigFile) -> Action {
    // Action::Start()
    unimplemented!()
}

// fn validate_start_output(values: Values) -> Result<(), String> {
//     let values = value.split_whitespace();
//     let mut text = false;
//     let mut chart = false;
//     let mut full = false;
//     let mut none = false;
//
//     for value in values {
//         if value == "text" && !chart {
//             text = true;
//         } else if value == "text" && chart {
//             return Err("text output cannot live side by side with chart".to_string());
//         } else if value == "chart" && !text {
//             chart = true;
//         } else if value == "chart" && text {
//             return Err("chart output cannot live side by side with text".to_string());
//         } else if value == "full" && !none {
//             full = true;
//         } else if value == "full" && none {
//             return Err("full output cannot live side by side with none".to_string());
//         } else if value == "none" && !full {
//             none = true;
//         } else if value == "none" && full {
//             return Err("none output cannot live side by side with full".to_string());
//         } else if value != "text" && value != "chart" && full {
//             return Err(format!("{} output cannot live side by side with full", value));
//         } else if value != "text" && value != "chart" && none {
//             return Err(format!("{} output cannot live side by side with none", value));
//         }
//     }
//     Ok(())
// }
