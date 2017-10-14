use std::io::{Write, Read};
use std::net::*;
use std::collections::VecDeque;

extern crate task1;
use task1::*;

#[derive(Default)]
pub struct Queue {
    pub q: VecDeque<String>,
}

#[derive(Debug,PartialEq)]
pub enum Erro {
    FormatError,
    NoMsgErr,
}

impl From<Err> for Erro {
    fn from(x: Err) -> Self {
        match x {
            Err::FormatErr => Erro::FormatError,
        }
    }
}

fn main() {
    let mut a = Queue { q: VecDeque::new() };
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(x) => {
            for stream in x.incoming() {
                match stream {
                    Err(e) => println!("unable to connect: {}", e),
                    Ok(stream) => {
                        match a.read_write(stream) {
                            Ok(_) => {}
                            Err(err) => {
                                println!("{:?}", err);
                            }
                        }
                    }
                }
            }
        }
        Err(_) => println!("BindError"),
    }
}

impl Queue {
    fn read_write(&mut self, mut stream: TcpStream) -> Result<(), Erro> {
        let mut read_data = String::new();
        let mut t = Text { satz: String::new() };
        match stream.read_to_string(&mut read_data) {
            Ok(x) => {
                if x == 0 {
                    println!("No Command");
                }
            }
            Err(err) => {
                panic!("Error detected: {}", err);
            }
        }
        match t.parse(read_data.as_str()) {
            Ok(Some(_)) => {
                match self.q.pop_back() {
                    None => Err(Erro::NoMsgErr),
                    Some(m) => {
                        let n = m + "\n";
                        match stream.write(n.as_bytes()) {
                            Ok(_) => {}
                            Err(err) => println!("{}", err),
                        }
                        Ok(())
                    }
                }
            }
            Ok(None) => {
                self.q.push_back(t.satz.clone());
                Ok(())
            }
            Err(_) => Err(Erro::FormatError),
        }
    }
}
