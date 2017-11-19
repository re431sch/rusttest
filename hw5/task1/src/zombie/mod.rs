use nix::unistd::fork;
use nix::unistd::ForkResult::{Child, Parent};
use std::process::Command;
use std::{thread, time};


/// Erstellt einen zombie Prozess
pub fn run_zombie() {

    match fork() {
        Ok(Child) => {

            let ten_millis = time::Duration::from_millis(1);
            thread::sleep(ten_millis);

        }
        Ok(Parent {..}) => {

            let a = Command::new("ps").arg("t").status().expect("failed");
            println!("{}",a);

        }
        Err(_) => panic!("fork failed"),
    }
}
