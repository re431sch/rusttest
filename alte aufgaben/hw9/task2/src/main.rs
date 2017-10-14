use std::io::{Write, Read};
use std::net::*;
use std::collections::VecDeque;
use std::thread;
use std::sync::{Arc, Mutex};

extern crate task1;
use task1::*;

#[derive(Default)]
pub struct Queue {
    pub q: Mutex<VecDeque<String>>,
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
    let a = Arc::new(Queue::new());
    match TcpListener::bind("127.0.0.1:8080") {
        Ok(x) => {
            for stream in x.incoming() {
                match stream {
                    Err(e) => println!("unable to connect: {}", e),
                    Ok(mut stream) => {
                        let a_cln = a.clone();
                        thread::spawn(move || match read_write(&a_cln, &mut stream) {
                                          Ok(_) => {}
                                          Err(err) => println!("{:?}", err),
                                      });
                    }
                }
            }
        }
        Err(_) => println!("BindError"),
    }
}



impl Queue {
    pub fn new() -> Queue {
        let kuh = Mutex::new(VecDeque::new());
        Queue { q: kuh }
    }

    pub fn lock_write(&self, msg: String) {
        let mut x = self.q.lock().unwrap();
        x.push_back(msg);
    }

    pub fn lock_read(&self) -> Option<String> {
        let mut x = self.q.lock().unwrap();
        x.pop_back()
    }
}

fn read_write(storage: &Queue, stream: &mut TcpStream) -> Result<(), Erro> {
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
            match storage.lock_read() {
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
            storage.lock_write(t.satz.clone());
            Ok(())
        }
        Err(_) => Err(Erro::FormatError),
    }
}
