use procinfo::pid::*;
use std::process;


//-------------------------------------------------------------------------------------get_next_pid


/// Erstellung des pstree mit Hilfe eines Vektors
pub fn get_next_pid(bispid: i32) -> Result<String, &'static str> {

    let mut vec = Vec::new();
    let ne = stat_self().unwrap();
    let mut newpid = ne.pid;

    /// match für den Übergabeparameter
    match stat(bispid) {
        Ok(_) => {
            /// match für die eigene PID
            match stat_self() {
                Ok(a) => {
                    /// Schleife, die von der eigenen PID bis zum Übergabeparameter läuft
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
pub fn ausgabe(mut vec: Vec<i32>) -> Result<String, &'static str> {

    /// Einträge des Vektors umdrehen
    let a = vec.sort();
    let mut len = vec.len();
    let mut ausgabe = String::new();

    /// Vektor durchlaufen und dessen Einträge printen
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
