extern crate nix;
use nix::sys::wait::waitpid;
use nix::sys::wait::WCONTINUED;
use nix::unistd::{pipe, fork, read, write};
use nix::unistd::ForkResult::{Parent, Child};
use std::{process, env, str};
mod unit_test_pipe;

/// enum with the two needed Errors
#[derive(Debug)]
enum Error {
    Nix(nix::Error),
    Utf(std::str::Utf8Error),
}

/// main function.
fn main() {
    match pcc() {
        Ok(x) => x,
        Err(_) => {
            println!("Error");
            process::exit(1);
        }
    }
}

/// Function to spawn 2 Childs and a Parent, which then outputs the input arguments
///and the sum and product of the arguments with the help u separate functions.
fn pcc() -> Result<(), Error> {
    let (reader, writer) = pipe().map_err(Error::Nix)?;
    let vec: Vec<String> = env::args().collect();
    const A: usize = 256;
    if vec.len() > 2 {
        let cstream = concatenate_strings(&vec);
        let pid = fork();
        match pid {

            Ok(Parent { child }) => {
                try!(write(writer, cstream.as_bytes()).map_err(Error::Nix));
                println!("sending to childs: {}", cstream);
                waitpid(child, Option::Some(WCONTINUED))
                    .map_err(Error::Nix)?;
                let pid2 = fork();
                match pid2 {
                    Ok(Parent { child }) => {
                        write(writer, cstream.as_bytes()).map_err(Error::Nix)?;
                        waitpid(child, Option::Some(WCONTINUED))
                            .map_err(Error::Nix)?;
                        Ok(())
                    }
                    Ok(Child) => {
                        let mut read2_buf = [0u8; A];
                        let bytes2_read = read(reader, &mut read2_buf).map_err(Error::Nix)?;
                        let me = str::from_utf8(&read2_buf[0..bytes2_read])
                            .map_err(Error::Utf)?;
                        let v: Vec<&str> = split_into_strings(me);
                        println!("Mul = {}", mul_strings(v));
                        Ok(())
                    }
                    Err(_) => {
                        println!("Error: Fork Failed");
                        process::exit(1);
                    }

                }
            }
            Ok(Child) => {
                let mut read2_buf = [0u8; A];
                let bytes2_read = read(reader, &mut read2_buf).map_err(Error::Nix)?;
                let me = str::from_utf8(&read2_buf[0..bytes2_read])
                    .map_err(Error::Utf)?;
                let v: Vec<&str> = split_into_strings(me);
                println!("Sum = {}", sum_strings(v));
                Ok(())
            }
            Err(_) => {
                println!("Error: Fork Failed");
                process::exit(1);
            }
        }
    } else {
        println!("Correct usage: number number <number> ...");
        process::exit(1);
    }
}

/// Splitting the String into Slices without whitespace
fn split_into_strings(s: &str) -> Vec<&str> {
    let str: Vec<&str> = s.split_whitespace().collect();
    str
}
/// Compute the sum of the String
fn sum_strings(vec: Vec<&str>) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut sum = 0;
    for i in vec {
        match i.parse::<i32>() {
            Ok(x) => {
                v.push(x);
            }
            Err(_) => {
                println!("Correct usage: number number <number> ...");
                process::exit(1);
            }
        }
    }
    for i in v {
        sum += i;
    }
    sum
}
/// Compute the product of the String
fn mul_strings(vec: Vec<&str>) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut mul = 1;
    for i in vec {
        match i.parse::<i32>() {
            Ok(x) => {
                v.push(x);
            }
            Err(_) => {
                println!("Correct usage: number number <number> ...");
                process::exit(1);
            }
        }
    }
    for i in v {
        mul *= i;
    }
    mul
}
//function creates a string containing the passed arguments
fn concatenate_strings(v: &Vec<String>) -> String {
    let mut s = String::new();
    for i in 1..&v.len() + 0 {
        if i == v.len() - 1 {
            s = s + &v[i];
        } else {
            s = s + &v[i] + " ";
        }
    }
    s
}
