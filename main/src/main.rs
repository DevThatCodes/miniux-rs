use std::{fs, io, process::exit};

use colored::Colorize;

#[derive(Debug)]
enum ExitMessages {
    ByUser,
    Crashed,
    InvalidUser(String)
}

fn exit_(reason: ExitMessages) {
    let exit_reason = match reason {
        ExitMessages::ByUser => {("successfully exited Miniux".to_string(), 0)}
        ExitMessages::Crashed => {("uh oh, it seems Miniux has crashed".to_string(), 2)}
        ExitMessages::InvalidUser(user) => {(format!("user: {} does not exist", user), 1)}
    };
    println!("{}", exit_reason.0);
    exit(exit_reason.1)
}

fn main() {
    let version = "Miniux 0.10 (Rat)";
    
    // getting the file

    let jsondata = json::parse(&fs::read_to_string("miniux-rat").unwrap()).unwrap();

    // username

    let mut user = String::from("");
    println!("welcome to {}, what is your username?", version.truecolor(100, 255, 100));
    let _ = io::stdin().read_line(&mut user);
    user = user.trim().to_string();
    if !jsondata["users"].to_string().contains(&format!("\"{}\"", user)) {
        exit_(ExitMessages::InvalidUser(user.clone()))
    }

    // password, TODO: if user exists

    let mut password = String::from("");
    println!("hello {}, what is your password?", jsondata["users"][user.clone()]["displayname"].to_string().truecolor(100, 255, 100));
    let _ = io::stdin().read_line(&mut password);
    password = password.trim().to_string();
    // check if password is correct
    if password == jsondata["users"][user]["password"].to_string() {
        println!("password correct!")
    } 
    // main loop
    
    let mut running = true;
    while running {
        running = false 
    }
    
    exit_(ExitMessages::ByUser)
}

// users:
//  - username:
//   - displayname,
//   - password
//   - privatefiles:
//      - name:
//         - type
//         - content,
// files:
//  - name:
//     - type,
//     - content
