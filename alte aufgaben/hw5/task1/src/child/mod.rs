extern crate nix;
use nix::unistd::{fork, getpid};
use nix::unistd::ForkResult::{Parent, Child};
use nix::sys::wait::wait;
use nix::sys::wait::WaitStatus::Exited;
mod pstree;

pub fn run_childs(start_pid: i32, arg: &str) -> Result<(), String> {
    match arg.parse::<u8>() {
        Ok(x) => {
            if x == 0 {
                return Ok(());
            }
            match fork() {
                Ok(Parent { child }) => {
                    match wait() {
                        Ok(Exited(c_pid, c_status)) => {
                            let s = "I am ".to_string() + getpid().to_string().as_str() +
                                    " and my child is " +
                                    child.to_string().as_str() +
                                    ". After I waited for " +
                                    c_pid.to_string().as_str() +
                                    ", it sent me status " +
                                    c_status.to_string().as_str();
                            println!("{}", s);
                            Ok(())
                        }
                        _ => Err("wait failed".to_string()),
                    }
                }
                Ok(Child) => {
                    println!("hello, I am child (pid: {})", getpid());
                    if x < 2 {
                        println!("");
                        match pstree::pstree(start_pid) {
                            Ok(z) => {
                                println!("{}", z);
                                Ok(())
                            }
                            Err(err) => Err(err.to_string()),
                        }
                    } else {
                        let a = x - 1;
                        match run_childs(start_pid, a.to_string().as_str()) {
                            Ok(y) => Ok(y),
                            Err(err) => Err(err),
                        }
                    }
                }
                Err(_) => Err("Fork failed".to_string()),
            }
        }
        Err(_) => Err("parsing failed".to_string()),
    }
}
