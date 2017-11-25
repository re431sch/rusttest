extern crate nix;

mod command;
mod shell;

use std::process;


fn main() {
    let input = b"echo";
    let mut output = Vec::new();
    let mut s = shell::Shell::new(&input[..], &mut output, "shell".into());
    println!("{:?}", s);
    //shell::Shell::prompt(s);
    /*match s.start() {
        Ok(_) => process::exit(0),
        Err(_) => process::exit(1),
    }*/
}
