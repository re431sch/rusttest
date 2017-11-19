extern crate nix;
extern crate procinfo;

mod zombie;
mod child;
use procinfo::pid::stat_self;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        1 => zombie::run_zombie(),
        2 => {
            match stat_self() {
                Ok(a) => {
                    let ref arg = args[1];
                    let pid = a.pid;
                    match child::run_childs(pid, &arg) {
                        Ok(a) => a,
                        Err(e) => {
                            println!("{}", e);
                            std::process::exit(1);
                        }
                    }
                }
                Err(_) => println!("sf"),
            }
        }
        _ => println!("fail"),
    }
}
