use std::{process, thread, time};
use nix::unistd::fork;
use nix::unistd::ForkResult::Child;
use std::process::Command;


///Kommentar
pub fn run_zombie() {
    match fork() {
        Ok(Child) => {
            process::exit(1);
        }
        Ok(_) => {
            thread::sleep(time::Duration::from_secs(1));
            Command::new("ps")
                .arg("t")
                .status()
                .expect("failed to execute process");
        }
        Err(_) => {
            println!("Fork failed");
        }
    }
}
