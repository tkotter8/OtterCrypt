use clap;

// Variables
pub const DEBUG_ARG_NAME: &str = "debug";

// Methods
pub fn get_debug_arg() -> clap::Arg<'static, 'static> {
    clap::Arg::with_name(DEBUG_ARG_NAME)
        .help("Prints debug information")
        .long(DEBUG_ARG_NAME)
        .short("d")
}
