extern crate tokio_io;
extern crate tokio_proto;
extern crate tokio_service;
extern crate futures;
extern crate bytes;

use std::io;
use std::ops::Range;

use tokio_io::*;
use tokio_service::*;
use tokio_proto::*;

struct Echo {
    buf: Vec<u8>,
    pending: Range<usize>,
    stream: TcpStream,
}

impl Echo {
    fn from_stream(stream: TcpStream) -> io::Result<Echo> {
        Ok(Echo{
            buf: vec![0; 256],
            pending: 0..0,
            stream: stream,
        })
    }
}

fn main() {
    let reactor = Reactor::default().unwrap();
    let address = "127.0.0.1:8080".parse().unwrap();
    listen(&reactor.handle(), address, Echo::from_stream).unwrap();
    reactor.run().unwrap();
}

impl Task for Echo {
    fn tick(&mut self) -> io::Result<Tick> {
        loop {
            match self.pending {
                Range { start, end } if start == end => {
                    if let Some(len) = try!(self.stream
                        .try_read(&mut self.buf)) {
                        if len == 0 {
                            return Ok(Tick::Final);
                        }
                        self.pending = 0..len;
                    } else {
                        return Ok(Tick::WouldBlock);
                    }
                }
                ref mut range => {
                    if let Some(len) = try!(self.stream
                        .try_write(&self.buf[range.clone()])) {
                        range.start += len;
                    } else {
                        return Ok(Tick::WouldBlock);
                    }
                }
            }
        }
    }
}
