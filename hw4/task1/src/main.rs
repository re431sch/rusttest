extern crate procinfo;
mod readproc;
use readproc::*;


fn main() {

    match self_pids() {
        Ok(a) => {
            //println!("PID: {}, PPiD: {}", a.0, a.1);
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
        Ok(c) => println!("Alle tasks: {}", c),
        Err(e) => println!("{}", e),
    }

}
