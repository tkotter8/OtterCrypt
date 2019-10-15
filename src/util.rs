// external
use clap;
use colored::*;

// Variables
pub const DEBUG_ARG_NAME: &str = "debug";

// Methods
pub fn get_debug_arg() -> clap::Arg<'static, 'static> {
    clap::Arg::with_name(DEBUG_ARG_NAME)
        .help("Prints debug information")
        .long(DEBUG_ARG_NAME)
        .short("d")
}

pub fn is_o8_path(path: &str) -> bool {
    if path.len() < 4 {
        return false;
    }
    &path.to_lowercase()[0..5] == "o8://"
}

pub fn error(msg: &str) {
    println!("{} {}", "error:".bright_red().bold(), msg);
}
