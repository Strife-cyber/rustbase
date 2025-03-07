use std::collections::HashMap;
use std::fs::File;
use std::io::{self, Read, Write};
use serde_json;
use crate::memory::structure::Store;
use super::database::Database;

impl Database {
    /// Loads the database from a JSON file if it exists.
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice that holds the name of the database file.
    ///
    /// # Returns
    ///
    /// * `Ok(Database)` - A database instance loaded from the file.
    /// * `Err(io::Error)` - If the file cannot be read or parsed.
    pub fn load(name: &str) -> io::Result<Database> {
        let mut file = File::open(format!("{}.json", name))?;
        let mut json = String::new();
        file.read_to_string(&mut json)?;
        let map: HashMap<String, Store> = serde_json::from_str(&json)?;

        Ok(Database { name: name.to_string(), stores: map })
    }

    /// Stores the current database values in a JSON file.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the database is successfully stored.
    /// * `Err(io::Error)` - If the file cannot be created or written to.
    pub fn store(&mut self) -> io::Result<()> {
        let json = serde_json::to_string(&self.stores)?;
        let mut file = File::create(format!("{}.json", self.name))?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
