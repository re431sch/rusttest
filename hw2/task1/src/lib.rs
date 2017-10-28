pub fn hamming_distance(s1: &str, s2: &str) -> Result<usize,String> {
    if s1.len() != s2.len() {
        return Err(String::from("falsch"));
    }

        let mut count = 0;
        for i in 0 ..  s1.len() {
            if s1.chars().nth(i) != s2.chars().nth(i) {
                count += 1;
            }

        }
        Ok(count)
}
