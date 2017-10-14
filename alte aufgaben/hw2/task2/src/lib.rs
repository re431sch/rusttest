#[derive(Debug,PartialEq)]
pub struct Config {
    pub search: char,
    pub line: String,
}

pub fn run(y: &Config) -> u64 {
    let line = y.line.clone();
    let c = y.search;
    let mut i = 0;
    for x in line.chars() {
        if x == c {
            i += 1;
        }
    }
    i
}

#[allow(dead_code)]
pub fn print_arguments(vec: &Vec<String>) {
    for argument in vec {
        println!("args found: {}", argument);
    }
}

#[allow(dead_code)]
fn parse_arguments_simple(vec: &Vec<String>) -> Config {
    let x = vec[1].chars().next().expect("no value.");
    let y = vec[2].to_string();
    let xy = Config { search: x, line: y };
    xy
}

impl Config {
    pub fn new(vec: &Vec<String>) -> Result<Config, &'static str> {
        if vec.len() < 3 {
            return Err("not enough arguments");
        }
        match vec[1].as_str().chars().nth(0) {
            Some(n) => {
                let config = Config {
                    search: n,
                    line: vec[2].to_string(),
                };
                Ok(config)
            }
            _ => Err("char mismatch"),
        }
    }
}
