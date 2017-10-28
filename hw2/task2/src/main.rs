extern crate task2;
use task2::Config;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    //print_arguments(&args);
    //println!("{:?}", parse_arguments_simple(&args));
    //let config = Config::new(&args);
    match Config::new(&args) {
        Ok(a) => {
            println!("You asked me to count all {} in {}", a.search, a.line);
            println!("Found {} {} in {}", task2::run(&a), a.search, a.line);
        }
        Err(err) => {
            println!("{}", err);
            std::process::exit(1);
        }
    }
}
