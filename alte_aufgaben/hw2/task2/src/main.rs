mod lib;
use lib::*;
use std::*;

fn main() {
    let vec: Vec<String> = env::args().collect();
    match lib::Config::new(&vec) {
        Ok(xy) => {
            println!("You asked me to count all '{}' in '{}'", xy.search, xy.line);
            println!("Found {} '{}' in '{}'", run(&xy), xy.search, xy.line);
        }
        Err(err) => {
            println!("{}", err);
            process::exit(1)
        }
    }
}
