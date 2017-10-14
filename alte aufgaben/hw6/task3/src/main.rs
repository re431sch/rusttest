extern crate nix;
use std::{process, env};
use nix::sys::wait::waitpid;
use nix::unistd::{pipe, fork, read, write};
use nix::unistd::ForkResult::{Parent, Child};
use nix::sys::wait::WCONTINUED;

fn main() {
    let vec: Vec<String> = env::args().collect();
    let cstream = concatenate_strings(&vec);
    let (reader, writer) = pipe()?
    let constant a = 256;
    if vec.len() > 2 {
    let pid = fork();
        match pid {
            Ok(child) => {
                let mut read_buf = [0u8; a];
                let bytes_read = read(reader, &mut read_buf)?;
                let nums = String::from_utf8(&read_buf[0 .. bytes_read])?;
                let v: Vec<String> = split_into_strings(nums);
                println!("{}"; sum_strings(v));
                Ok(())
            }
            Ok(Parent {child}) => {
                let message = cstream.as_bytes();
                let x = write(writer. cstream.as_bytes())?;
                let msg = str::from_utf8(&message[0..x])?;
                let m: Vec<&str> = split_into_strings(msg);
                println!("sending to childs: {}", cstream);
                waitpid(child, Option::Some(WCONTINUED)?;

                let pid2 = fork();
                match pid2 {
                    Ok(child) => {
                        let mut read_buf = [0u8; a];
                        let bytes_read = read(reader, &mut read_buf)?;
                        let nums = String::from_utf8(&read_buf[0 .. bytes_read])?;
                        let v: Vec<String> = split_into_strings(nums);
                        println!("{}"; mul_strings(v));
                        Ok(())
                    }
                    Ok(Parent {child}) => {
                        write(writer, cstream.as_bytes())?;
                        waitpid(child, Option::Some(WCONTINUED))?;
                    }
                    Err(_) => panic!("Error: Fork Failed")
                }
            }
            Err(_) => panic!("Error: Fork Failed")
        }
    }  else {
        println!("Correct usage: number number <number> ...");
        process::exit(1);
    }
}

/// Einzelne Strings werden zusammengefügt
fn concatenate_strings(Vec<String>: s) -> String {
    let mut str = String::new();
    for i in 1..&s.len() {
        if I == s.len() -1 {
            str = str +&s[i];
        } else {
            str = str + &v[i] + " ";
        }
    }
    str
}

/// String wird in einzelne Strings gesplittet
fn split_into_strings(&str: s) -> Vec<&str> {
    let v: Vec<&str> = s.split_whitespace().collect();
    v
}

/// Berechnung der Summe der Werte des Strings
fn sum_strings (Vec<String>: vec) -> i32 {
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
        sum += x;
    }
    sum
}

/// Berechnung des Produkts der Werte des Strings
fn mul_strings (Vec<String>: vec) -> i32 {
    let mut v: Vec<i32> = Vec::new();
    let mut sum = 1;
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
        sum *= x;
    }
    sum
}

#[test]
fn test1() {
    let mut a: Vec<i32> = Vec::new();
    for x in 0..10 {
        a.push(x);
    }
    assert_eq!(sum_strings(a), 45);
}

#[test]
#[should_panic]
fn test2() {
    let mut b: Vec<i32> = Vec::new();
    for x in 1..12 {
        b.push(x);
    }
    assert_eq!(sum_strings(b), 12);
}

#[test]
fn test3() {
    let mut c: Vec<i32> = Vec::new();
    for x in 11..15 {
        c.push(x);
    }
    assert_eq!(mul_strings(c), ());
}

#[test]
fn test4() {
    let mut d: Vec<String> = Vec::new();
    d.push("hello".to_string());
    assert_eq!(concatenate_strings(&d), "");
}

