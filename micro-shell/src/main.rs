use std::io::{self, Write};
use std::process::{Command, Stdio, Child};
use std::env;
use std::path::Path;



fn main() -> std::io::Result<()> {
    let stdout = io::stdout();
    let stdin = io::stdin();


    // Shell input loop : 1 input : 1 loop. Loop named for return statement
    'shell_loop:loop {
        
        let mut user_input = String::with_capacity(256);
        {
            let mut handle = stdout.lock();
            handle.write_all(b"rs > ")?;
            handle.flush()?;
        }

        // User Input: On prend une référence mutable
        stdin.read_line(&mut user_input)?;

        // Peekable because we have to be able to check if next() element exists
        let mut commands = user_input.split("|").map(|x| x.trim()).peekable();
        let mut user_input = String::with_capacity(0);
        let mut previous_command: Option<Child> = None;


        // Commands pipe-separated loop
        while let Some(command) = commands.next() {

            if command == "" { continue }

            let mut args = command.split(" ").map(|x| x.trim());
            let cmd = args.next().expect("Input command split problem");

            match cmd {
                "" => continue,

                "exit" => break 'shell_loop,

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

                    let stdin = previous_command.map_or(
                        Stdio::inherit(),
                        |output: Child| Stdio::from(output.stdout.unwrap())
                    );

                    let stdout = if commands.peek().is_some() { // another pipe after this command
                        Stdio::piped()
                    } else {
                        Stdio::inherit()
                    };

                    let mut process = Command::new(cmd)
                        .args(args)
                        .stdin(stdin)
                        .stdout(stdout)
                        .spawn();

                    match process {
                        Ok(mut process) => {
                            process.wait();
                            previous_command = Some(process);
                        },
                        Err(e) => {
                            previous_command = None;
                            eprintln!("{}", e);
                        },
                    };

                }    
            }
        }
    }
    Ok(())
}




// Exercice 8 : simple pipe
fn ex_8_pipe() {
    println!("Ex Pipe : ls | cat (prints ls vertically)");
    let cmd = Command::new("ls").stdout(Stdio::piped()).spawn().expect("ls failed to execute");
    let stdout = Stdio::from(cmd.stdout.expect("Failed"));
    let mut process = Command::new("cat").stdin(stdout).spawn().expect("Piped command failed to execute");
    process.wait();
}


