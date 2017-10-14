#[cfg(test)]
mod tests {
    extern crate task1;
    use self::task1::{Text, Err};

    #[test]
    fn test_publish() {
        let a = "PUBLISH Hallo\n";
        assert_eq!(Ok(None), Text::default().parse(a));
    }

    #[test]
    fn test_publish_retrieve() {
        let mut b = Text::default();
        let c = "PUBLISH Hallo\n";
        let d = "RETRIEVE\n";
        assert_eq!(Ok(None), b.parse(c));
        assert_eq!(Ok(Some("Hallo".to_string())), b.parse(d));
    }

    #[test]
    fn test_publish_retrieve_with_whitespace() {
        let mut e = Text::default();
        let f = "   PUBLISH Hallo\n";
        let g = "   RETRIEVE\n";
        assert_eq!(Ok(None), e.parse(f));
        assert_eq!(Ok(Some("Hallo".to_string())), e.parse(g));
    }

    #[test]
    fn test_publish_without_newline() {
        let mut h = Text::default();
        let i = "PUBLISH Hallo";
        assert_eq!(Err(Err::FormatErr), h.parse(i));
    }

    #[test]
    fn test_retrieve_without_newline() {
        let mut j = Text::default();
        let k = "RETRIEVE";
        assert_eq!(Err(Err::FormatErr), j.parse(k));
    }

    #[test]
    fn test_publish_more_newlines() {
        let mut l = Text::default();
        let m = "PUBLISH Hallo\n Welt\n";
        assert_eq!(Ok(None), l.parse(m));
    }

    #[test]
    fn test_publish_and_retrieve_more_newlines() {
        let mut n = Text::default();
        let o = "PUBLISH Hallo\n Welt\n";
        let p = "RETRIEVE\n";
        assert_eq!(Ok(None), n.parse(o));
        assert_eq!(Ok(Some("Hallo".to_string())), n.parse(p));
    }
}
