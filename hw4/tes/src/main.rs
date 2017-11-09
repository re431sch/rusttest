extern crate procinfo;
use procinfo::pid::*;
use procinfo::loadavg;

fn main() {
    match self_pids() {
        Ok(a) => {
            //println!("My PID: {}", a.0);
            //println!("My PPiD: {}", a.1);
            match get_pid_command(a.0) {
                Ok(_) => {},
                Err(e) => println!("{}", e),
            }
            match get_thread_count(a.0) {
                Ok(c) => {
                    println!("My PID: {} - process1 running {} threads", a.0,  c);
                    println!("My PPID: {} - process2 running {} threads", a.1,  c);
                }
                Err(e) => println!("{}", e),
            }
        }
        Err(e) => println!("{}", e),
    }
    match get_ownprocess_mem() {
        Ok(a) => {
            println!("My mem: {} (vspace), {} (code), {} (data)", a.0, a.1, a.2);
        }
        Err(e) => println!("{}", e),
    }
    match get_last_created_command() {
        Ok(a) => println!("Last process created in system was: {}", a),
        Err(e) => println!("{}", e),
    }
    match get_task_total() {
        Ok(c) => println!("Total number of tasks: {}", c),
        Err(e) => println!("{}", e),
    }
}
//------------------------------------------------------------------------------------------------1
fn self_pids() -> Result<(i32, i32), &'static str> {
    match stat_self().ok() {
        Some(b) => {
            let a = (b.pid, b.ppid);
            Ok(a)
        }
        _ => Err("falsch"),
    }
}
//------------------------------------------------------------------------------------------------2
fn get_pid_command(pid: i32) -> Result<String, &'static str> {
    match stat(pid).ok() {
        Some(a) => {
            let b = a.command;
            Ok(b)
        }
        _ => Err("PID not alive: no command name found"),
    }
}
//------------------------------------------------------------------------------------------------3
fn get_last_created_command() -> Result<String, &'static str> {
    match loadavg().ok() {
        Some(a) => {
            let b = a.last_created_pid;
            match get_pid_command(b) {
                Ok(c) => Ok(c),
                Err(e) => Err(e),
            }
        }
        _ => Err("No last command via PID found"),
    }
}
//------------------------------------------------------------------------------------------------4
fn get_thread_count(pid: i32) -> Result<u32, &'static str> {
    match stat(pid).ok() {
        Some(a) => {
            let b = a.num_threads as u32;
            Ok(b)
        }
        _ => Err("PID not alive: no threads counted"),
    }
}
//------------------------------------------------------------------------------------------------5
fn get_task_total() -> Result<u32, &'static str> {
    match loadavg().ok() {
        Some(a) => {
            let b = a.tasks_total;
            Ok(b)
        }
        _ => Err("No total count of tasks in system found"),
    }
}
//------------------------------------------------------------------------------------------------6
fn get_ownprocess_mem() -> Result<(usize,usize,usize), &'static str> {
    match statm_self().ok() {
        Some(b) => {
            let a = (b.size, b.text, b.data);
            Ok(a)
        }
        _ => Err("falsch"),
    }
}
