#[cfg(test)]
mod test {
    use shell::Shell;

    #[test]
    fn test_prompt_in_memory_with_string() {
        let input = b"echo";
        let mut output = Vec::new();

        let mut shell = Shell {
            reader: &input[..],
            writer: &mut output,
            should_exit: false,
            name: "bsys-shell".to_string(),
        };

        let output = shell.prompt().unwrap().unwrap();

        assert_eq!("echo", output);
    }

    #[test]
    fn test_loop_of_shell_which_wants_to_exit() {
        let input = b"BlaBla";
        let mut output = Vec::new();

        let mut shell = Shell {
            reader: &input[..],
            writer: &mut output,
            should_exit: true,
            name: "bsys-shell".to_string(),
        };

        let output = shell.start().unwrap();

        assert_eq!((), output);
    }

    #[test]
    fn test_loop_with_exit_command() {
        let input = b"exit";
        let mut output = Vec::new();

        let mut shell = Shell {
            reader: &input[..],
            writer: &mut output,
            should_exit: false,
            name: "bsys-shell".to_string(),
        };

        let output = shell.start().unwrap();

        assert_eq!((), output);
    }
}
