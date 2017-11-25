#[cfg(test)]
mod tests {
    use{concatenate_strings, split_into_strings, sum_strings, mul_strings};

    #[test]
    fn test() {
        let mut vec: Vec<String> = Vec::new();
        vec.push("eins".to_string());
        vec.push("zwei".to_string());
        assert_eq!(concatenate_strings(&vec), "eins zwei");
    }

    #[test]
    fn test2() {
        let s = "eins zwei":
        assert_eq!(split_into_strings(s)[0], "eins");
        assert_eq!(split_into_strings(s)[1], "zwei");
    }

    #[test]
    fn test3() {
        let mut vec: Vec<&str> = Vec::new();
        vec.push("3");
        vec.push("4");
        assert_eq!(sum_strings(vec), 7);
    }

    #[test]
    #[should_panic]
    fn test4() {
        let mut vec: Vec<&str> = Vec::new();
        vec.push("3");
        vec.push("4");
        assert_eq!(sum_strings(vec), 56);
    }

    #[test]
    fn test5() {
        let mut vec: Vec<&str> = Vec::new();
        vec.push("3");
        vec.push("4");
        assert_eq!(mul_strings(vec), 12);
    }

    #[test]
    #[should_panic]
    fn test6() {
        let mut vec: Vec<&str> = Vec::new();
        vec.push("3");
        vec.push("4");
        assert_eq!(mul_strings(vec), 55);
    }

}
