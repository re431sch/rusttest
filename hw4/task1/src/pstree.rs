use procinfo::pid::*;
use procinfo::loadavg;
use readproc::*;
use std::process;

pub fn get_next_pid(bispid: i32) -> Result<String, &'static str> {
    let mut vec = Vec::new();
    let ne = stat_self().unwrap();
    let mut newpid = ne.pid;
    match stat(bispid) {
        Ok(_) => {
            match stat_self() {
                Ok(a) => {
                    vec.push(a.pid);
                    //let mut newpid = a.ppid;
                    while newpid != bispid {
                        match stat(newpid) {
                            Ok(b) => {
                                vec.push(b.pid);
                                newpid = b.ppid;
                            }
                            Err(_) => println!("error"),
                        }
                        /*if newpid == bispid {
                            break;
                        }*/
                    }
                    vec.push(newpid);
                    ausgabe(vec)
                }
                Err(e) => {
                    println!("{}", e);
                    process::exit(1);
                }
            }
        }
        Err(_) => Err("error"),
    }
}

pub fn ausgabe(mut vec: Vec<i32>) -> Result<String, &'static str> {
    let a = vec.sort();
    let mut len = vec.len() - 1;
    let mut ausgabe = String::new();
    for i in 0 .. len {
        match stat(vec[i]) {
            Ok(a) => {
                if len == 1 {
                    ausgabe += format!("{}({}) ", a.command, a.pid).as_str();
                } else {
                    ausgabe += format!("{}({})---", a.command, a.pid).as_str();
                }
                len = len - 1;
            }
            Err(_) => println!("error"),
        }
    }
    Ok(ausgabe)
}
