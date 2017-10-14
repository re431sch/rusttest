extern crate procinfo;
mod readproc;
use readproc::*;
mod pstree;
use pstree::*;
use std::{process, env};
/// reads data from the proc-folder by using the functions in pstree.rs and readproc.rs
fn main() {
    let vec: Vec<String> = env::args().collect();
    match vec.len() {
        1 => readproc(),
        2 => {
            match vec[1].parse::<i32>() {
                Ok(x) => {
                    match pstree(x) {
                        Ok(y) => println!("{}", y),
                        Err(err) => {
                            println!("{}", err);
                            process::exit(1);
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }
        _ => {
            println!("Correct usage: Wrong Format!");
            process::exit(1);
        }
    }
}

fn readproc() {
    match self_pids() {
        Ok(x) => {
            match get_pid_command(x.0) {
                Ok(y) => {
                    match get_thread_count(x.0) {
                        Ok(z) => {
                            println!("My PID : {} - {} running {} threads", x.0, y, z);
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
            match get_pid_command(x.1) {
                Ok(y) => {
                    match get_thread_count(x.1) {
                        Ok(z) => {
                            println!("My PPID : {} - {} running {} threads", x.1, y, z);
                        }
                        Err(err) => {
                            println!("{}", err);
                        }
                    }
                }
                Err(err) => {
                    println!("{}", err);
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    match get_ownprocess_mem() {
        Ok(x) => {
            println!("My mem : {} (vspace), {} (code), {} (data)", x.0, x.1, x.2);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    match get_last_created_command() {
        Ok(x) => {
            println!("Last process created in system was: {}", x);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
    match get_task_total() {
        Ok(x) => {
            println!("Total number of tasks: {}", x);
        }
        Err(err) => {
            println!("{}", err);
        }
    }
}
