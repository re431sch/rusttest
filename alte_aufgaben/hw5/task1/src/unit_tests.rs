#[cfg(test)]
mod tests {
    use child::run_childs;
    use nix::unistd::getpid;

    #[test]
    fn test0_child() {
        let a = getpid();
        assert_eq!(run_childs(a, "3".into()), Ok(()));
    }
}
