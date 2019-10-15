// external
use clap;
// internal
mod copy;
mod util;

fn main() {
    // build app
    let app = clap::App::new("Otter Crypt")
        .author("Tyler Kropiewnicki <tkotter8@gmail.com>")
        .version("0.1")
        .setting(clap::AppSettings::SubcommandRequiredElseHelp)
        // flags
        .arg(util::get_debug_arg())
        // subcommands
        .subcommand(copy::get_subcommand());

    // parse arguments
    let matches = app.get_matches();

    // handle arguments
    if matches.is_present(util::DEBUG_ARG_NAME) {
        println!("{:#?}", matches);
    }
    match matches.subcommand() {
        (copy::NAME, Some(matches)) => copy::copy(matches),
        _ => panic!("How did you get here?"),
    }
}
