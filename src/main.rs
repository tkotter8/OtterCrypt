use clap;

mod copy;
mod util;

fn main() {
    // build app
    let app = clap::App::new("Otter Crypt")
        .author("Tyler Kropiewnicki <tkotter8@gmail.com>")
        .version("0.1")
        .setting(clap::AppSettings::ArgRequiredElseHelp)
        // arguments
        .arg(util::get_debug_arg())
        // subcommands
        .subcommand(copy::get_subcommand());

    // parse arguments
    let matches = app.get_matches();

    // handle arguments
    if matches.is_present(util::DEBUG_ARG_NAME) {
        println!("{:#?}", matches);
    }

    // handle subcommands
    match matches.subcommand() {
        (copy::NAME, Some(subcommand)) => copy::copy(subcommand),
        _ => panic!("How did you get here?"),
    }
}
