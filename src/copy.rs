// external
use clap;
// internal
use super::util;

// Variables
pub const NAME: &str = "copy";

// Methods
pub fn get_subcommand() -> clap::App<'static, 'static> {
    clap::App::new(NAME)
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        // flags
        .arg(util::get_debug_arg())
        // args
        .arg(clap::Arg::with_name("src")
            .required(true))
        .arg(clap::Arg::with_name("dest")
            .required(true))
}

pub fn copy(matches: &clap::ArgMatches) {
    if matches.is_present(util::DEBUG_ARG_NAME) {
        println!("{:#?}", matches);
    }
    let src = matches.value_of("src").unwrap();
    let dest = matches.value_of("dest").unwrap();
    if !util::is_o8_path(src) && !util::is_o8_path(dest) {
        util::error("At least one path must be an o8 path");
    } else {
        println!("Source: {}\nDestination: {}", src, dest);
    }
}
