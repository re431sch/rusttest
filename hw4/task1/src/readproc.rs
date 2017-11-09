use procinfo::pid::*;
use procinfo::loadavg;

/// Funktion gibt die eigene PID und die PPID in einem Tupel zurück
pub fn self_pids() -> Result<(i32, i32), &'static str> {
    match stat_self().ok() {
        Some(b) => {
            let a = (b.pid, b.ppid);
            Ok(a)
        }
        _ => Err("falsch"),
    }
}
//------------------------------------------------------------------------------------------------1
/// Funktion gibt den Command Namen zu einer PID zurück
pub fn get_pid_command(pid: i32) -> Result<String, &'static str> {
    match stat(pid).ok() {
        Some(a) => {
            let b = a.command;
            Ok(b)
        }
        _ => Err("PID not alive: no command name found"),
    }
}
//------------------------------------------------------------------------------------------------2
/// Funktion gibt den Command Namen des zuletzt erzeugten Prozesses im System zurück
pub fn get_last_created_command() -> Result<String, &'static str> {
    match loadavg().ok() {
        Some(a) => {
            let b = a.last_created_pid;
            match get_pid_command(b) {
                Ok(c) => Ok(c),
                Err(e) => Err(e),
            }
        }
        _ => Err("No last command via PID found"),
    }
}
//------------------------------------------------------------------------------------------------3
/// Funktion liefert Anzahl der Threads pro PID zurück
pub fn get_thread_count(pid: i32) -> Result<u32, &'static str> {
    match stat(pid).ok() {
        Some(a) => {
            let b = a.num_threads as u32;
            Ok(b)
        }
        _ => Err("PID not alive: no threads counted"),
    }
}
//------------------------------------------------------------------------------------------------4
/// Funktion liefert die Gesamtmenge aller Tasks im System
pub fn get_task_total() -> Result<u32, &'static str> {
    match loadavg().ok() {
        Some(a) => {
            let b = a.tasks_total;
            Ok(b)
        }
        _ => Err("No total count of tasks in system found"),
    }
}
//------------------------------------------------------------------------------------------------5
/// Funktion liefert Werte für vsize, code und data
pub fn get_ownprocess_mem() -> Result<(usize,usize,usize), &'static str> {
    match statm_self().ok() {
        Some(b) => {
            let a = (b.size, b.text, b.data);
            Ok(a)
        }
        _ => Err("falsch"),
    }
}
