#[derive(Default)]
pub struct Text {
    pub satz: String,
}

#[derive(Debug, PartialEq)]
pub enum Err {
    FormatErr,
}

impl Text {
    pub fn parse(&mut self, message: &str) -> Result<Option<String>, Err> {
        match message.find('\n') {
            Some(x) => {
                let s = message[0..x].trim_left().to_string();
                let vs: Vec<&str> = s.splitn(2, ' ').collect();
                match vs[0] {
                    "PUBLISH" => {
                        self.satz = vs[1..].join(" ");
                        Ok(None)
                    }
                    "RETRIEVE" => Ok(Some(self.satz.clone())),
                    _ => Err(Err::FormatErr),
                }
            }
            None => Err(Err::FormatErr),
        }
    }
}
