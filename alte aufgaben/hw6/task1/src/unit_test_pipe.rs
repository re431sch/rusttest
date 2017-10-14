#[cfg(test)]
mod tests {
    use {sum_strings, mul_strings, split_into_strings, concatenate_strings};

    #[test]
    fn test1() {
        let mut a: Vec<&str> = Vec::new();
        a.push("1");
        a.push("20");
        a.push("3");
        assert_eq!(sum_strings(a), 24);
    }

    #[test]
    fn test2() {
        let b = "Hello World";
        assert_eq!(split_into_strings(b)[1], "World");
    }

    #[test]
    fn test3() {
        let mut c: Vec<&str> = Vec::new();
        c.push("1");
        c.push("20");
        c.push("3");
        assert_eq!(mul_strings(c), 60);
    }

    #[test]
    fn test4() {
        let mut d: Vec<String> = Vec::new();
        d.push("hello".to_string());
        assert_eq!(concatenate_strings(&d), "");
    }

    #[test]
    #[should_panic]
    fn test5() {
        let mut e: Vec<&str> = Vec::new();
        e.push("1");
        e.push("20");
        e.push("3");
        assert_eq!(mul_strings(e), 8880);
    }
}
