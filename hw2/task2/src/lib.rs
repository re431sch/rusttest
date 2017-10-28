#[derive(Debug,PartialEq)]
pub struct Config {
    pub search: char,
    pub line: String,
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, &'static str >{
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        match args[1].chars().next() {
            Some(n) => {
                let config = Config {
                    search: n,
                    line: args[2].to_string().clone(),
                };
                Ok(config)
            },
            _ => Err("char mismatch"),
        }
    }
}

pub fn run(config: &Config) -> u64 {
    let mut count = 0;
    let s = config.search;
    let l = config.line.clone();
    for i in l.chars() {
        if i == s {
            count += 1
        }
    }
    count
}

#[allow(dead_code)]
fn print_arguments(args: &Vec<String>) {
    for i in 0 .. args.len() {
        println!("{}", args[i]);
    }
}

#[allow(dead_code)]
fn parse_arguments_simple(args: &Vec<String>) -> Config {
    let config = Config {
        search: args[1].chars().next().unwrap(),
        line: args[2].to_string().clone(),
    };
    config
}
