use clap;

// Variables
pub const NAME: &str = "copy";

// Methods
pub fn get_subcommand() -> clap::App<'static, 'static> {
    clap::App::new(NAME)
        .arg(clap::Arg::with_name("debug")
            .short("d"))
}

pub fn copy(matches: &clap::ArgMatches) {
    if matches.is_present("debug") {
        println!("{:#?}", matches);
    }
}
