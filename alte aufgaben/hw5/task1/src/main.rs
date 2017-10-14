extern crate procinfo;
extern crate nix;

mod zombie;
mod child;
use zombie::*;
use child::*;
use std::{process, env};
use nix::unistd::getpid;
mod unit_tests;

fn main() {
    let vec: Vec<String> = env::args().collect();

    match vec.len() {
        1 => {
            run_zombie();
        }
        2 => {
            let a = getpid();
            let ref b = vec[1];
            match run_childs(a, &b) {
                Ok(x) => x,
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }
        _ => {
            println!("Correct usage: no PID");
            process::exit(1);
        }
    }
}
