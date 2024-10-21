use std::env;

use mini_grep::Config;
use std::process;
fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(e) = mini_grep::run(config) {
        println!("{}", e);
        process::exit(1);
    };
}
