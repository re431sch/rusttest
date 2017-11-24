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
        Err(_) => println!("dg"),
    }
}

#[derive(Debug)]
enum Error {
    Nix(nix::Error),
    Utf(std::str::Utf8Error),
}

fn mach() -> Result<(), Error> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() >= 3 {

        const PUFFER: usize = 256;
        let (reader, writer) = pipe().map_err(Error::Nix)?;

        match fork() {
            Ok(Parent {child}) => {
                let concat = concatenate_strings(&args);
                println!("sending to childs: {}", concat);
                write(writer, concat.as_bytes()).map_err(Error::Nix)?;
                waitpid(child, Option::Some(WCONTINUED)).map_err(Error::Nix)?;
                match fork() {
                    Ok(Parent {child}) => {
                        write(writer, concat.as_bytes()).map_err(Error::Nix)?;
                        waitpid(child, Option::Some(WCONTINUED)).map_err(Error::Nix)?;
                        Ok(())
                    }
                    Ok(Child) => {
                        let mut forreader = [0u8; PUFFER];
                        let r = read(reader, &mut forreader).map_err(Error::Nix)?;
                        let s = std::str::from_utf8(&forreader[0 .. r]).map_err(Error::Utf)?;
                        let mul = mul_strings(split_into_strings(s));
                        println!("Mul = {}", mul);
                        Ok(())
                    }
                    Err(_) => {
                        println!("Error: Fork Failed");
                        std::process::exit(1);
                    }
                }
            }
            Ok(Child) => {
                let mut forreader = [0u8; PUFFER];
                let r = read(reader, &mut forreader).map_err(Error::Nix)?;
                let s = std::str::from_utf8(&forreader[0 .. r]).map_err(Error::Utf)?;
                let sum = mul_strings(split_into_strings(s));
                println!("Sum = {}", sum);
                Ok(())
            }
            Err(_) => {
                println!("Error: Fork Failed");
                std::process::exit(1);
            }
        }
    } else {
        println!("Correct usage: number number <number> ...");
        std::process::exit(1);
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
    for i in vec {
        match i.parse::<i32>() {
            Ok(a) => zahl += a,
            Err(_) => println!("parse error"),
        }
    }
    zahl
}


// -------------------------------------------------------------------------------------mul_strings


fn mul_strings(vec: Vec<&str>) -> i32 {
    let mut zahl = 1;
    for i in vec {
        match i.parse::<i32>() {
            Ok(a) => zahl *= a,
            Err(_) => println!("parse error"),
        }
    }
    zahl
}
