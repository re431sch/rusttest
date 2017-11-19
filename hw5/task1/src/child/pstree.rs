use procinfo::pid::{stat, stat_self};
use std::process;


//-------------------------------------------------------------------------------------get_next_pid


/// Erstellung des pstree mit Hilfe eines Vektors
pub fn get_next_pid(bispid: i32) -> Result<String, &'static str> {

    let mut vec = Vec::new();
    let ne = stat_self().unwrap();
    let mut newpid = ne.pid;

    match stat(bispid) {
        Ok(_) => {
            //match für die eigene PID
            match stat_self() {
                Ok(_) => {
                    //Schleife, die von der eigenen PID bis zur bispid läuft
                    while newpid != bispid {
                        match stat(newpid) {
                            Ok(b) => {
                                vec.push(b.pid);
                                newpid = b.ppid;
                            }
                            Err(_) => println!("Eltern PID nicht erkannt"),
                        }
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
        Err(_) => Err("Aufruf ist keine PID"),
    }
}


//------------------------------------------------------------------------------------------Ausgabe


/// Ausgabe des pstree
pub fn ausgabe(vec: Vec<i32>) -> Result<String, &'static str> {

    let mut ausgabe = String::new();

    for i in 0..vec.len() {
        let j = (vec.len() - 1) - i;
        match stat(vec[j]) {
            Ok(a) => {
                if j == 0 {
                    ausgabe += format!("{}({})", a.command, a.pid).as_str();
                } else {
                    ausgabe += format!("{}({})---", a.command, a.pid).as_str();
                }
            }
            Err(_) => println!("error"),
        }
    }
    Ok(ausgabe)
}
