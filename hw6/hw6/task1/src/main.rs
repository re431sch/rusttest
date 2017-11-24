extern crate nix;

use nix::unistd::getpid;
use nix::unistd::fork;
use nix::unistd::ForkResult::{Child, Parent};
use std::process::Command;
use std::{thread, time};
use nix::unistd::pipe;


fn main() {

    let args: Vec<String> = std::env::args().collect();
    //let (write, read) = pipe();
    match mach(&args) {
        Ok(x) => x,
        Err(_) => println!("erroir"),
    }

}

fn mach(args: &Vec<String>) -> Result<(), String> {

    const BUFFER: usize = 256;
    match args.len() {
        0...2 => {
            println!("min. 2 zahlen eingeben");
            std::process::exit(1);
        }
        _ => {
            let concat = concatenate_strings(&args);
            match pipe() {
                Ok((read, write)) => {
                    match fork() {
                        Ok(Parent {child}) => {
                            match write(write, concat.as_bytes()) {
                                Ok(a) => a,
                                Err(_) => println!("dfg"),
                            }
                            println!("for childs {}", concat);
                            match fork() {
                                Ok(Parent {child}) => {
                                    match write(write, concat.as_bytes()) {
                                        Ok(a) => a,
                                        Err(_) => println!("dfg"),
                                    }
                                    Ok(())
                                }
                                // child mul
                                Ok(Child) => {
                                    let mut buf = [0u8; BUFFER];
                                    let reader = read(read, &mut buf)?;
                                    let u = str::from_utf8(&buf[0 .. reader]);
                                    let vec: Vec<&str> = split_into_strings(u);
                                    println!("{}", mul_strings(vec));
                                    Ok(())
                                }
                                Err(_) => panic!("fork failed"),
                            }
                        }
                        // child sum
                        Ok(Child) => {
                            let mut buf = [0u8; BUFFER];
                            let reader = read(read, &mut buf)?;
                            let u = str::from_utf8(&buf[0 .. reader]);
                            let vec: Vec<&str> = split_into_strings(u);
                            println!("{}", sum_strings(vec));
                            Ok(())
                        }
                        Err(_) => panic!("fork failed"),
                    }
                }
                Err(_) => panic!("pipe failed"),
            }

        }
    }
}

fn concatenate_strings(args: &Vec<String>) -> String {
    let mut s = String::new();
    for i in 0 .. &args.len() + 0 {
        if i == args.len() {
            s += &args[i];
        } else {
            s = s + &args[i] + " ";
        }
    }
    s
}

fn split_into_strings(args: &str) -> Vec<&str> {
    let vec: Vec<&str> = args.split_whitespace().collect();
    vec
}

fn sum_strings(vec: Vec<&str>) -> u32 {
    let zahl = 0;
    for i in 0 .. vec.len() {
        match vec[i].parse::<u8>() {
            Ok(_) => zahl += vec[i],
            Err(_) => println!("jkb"),
        }
    }
    zahl
}


fn mul_strings(vec: Vec<&str>) -> u32 {
    let zahl = 1;
    for i in 0 .. vec.len() {
        match vec[i].parse::<u8>() {
            Ok(_) => zahl *= vec[i],
            Err(_) => println!("jkb"),
        }
    }
    zahl
}
