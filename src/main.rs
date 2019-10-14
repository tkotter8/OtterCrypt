use clap;

mod copy;

fn main() {
    // build app
    let app = clap::App::new("otr");
    // parse args
    let matches = app.get_matches();
    // test
    println!("{:?}", matches);
    println!("{}", copy::test());
}
