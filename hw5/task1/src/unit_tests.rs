#[cfg(test)]
mod tests {
    use child::run_childs;
    use procinfo::pid::*;

    #[test]
    fn test() {
        match stat_self() {
            Ok(a) =>  {
                let pid = a.pid;
                assert_eq!(run_childs(pid, "5".into()), Ok(()));
            }
            Err(_) => println!("error"),
        }
    }
}
