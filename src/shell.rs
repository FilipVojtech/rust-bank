#![allow(clippy::expect_used)]

use std::io;
use std::io::Write;
use std::process::exit;
use crate::bank_os;
use crate::strings;

pub fn get_input() -> Vec<String> {
    let mut input = String::new();
    print!("> ");
    io::stdout()
        .flush()
        .expect("Couldn't access console");
    io::stdin()
        .read_line(&mut input)
        .expect("Could not read line");
    input = input.trim().to_owned();
    input.split(' ').map(str::to_string).collect()
}

pub fn parse_command(command: Vec<String>) {
    match command[0].as_str() {
        "account" | "acc" => {
            let name = String::new();
            let email = String::new();
            let balance = 0;
            let owed = 0;

            println!("\
                Name: {name}\n\
                Email: {email}\n\
                Password: ***\n\
                \n\
                Balance: {balance}\n\
                Owed: {owed}");
            todo!();
        }
        "send" => {
            if command.len() < 3 { println!("{}", strings::SPEC_ALL_ARGS) } else {
                #[allow(clippy::single_match_else)]
                    let amount = match command[1].parse::<f32>() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Please enter a valid number");
                        return;
                    }
                };
                let user = command[2].clone();
                bank_os::send(amount, &user);
            }
        }
        "receive" => { todo!(); }
        "help" => {
            println!("Available commands:");
            println!("\t             account   Get info about your account");
            println!();
            println!("\tsend [amount] [user]   Send money to another user");
            println!("\t    receive [amount]   Receive money from another user");
            println!();
            println!("\t       loan [amount]   Borrow some money");
            println!("\t      repay [amount]   Repay your loan");
            println!();
            println!("\t               ﷽   :)");
            println!("\t              logout   Logs you out");
            println!("\t       login [email]   Logs you in");
            println!("\t                help   Show this response");
            println!("\t                exit   Exit the bank");
        }
        "﷽" => { println!("In the name of Allah, the Most Gracious, the Most Merciful"); }
        "login" => {
            if command.len() < 2 {
                println!("{}", strings::SPEC_ALL_ARGS);
            } else {
                let password = rpassword::prompt_password("Enter your password: ".to_owned()).expect("Could not get password");
                println!("{:?}", password);
            }
        }
        "exit" => {
            println!("Bye :)");
            exit(0);
        }
        _ => { println!("Unknown command type \"help\" for list of commands."); }
    };
}
