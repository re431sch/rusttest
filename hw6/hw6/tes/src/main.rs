extern crate nix;

use nix::unistd::ForkResult::Parent;
use nix::unistd::fork;
use nix::unistd::pipe;
use nix::unistd::write;
use nix::unistd::read;
use nix::sys::wait::WCONTINUED;
use nix::sys::wait::waitpid;

fn main() {
    match mach() {
        Ok(a) => a,
        Err(e) => println!("{}",e),
    }
}

fn mach() -> Result<(), &'static str> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 {

        // match pipe
        match pipe() {
            Ok((reader, writer)) => {
                const PUFFER: usize = 256;
        // match fork
                match fork() {
                    Ok(Parent {child}) => {
                        let concat = concatenate_strings(&args);
                        println!("sending to childs: {}", concat);
        // match write
                        match write(writer, concat.as_bytes()) {
                            Ok(_) => (),
                            Err(e) => println!("{}",e),
                        }
                        match waitpid(child, Option::Some(WCONTINUED)) {
                            Ok(_) => (),
                            Err(e) => println!("{}",e),
                        }
        // match fork
                        match fork() {
                            Ok(Parent {child}) => {
                                match write(writer, concat.as_bytes()) {
                                    Ok(_) => (),
                                    Err(e) => println!("{}",e),
                                }
                                match waitpid(child, Option::Some(WCONTINUED)) {
                                    Ok(_) => (),
                                    Err(e) => println!("{}",e),
                                }
                            }
// child2 -----------------------------------------------------------------------------
                            Ok(_) => {
                                let mut forreader = [0u8; PUFFER];
                                match read(reader, &mut forreader) {
                                    Ok(a) => {
                                        match std::str::from_utf8(&forreader[0 .. a]) {
                                            Ok(sis) => {
                                                let mul = mul_strings(split_into_strings(sis));
                                                println!("Mul = {}", mul);
                                            }
                                            Err(e) => println!("{}",e),
                                        }
                                    }
                                    Err(e) => println!("{}",e),
                                }
                            }
// child2 ---------------------------------------------------------------------------
                            Err(e) => println!("{}",e),
                        }
                    }
// child1 ----------------------------------------------------------------------
                    Ok(_) => {
                        let mut forreader = [0u8; PUFFER];
                        match read(reader, &mut forreader) {
                            Ok(a) => {
                                match std::str::from_utf8(&forreader[0 .. a]) {
                                    Ok(sis) => {
                                        let sum = sum_strings(split_into_strings(sis));
                                        println!("Sum = {}", sum);
                                    }
                                    Err(e) => println!("{}",e),
                                }
                            }
                            Err(e) => println!("{}",e),
                        }

                    }
// child1 ----------------------------------------------------------------------
                    Err(e) => println!("{}",e),
                }
                Ok(())
            }
            Err(_) => Err("pipe fail"),
        }
    } else {
        Err("kj")
    }
}


// -----------------------------------------------------------------------------concatenate_strings


fn concatenate_strings(args: &Vec<String>) -> String {
    let mut s = String::new();
    for i in 0 .. args.len() {
        if i == args.len() {
            s += &args[i];
        } else {
            s = s + &args[i] + " ";
        }
    }
    s
}


// ------------------------------------------------------------------------------split_into_strings


fn split_into_strings(args: &str) -> Vec<&str> {
    let vec: Vec<&str> = args.split_whitespace().collect();
    vec
}


// -------------------------------------------------------------------------------------sum_strings


fn sum_strings(vec: Vec<&str>) -> i32 {
    let mut zahl = 0;
    for i in 1 .. vec.len() {
        match vec[i].parse::<i32>() {
            Ok(a) => zahl += a,
            Err(_) => println!("parse error"),
        }
    }
    zahl
}


// -------------------------------------------------------------------------------------mul_strings


fn mul_strings(vec: Vec<&str>) -> i32 {
    let mut zahl = 1;
    for i in 1 .. vec.len() {
        match vec[i].parse::<i32>() {
            Ok(a) => zahl *= a,
            Err(_) => println!("parse error"),
        }
    }
    zahl
}
