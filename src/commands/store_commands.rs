use std::io;
use std::io::{BufRead, Write};
use crate::memory::structure::Store;
use crate::commands::help::print_store_help;
use crate::commands::store_functions::{handle_delete_record, handle_filter, handle_filters,
                                       handle_get_record, handle_list_records, handle_new_record,
                                       handle_print_operators, handle_query, handle_sort};

pub fn run_store_command_loop(store: &mut Store) -> io::Result<()> {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    loop {
        print!("{}> ", store.name);
        stdout.flush()?;
        let mut input = String::new();
        stdin.lock().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() { continue; }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0].to_lowercase();

        match command.as_str() {
            "exit" => break,
            "help" => print_store_help(),
            "new_record" => handle_new_record(store, &parts)?,
            "delete_record" => handle_delete_record(store, &parts)?,
            "list_records" => handle_list_records(store)?,
            "get_record" => handle_get_record(store, &parts)?,
            "filter" => handle_filter(store, &parts)?,
            "filters" => handle_filters(store, &parts)?,
            "operators" => handle_print_operators(),
            "query" => handle_query(store, &parts)?,
            "sort" => handle_sort(store, &parts)?,
            _ => println!("Unknown command: {}. Type 'help' for a list of commands.", command)
        }
    }

    println!("Let's step down and go back to the database!");
    Ok(())
}