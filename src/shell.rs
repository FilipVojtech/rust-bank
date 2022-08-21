use std::io;
use std::io::Write;
use std::process::exit;

pub(crate) fn get_input() -> String {
    let mut input = String::new();
    print!("> ");
    io::stdout()
        .flush()
        .expect("Couldn't access console");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    input.pop();
    input
}

pub(crate) fn parse(command: &str) {
    match command {
        "send" => {

        }
        "help" => {
            println!("Available commands:");
            println!("\tsend   Send money to another user");
            println!();
            println!("\thelp   Show this response");
            println!("\texit   Exit the bank");
        }
        "exit" => {
            println!("Bye :)");
            exit(0);
        }
        _ => { println!("Unknown command type \"help\" for list of commands."); }
    };
}
