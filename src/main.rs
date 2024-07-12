use std::{
    env,
    io::{stdin, stdout, Write},
    path::Path,
    process::Command,
};

fn main() {
    loop {
        print!("> ");
        stdout().flush().unwrap();

        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut parts = input.split_whitespace();
        let cmd = parts.next().unwrap();
        let args = parts;

        match cmd {
            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let root = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(root) {
                    eprintln!("{}", e);
                }
            }
            "exit" => {
                return;
            }
            cmd => {
                let child = Command::new(cmd).args(args).spawn();

                // gracefully handle malformed user input
                match child {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    }
                    Err(e) => eprintln!("{}", e),
                };
            }
        }
    }
}
