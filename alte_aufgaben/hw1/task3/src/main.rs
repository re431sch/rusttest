fn main() {
    let s = "♥ The quick brown fox jumps over the lazy dog. ♥";
    println!("In '♥ The quick brown fox jumps over the lazy dog. ♥', {} 'e' were found",
             count(s, 'e'));
}

fn count(line: &str, c: char) -> u64 {
    let mut i = 0;
    for x in line.chars() {
        if x == c {
            i += 1;
        }
    }
    i
}

#[test]
fn test_one_char() {
    assert_eq!(count("♥ The quick brown fox jumps over the lazydog. ♥", 'T'),
               1);
}

#[test]
fn test_two_char() {
    assert_eq!(count("♥ The quick brown fox jumps over the lazy dog. ♥",
                     '♥'),
               2);
}

#[test]
#[should_panic]
fn test_wrong() {
    assert_eq!(count("♥ The quick brown fox jumps over the lazy dog. ♥", 'c'),
               2);
}

#[test]
fn test_four_char() {
    assert_eq!(count("♥ The quick brown fox jumps over the lazy dog. ♥", 'o'),
               4);
}

#[test]
fn test_no_char() {
    assert_eq!(count("♥ The quick brown fox jumps over the lazy dog. ♥", '!'),
               0);
}
