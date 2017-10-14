use procinfo::pid::*;
use procinfo::loadavg;

/// returns the PID and PPID
pub fn self_pids() -> Result<(i32, i32), &'static str> {
    match stat_self().ok() {
        Some(x) => {
            let tuple = (x.pid, x.ppid);
            Ok(tuple)
        }
        _ => Err("geht nicht."),
    }
}

/// returns command of the PID
pub fn get_pid_command(pid: i32) -> Result<String, &'static str> {
    match stat(pid).ok() {
        Some(x) => Ok(x.command.to_string()),
        _ => Err("PID not alive: no command name found"),
    }
}

/// returns the last created command
pub fn get_last_created_command() -> Result<String, &'static str> {
    match loadavg().ok() {
        Some(x) => {
            let y = x.last_created_pid as i32;
            match get_pid_command(y) {
                Ok(z) => Ok(z),
                Err(err) => Err(err),
            }
        }
        _ => Err("No last command via PID found"),
    }
}

/// returns the sum of all running threads
pub fn get_thread_count(pid: i32) -> Result<u32, &'static str> {
    match stat(pid).ok() {
        Some(x) => {
            let y = x.num_threads as u32;
            Ok(y)
        }
        _ => Err("PID not alive: no threads counted"),
    }
}

/// returns the total number of tasks
pub fn get_task_total() -> Result<u32, &'static str> {
    match loadavg().ok() {
        Some(x) => Ok(x.tasks_total),
        _ => Err("no total count of tasks in system found"),
    }
}

/// returns the the total mem specs
pub fn get_ownprocess_mem() -> Result<(usize, usize, usize), &'static str> {
    match statm_self().ok() {
        Some(x) => {
            let tuple = (x.size, x.text, x.data);
            Ok(tuple)
        }
        _ => Err("geht nicht."),
    }
}
