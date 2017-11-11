use procinfo::pid::*;
use procinfo::loadavg;
use readproc::*;

pub fn get_next_pid(pid: pid_t)  {

    let mut accpid = pid;
    match get_pid_command(accpid) {
        Ok(p) => {
            printl!("{}", p);
            match self_pids() {
                Ok(a) => {
                    printl!("({})---", a.0);
                    accpid = a.1;
                    get_next_pid(accpid);
                }
                Err(e) => println!("{}", e),
        }
        Err(p) => println!("{}", p),
    }

}
