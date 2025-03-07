use std::io;
use std::io::{BufRead, Write};
use crate::memory::database::Database;
use crate::commands::help::{print_database_help};
use crate::commands::database_functions::{handle_delete_store, handle_list_stores, handle_move_to_store, handle_new_store, handle_save_database};

pub fn run_database_command_loop(mut database: Database) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{}> ", database.name);
        stdout.flush()?;
        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() { continue; }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "exit" => break,
            "help" => print_database_help(),
            "save" => handle_save_database(&mut database),
            "list_stores" => handle_list_stores(&mut database),
            "new_store" => handle_new_store(&mut database, &parts)?,
            "delete_store" => handle_delete_store(&mut database, &parts)?,
            "store" => handle_move_to_store(&mut database, &parts)?,
            _ => println!("Unknown command: {}. Type 'help' for a list of commands.", command)
        }
    }

    println!("Let's go down a level!");
    Ok(())
}
