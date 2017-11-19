extern crate nix;
extern crate procinfo;

use nix::unistd::{fork,getpid,execv};
use nix::unistd::ForkResult::{Child, Parent};
use std::process::Command;


fn main() {

    let args: Vec<String> = std::env::args().collect();

    run_zombie();
}

//---------------------------------------------------------------------------------------run_zombie


pub fn run_zombie() {
    match  fork() {
        Ok(Child) => {
            //println!("Child: {}", getpid());
            let ten_millis = std::time::Duration::from_millis(1);
            std::thread::sleep(ten_millis);
        }
        Ok(Parent {child}) => {

            //println!("Parent of: {} mit PID: {}", child, getpid());
            Command::new("ps").arg("t").status().expect("fauilded");

        }
        Err(_) => panic!("fork failed"),
    }
}
