#[cfg(test)]
mod tests2 {
    use procinfo::pid::{status, status_self};
    use {get_next_pid, ausgabe};

    #[test]
    fn test1_ausgabe() {
        //let a = vec![1];
        assert_eq!(Ok("systemd".to_string()), ausgabe(vec![1]));
    }

    #[test]
    fn test2_get_next_pid() {
        assert_eq!(Ok("systemd".to_string()), get_pid_command(1));
    }
