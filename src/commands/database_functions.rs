use std::io;
use std::collections::HashSet;
use crate::memory::database::Database;
use crate::commands::store_commands::run_store_command_loop;

pub fn handle_new_store(database: &mut Database, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 3 {
        println!("Usage: new_store <name> <attributes>");
        return Ok(());
    }

    let name = parts[1].to_string();
    let attributes: HashSet<String> = parts[2].split(",").map(|x| x.trim().to_string()).collect();
    database.add_store(name, attributes)?;
    println!("Store '{}' created.", parts[1]);
    Ok(())
}

pub fn handle_delete_store(database: &mut Database, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 2 {
        println!("Usage: delete_store <name>");
        return Ok(());
    }
    database.delete_store(parts[1]);
    println!("Store '{}' deleted.", parts[1]);
    Ok(())
}

pub fn handle_list_stores(database: &mut Database) {
    if database.stores.is_empty() {
        println!("No stores found.");
    } else {
        println!("Stores:");
        for name in database.stores.keys() {
            println!("- {}", name);
        }
    }
}

pub fn handle_save_database(database: &mut Database) {
    database.store().unwrap();
}

pub fn export_database(database: &mut Database) {
    database.generate_script().unwrap()
}

pub fn handle_move_to_store(database: &mut Database, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 2 {
        println!("Usage: store <name>");
        return Ok(());
    }

    let store_name = parts[1];

    // If the store doesn’t exist, create it
    if !database.stores.contains_key(store_name) {
        database.add_store(store_name.to_string(), HashSet::new())?;
        println!("Store '{}' created.", store_name);
    }

    // Get a mutable reference to the store and run the command loop
    if let Some(store) = database.get_store_mut(store_name) {
        run_store_command_loop(store)?;
    } else {
        println!("Store '{}' not found after creation. This should not happen.", store_name);
    }

    Ok(())
}
