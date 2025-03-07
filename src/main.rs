mod memory;
mod commands;

use std::io::{stdin, stdout, BufRead, Write};
use crate::commands::database_command::run_database_command_loop;
use crate::commands::help::print_help;
use crate::memory::database::Database;

fn main() {
    let stdin = stdin();
    let mut stdout = stdout();

    loop {
        print!("> ");
        stdout.flush().unwrap();
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() { continue; }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "exit" => break,
            "help" => print_help(),
            "database" => handle_database(&parts),
            _ => print_help()
        }
    }
}

fn handle_database(parts: &[&str]) {
    if parts.len() < 2 {
        println!("Usage: database <name>");
        return;
    }

    let database_name = parts[1];

    if let database = Database::load(database_name) {
        println!("Database loaded successfully from json file!");
        run_database_command_loop(database.unwrap()).unwrap();
        return;
    }
    println!("Database file not found!");
    println!("Creating database for this setup");
    run_database_command_loop(Database::new(database_name).unwrap()).unwrap()
}
