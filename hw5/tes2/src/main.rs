extern crate nix;
extern crate procinfo;

use nix::unistd::{fork,getpid,execv};
use nix::unistd::ForkResult::{Child, Parent};
use std::process::Command;
use nix::sys::wait::wait;
use nix::sys::wait::waitpid;
use nix::libc::pid_t;
use procinfo::pid::stat_self;

fn main() {
    /*match self_pids() {
        Ok(b) => next(b, 4),
        Err(e) => println!("{}", e),
    }*/
    //next(5);
    match stat_self() {
        Ok(a) => {
            let b = a.pid;
            println!("{}",b);
        }
        Err(_) => println!("sf"),
    }

}

/*fn next(mut a: i32) {

    match  fork() {
        Ok(Child) => {
            println!("hello, I am child (pid:{})", getpid());
            if a > 1 {
                a = a -1;
                next(a);
            }
        }
        Ok(Parent {child}) => {
            wait();
            println!("I am {} and my child is {}.", getpid(), child);

        }
        Err(_) => panic!("fork failed"),
    }
}

pub fn self_pids() -> Result<i32, &'static str> {
    match stat_self().ok() {
        Some(b) => {
            let a = b.pid;
            Ok(a)
        }
        _ => Err("Keine PID oder PPID"),
    }
}*/
