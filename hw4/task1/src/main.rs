extern crate procinfo;
mod readproc;
mod pstree;
use readproc::*;
use pstree::*;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    match args.len() {

        //-----------------------------------------------------------------------------------Fall 1
        //Programmaufruf ohne Parameter
        1 => {
            match self_pids() {
                Ok(a) => {

                    //Ausgabe der PID, Name und Thread
                    match get_pid_command(a.0) {
                        Ok(b) => {
                            match get_thread_count(a.0) {
                                Ok(c) => {
                                    println!("My PID: {} - {} running {} threads", a.0, b, c);
                                }
                                Err(e) => println!("{}", e),
                            }
                        }
                        Err(e) => println!("{}", e),
                    }

                    //Ausgabe der PPID, Name und Thread
                    match get_pid_command(a.1) {
                        Ok(b) => {
                            match get_thread_count(a.1) {
                                Ok(c) => {
                                    println!("My PPID: {} - {} running {} threads", a.1, b, c);
                                }
                                Err(e) => println!("{}", e),
                            }
                        }
                        Err(e) => println!("{}", e),
                    }
                }
                Err(e) => println!("{}", e),
            }

            //Ausgebe Speicher Status
            match get_ownprocess_mem() {
                Ok(a) => {
                    println!("My mem: {} (vspace), {} (code), {} (data)", a.0, a.1, a.2);
                }
                Err(e) => println!("{}", e),
            }

            //Ausgabe letzter Prozess
            match get_last_created_command() {
                Ok(a) => println!("Last process created in system was: {}", a),
                Err(e) => println!("{}", e),
            }

            //Ausgabe alle taks
            match get_task_total() {
                Ok(c) => println!("Alle tasks: {}", c),
                Err(e) => println!("{}", e),
            }
        }
        //-----------------------------------------------------------------------------------Fall 2
        //Programmaufruf mit PID
        2 => {
            //Parameter vom Typ i32
            match args[1].parse::<i32>() {
                Ok(a) => {
                    //Erstellung vom pstree
                    match get_next_pid(a) {
                        Ok(b) => println!("{}", b),
                        Err(e) => {
                            println!("{}",e);
                            std::process::exit(1);
                        }
                    };
                }
                Err(e) => {
                    println!("{}",e);
                    std::process::exit(1);
                }
            }
        }
        //Programmaufruf mit zu vielen Parametern
        _ => {
            println!("Correct usage: no param or param PID");
            std::process::exit(1);
        }
    }
}
