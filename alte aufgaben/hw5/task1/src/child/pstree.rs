use std::process;
use procinfo::pid::*;


///rekursiv pid >> ppid bis zur eingegeben PID rückwärts.
pub fn pstree(pid: i32) -> Result<String, &'static str> {
    match stat(pid) {
        Ok(_) => {
            let mut v = Vec::new();
            match stat_self() {
                Ok(y) => {
                    let mut a = y.ppid;
                    v.push(y.pid);
                    loop {
                        match stat(a) {
                            Ok(x) => {
                                v.push(x.pid);
                                a = x.ppid;
                            }
                            Err(_) => {
                                println!("mandatory Errormessage");
                            }
                        }
                        if a == pid {
                            break;
                        }
                    }
                    v.push(a);
                    output(v)
                }
                Err(err) => {
                    println!("{}", err);
                    process::exit(1);
                }
            }
        }
        Err(_) => Err("Correct usage: No PID"),
    }
}
/// output
pub fn output(v: Vec<i32>) -> Result<String, &'static str> {
    let mut s: String = String::new();
    for i in 0..v.len() {
        let a = (v.len() - 1) - i;
        match stat(v[a]) {
            Ok(x) => {
                if a == 0 {
                    s += format!("{}({})", x.command, x.pid).as_str();
                } else {
                    s += format!("{}({})---", x.command, x.pid).as_str();
                }
            }
            Err(_) => println!("Error"),
        }
    }
    Ok(s)
}
