mod pstree;
use nix::unistd::{getpid,fork};
use nix::unistd::ForkResult::{Parent, Child};
use nix::sys::wait::wait;
use nix::sys::wait::WaitStatus::Exited;


/// Erstellt kind Prozesse und gibt sie mit der pstree Funktion aus
pub fn run_childs(start_pid: i32, arg: &str) -> Result<(), String> {

    match arg.parse::<u8>() {
        Ok(mut a) => {
            if a < 1 {
                return Ok(());
            }
            match fork() {
                Ok(Child) => {
                    println!("hello, I am child (pid:{})", getpid());
                    if a == 1 {
                        match pstree::get_next_pid(start_pid) {
                            Ok(tree) => {
                                println!("{}", tree);
                                Ok(())
                            }
                            Err(e) => Err(e.to_string()),
                        }
                    } else {
                        a = a - 1;
                        match run_childs(start_pid, a.to_string().as_str()) {
                            Ok(()) => Ok(()),
                            Err(e) => Err(e),
                        }
                    }
                }
                Ok(Parent {child}) => {
                    match wait() {
                        Ok(Exited(pid, stat)) => {
                            println!("I am {} and my child is {}.   After I waited for {}, it sent me status {}", getpid(), child, pid, stat);
                            Ok(())
                        }
                        _ => Err("failed".to_string()),
                    }
                }
                Err(_) => Err(String::from("fork failed")),
            }
        }
        Err(_) => Err(String::from("fork failed")),
    }
}
