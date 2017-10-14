use std::env;
use std::process;

#[derive(Debug,PartialEq)]
struct Config {
    search: char,
    line: String,
}

fn main() {
    let vec: Vec<String> = env::args().collect();
    //print_arguments(&vec);
    match parse_arguments(&vec) {
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

fn run(y: &Config) -> u64 {
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
fn print_arguments(vec: &Vec<String>) {
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


fn parse_arguments(vec: &Vec<String>) -> Result<Config, &'static str> {
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

#[test]
fn test_parse_config_simple() {
    let a = vec!["Not interested".to_string(),
                 "e".to_string(),
                 "Numero Due".to_string()];
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(parse_arguments_simple(&a), c);
}

#[test]
fn test_parse_config_1() {
    let a = vec!["Not interested".to_string(),
                 "e".to_string(),
                 "Numero Due".to_string()];
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(parse_arguments(&a), Ok(c));
}

#[test]
#[should_panic]
fn test_parse_config_2() {
    let a = vec!["Not interested".to_string(),
                 "x".to_string(),
                 "Numero Due".to_string()];
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(parse_arguments(&a), Ok(c));
}

#[test]
fn test_parse_config_3() {
    let a = vec!["Not interested".to_string(),
                 "0".to_string(),
                 "0".to_string()];
    let c = Config {
        search: '0',
        line: "0".to_string(),
    };
    assert_eq!(parse_arguments(&a), Ok(c));
}

#[test]
fn test_parse_config_err_1() {
    let a = vec!["Not interested".to_string(), "e".to_string()];
    assert_eq!(parse_arguments(&a), Err("not enough arguments"));
}

#[test]
fn test_parse_config_err_2() {
    let a = vec!["Not interested".to_string()];
    assert_eq!(parse_arguments(&a), Err("not enough arguments"));
}


#[test]
fn test_run_1() {
    let c = Config {
        search: 'e',
        line: "Numero Due".to_string(),
    };
    assert_eq!(run(&c), 2);
}

#[test]
fn test_run_2() {
    let c = Config {
        search: '♥',
        line: "♥ The quick brown fox jumps over the lazy dog. ♥".to_string(),
    };
    assert_eq!(run(&c), 2);
}

#[test]
#[should_panic]
fn test_run_3() {
    let c = Config {
        search: 'q',
        line: "♥ The quick brown fox jumps over the lazy dog. ♥".to_string(),
    };
    assert_eq!(run(&c), 2);
}

#[test]
fn test_run_4() {
    let c = Config {
        search: '!',
        line: "♥ The quick brown fox jumps over the lazy dog. ♥".to_string(),
    };
    assert_eq!(run(&c), 0);
}
