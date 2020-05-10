use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::env;
use std::path::Path;


// Exercice 8 : simple pipe
fn ex_8_pipe() {
    println!("Ex Pipe :");
    let cmd = Command::new("ls").stdout(Stdio::piped()).spawn().expect("ls not working");
    let stdout = Stdio::from(cmd.stdout.expect("Failed"));
    // let process = Command::new("grep").arg("Hello").stdin(stdout).spawn().expect("Grepping failed to execute");
    let mut process = Command::new("cat").stdin(stdout).spawn().expect("Grepping failed to execute");
    process.wait();
}


fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let stdin = io::stdin();
    let mut user_input = String::with_capacity(256);

    loop {
        let mut user_input_not_trimmed = String::with_capacity(256);
        {
            let mut handle = stdout.lock();
            handle.write_all(b"rs > ")?;
            handle.flush()?;
        }

        // User Input: On prend une référence mutable
        stdin.read_line(&mut user_input_not_trimmed)?;

        user_input = String::from(user_input_not_trimmed.trim());
        user_input_not_trimmed = String::with_capacity(0);

        if user_input == "" { continue }

        let mut args = user_input.split(" ");
        let cmd = args.next().expect("Input command split problem");

        match cmd {
            "" => continue,

            "exit" => break,

            "ex8" => {
                ex_8_pipe();
            },

            "cd" => {
                let new_dir = args.peekable().peek().map_or("/", |x| *x);
                let path = Path::new(new_dir);
                if let Err(e) = env::set_current_dir(&path) {
                    eprintln!("{}", e);
                }
            },
            
            cmd => {
                let process = Command::new(cmd).args(args).spawn();
                
                match process {
                    Ok(mut process) => { process.wait(); },
                    Err(e) => eprintln!("{}", e),
                };
            }
            
        }

    }

    Ok(())
}

