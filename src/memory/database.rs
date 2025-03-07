use std::io;
use super::structure::Store;
use std::collections::{HashMap, HashSet};

/// A `Database` is a collection of stores, each identified by a unique name.
pub struct Database {
    pub name: String,
    pub stores: HashMap<String, Store>,
}

impl Database {
    /// Creates a new empty `Database`.
    ///
    /// # Returns
    ///
    /// Returns a `Database` instance initialized with an empty store collection.
    pub fn new(name: &str) -> io::Result<Database> {
        Ok(Database {
            name: name.to_string(),
            stores: HashMap::new(),
        })
    }

    /// Adds a new store to the database.
    ///
    /// # Parameters
    /// - `name`: The name of the store to be added.
    /// - `attributes`: The set of attributes for the new store.
    ///
    /// # Errors
    /// Returns an error if the store creation fails.
    pub fn add_store(&mut self, name: String, attributes: HashSet<String>) -> io::Result<()> {
        match Store::make_store(name.clone(), attributes) {
            Ok(store) => {
                self.stores.insert(name, store);
                Ok(())
            }
            Err(e) => Err(e),
        }
    }

    /// Retrieves a store by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the store to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a reference to the store, or `None` if no store is found with the given name.
    pub fn get_store(&self, name: &str) -> Option<&Store> {
        self.stores.get(name)
    }

    /// Retrieves a mutable reference to a store by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the store to retrieve.
    ///
    /// # Returns
    ///
    /// Returns an `Option` containing a mutable reference to the store, or `None` if no store is found with the given name.
    pub fn get_store_mut(&mut self, name: &str) -> Option<&mut Store> {
        self.stores.get_mut(name)
    }

    /// Deletes a store by its name.
    ///
    /// # Parameters
    /// - `name`: The name of the store to delete.
    ///
    /// # Returns
    /// This function does not return a value. The store is removed from the database.
    pub fn delete_store(&mut self, name: &str) {
        self.stores.remove(name);
    }
}
