use std::io::{Write,BufRead};
use std::{env,io};


#[derive(Debug)]
pub struct Shell<R, W> {
    pub reader: R,
    pub writer: W,
    pub should_exit: bool,
    pub name: String,
}


impl<R: BufRead, W: Write> Shell<R, W> {
    pub fn new(input: R, output: W, name: String) -> Self {
        Self {
            reader: input,
            writer: output,
            should_exit: false,
            name: name,
        }
    }
    /*pub fn start() -> Result{
        shell_loop(self);
    }*/
    fn shell_loop(self) -> Result {
        loop {
            
        }
    }
    fn prompt(self) -> Result<Option<String>, String> {
        let mut input = String::new();
        let path = env::current_dir().unwrap();
        print!("{} {} >", self.name, path.display());
        io::stdout().flush().unwrap();
        let line = io::stdin().read_line(&mut input).unwrap();

        if line > 0 {
            Ok(Some(input))
        } else {
            Err("ff".to_string())

        }
    }
    /*fn run() {

    }*/
}
