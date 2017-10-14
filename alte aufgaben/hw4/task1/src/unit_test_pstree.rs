#[cfg(test)]
mod tests2 {
    use pstree::{pstree, output};

    #[test]
    fn test5_pstree() {
        assert_eq!(Err("Correct usage: No PID"), pstree(2));
    }
    #[test]
    fn test6_output() {
        let x = vec![1];
        assert_eq!(Ok("systemd(1)".to_string()), output(x));
    }
}
