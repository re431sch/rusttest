extern crate nix;
mod command;
use command::Command;
use std::str::FromStr;

use std::{env, process};
use std::io::prelude::*;
use std::io;
use nix::unistd::{execvp, fork};
use nix::unistd::ForkResult::{Parent, Child};
use nix::sys::wait::{waitpid, WaitStatus};


struct Shell {
    /// Whether the shell should exit its run loop at the next iteration.
    should_exit: bool,
}


impl Shell {
    /// Create a new `Shell`.
    fn new() -> Shell {
        Shell { should_exit: false }
    }

    /// At beginning of every loop print prompt and wait for input
    fn prompt(&self) -> Option<String> {
        let mut buf = String::new();

        print!("bsys-shell {}$ ", env::current_dir().unwrap().display());
        io::stdout().flush().unwrap();
        let n = io::stdin().read_line(&mut buf).unwrap();

        if n == 0 {
            // EOF.
            None
        } else {
            // snip the trailing newline on an owned String.
            let len = buf.trim_right().len();
            buf.truncate(len);
            Some(buf)
        }
    }
    /// looop function starts the shell and checks whether its an exec, empty or exit command.
    pub fn looop(mut self) -> Result<(), &'static str> {
        while !self.should_exit {
            match self.prompt() {
                Some(y) => {
                    match FromStr::from_str(&y) {
                        Ok(Command::Empty) => {
                            return Ok(());
                        }
                        Ok(Command::Exit) => {
                            self.should_exit = true;
                            println!("exit");
                            return Ok(());
                        }
                        Ok(Command::Exec { prog, argv }) => {
                            match fork() {
                                Ok(Child) => {
                                    match execvp(&prog, &argv) {
                                        Ok(_) => {
                                           process::exit(0)
                                        }
                                        Err(_) => {
                                            process::exit(1)
                                        }
                                    }
                                }
                                Ok(Parent { child }) => {
                                    match waitpid(child, Option::None) {
                                        Ok(WaitStatus::Exited(_, 0)) => {
                                            return Ok(());
                                        }
                                        Ok(WaitStatus::Exited(_, 1)) => {
                                            return Err("Error");
                                        }
                                        _ => { 
                                            return Err("Error");
                                        }
                                    }
                                }
                                Err(_) => {
                                    return Err("Error");
                                }
                            }
                        }
                        Err(_) => {
                            println!("Error");
                            process::exit(1);
                        }
                    }
                }
                None => {
                    println!("Error");
                    process::exit(1);
                }
            }
        }
        Ok(())
    }    
}

/// main executing loop function
fn main() {
    let s = Shell::new();
    // now s.loop or whatever ...
    match s.looop() {
        Ok(_) => {
            process::exit(0);
        }
        Err(_) => {
            process::exit(1);
        }
    }
}
