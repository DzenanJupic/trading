use clap::{App, Arg, SubCommand, ArgMatches};

pub fn clap_parser() -> ArgMatches {
    App::new("Trading")
        .about("A CLI that provides a huge range of trading related features like \
        algorithmic trading, live trading, price checks and account management.\
        \nIf your banks api currently is not supported, please open a issue on GitHub. I'll\
        try my best to implement it as fast as possible.")
        .subcommand(SubCommand::with_name("settings")
            .about("A CLI for manually change settings")
            .arg(Arg::with_name("show")
                .help("shows the current settings")
                .short("s")
            )
            .arg(Arg::with_name("change")
                .help("changes a current setting value [pattern: <setting>:<value> ]")
                .short("c")
                .takes_value(true)
                .multiple(true)
                .require_delimiter(true)
                .value_delimiter(":")
            )
            .arg(Arg::with_name("load")
                .help("loads settings from a file")
                .short("l")
                .takes_value(true)
                .validator(|path| {
                    let path = std::path::Path::new(&path);
                    if path.is_file() { Ok(()) }
                    else { Err(String::from("load takes a valid path to a file")) }
                })
                .conflicts_with_all(&["change"])
            )
            .subcommand(SubCommand::with_name("api")
                .about("A CLI for manually changing API settings")
                .arg(Arg::with_name("change")
                    .help("changes the used api")
                    .short("c")
                    .takes_value(true)
                )
                .arg(Arg::with_name("add")
                    .help("adds a new api access to the apis")
                    .short("a")
                    .takes_value(true)
                    .possible_values(&["comdirect"])
                    .requires_ifs(&[
                        ("comdirect", "key"),
                        ("comdirect", "secret"),
                        ("comdirect", "username"),
                        ("comdirect", "password"),
                    ])
                )
                .arg(Arg::with_name("key")
                    .help("sets the api key")
                    .short("k")
                    .takes_value(true)
                    .env("API_KEY")
                )
                .arg(Arg::with_name("secret")
                    .help("sets the api secret")
                    .short("s")
                    .takes_value(true)
                    .env("API_SECRET")
                )
                .arg(Arg::with_name("username")
                    .help("sets the api username")
                    .short("u")
                    .takes_value(true)
                    .env("API_USERNAME")
                )
                .arg(Arg::with_name("password")
                    .help("sets the api password")
                    .short("p")
                    .takes_value(true)
                    .env("API_PASSWORD")
                )
            )
            .subcommand(SubCommand::with_name("save")
                .about("A CLI for manually changing the save behavior while trading")
            )
        )
        .subcommand(SubCommand::with_name("start")
            .about("A CLI for algorithmic trading")
            .help("starts the trading algorithm")
            .arg(Arg::with_name("live_trading")
                .help("enables live trading (with real money!)")
            )
            .arg(Arg::with_name("back_testing")
                .help("let's you check if your algorithm would have worked in the past")
                .conflicts_with("live")
            )
            .arg(Arg::with_name("paper_trading")
                .help("let's you trade without real money [on by default]")
                .long_help("Let's you trade without real money\
                \nThis option gives you the ability to try you algorithm in real time. Notice \
                that the results could be slightly different than in reality. This is due to the \
                fact that it's not possible to simulate the latency of the internet and you broker \
                perfectly. Also service and order fees charged by your broker could be slightly \
                different. You can set these values in the settings. Over time the simulated \
                latency and fee will adapt to the averages when trading with real money.")
                .conflicts_with("live")
                .conflicts_with("back_test")
            )
            .arg(Arg::with_name("isin")
                .help("the ISIN of the product you want to trade")
                .short("i")
                .takes_value(true)
                .required(true)
            )
            .arg(Arg::with_name("wkn")
                .help("the WKN of the product you want to trade")
                .short("w")
                .takes_value(true)
                .required(true)
                .conflicts_with("isin")
            )
            .arg(Arg::with_name("symbol")
                .help("the symbol of the product you want to trade")
                .short("s")
                .takes_value(true)
                .required(true)
                .conflicts_with("isin")
                .conflicts_with("wkn")
            )
            .arg(Arg::with_name("chart")
                .help("Weather or not a chart should be displayed")
                .long_help("Weather or not a chart should be displayed\
                \nIf this argument is set a gui will open and a chart will be displayed.\
                Notice that this can lower the performance drastically! Especially when trading with \
                algorithms you should stick to bare text output. You always have the option to look \
                at the charts later on if you save the trades.")
                .short("c")
            )
            .arg(Arg::with_name("text")
                .help("Weather or not text based output should be displayed [on by default]")
                .short("t")
                .conflicts_with("chart")
            )
            .arg(Arg::with_name("output")
                .help("Specifies the amount of date that should be displayed [default: trades]")
                .long_help("Specifies the amount of date that should be displayed [default: trades]\
                \nThis argument let's you decide what amount of information should be displayed \
                while trading. Outputting  data will lead to a performance overhead. Still it's \
                recommended to output the trades, since this gives you the ability to check if \
                the algorithm goes crazy. full and none can't be specified together! \
                If full or none is set it overrides all other values. \
                Notice that you always have the option to look at the data afterwards if you save it.")
                .short("o")
                .takes_value(true)
                .multiple(true)
                .possible_values(&["full", "chart", "trades", "none"])
                .default_value("trades")
            )
            .arg(Arg::with_name("save")
                .help("Weather or not data like trades should be saved [default: trades]")
                .long_help("Weather or not data like trades should be saved [default: trades]\
                \nThis argument let's you decide how much data should be saved. \
                Please notice that this could have a little performance overhead. Still it's \
                absolutely recommended to save the data. Data saves will be asyncness and can \
                save your butt if one of the algorithms goes crazy. \
                Usually it also shouldn't be necessary to save the charts, since you can pull \
                them from the internet later.")
                .short("s")
                .takes_value(true)
                .multiple(true)
                .possible_values(&["full", "chart", "trades", "none"])
                .default_value("trades")
            )
        )
        .get_matches()
}