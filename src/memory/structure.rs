use std::io;
use serde_json::Value;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// A `Store` is a data structure similar to a table, representing a collection of records with dynamic attributes.
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Store {
    pub(super) id: i64,                                                   // ID used to track the next record ID.
    pub name: String,                                          // Name of the store.
    pub(super) attributes: HashSet<String>,                    // Set of attributes that define the store.
    pub(super) values: HashMap<i64, HashMap<String, Value>>,   // Store's records, keyed by their IDs.
}

impl Store {
    /// Creates a new `Store` with a default name and empty attributes and values.
    ///
    /// # Returns
    ///
    /// Returns a `Store` instance initialized with default values.
    pub fn new() -> io::Result<Store> {
        Ok(Store {
            id: 0i64,
            name: "DEFAULT".to_string(),
            attributes: HashSet::new(),
            values: HashMap::new(),
        })
    }

    /// Creates a new `Store` with the specified name and attributes.
    ///
    /// # Parameters
    /// - `name`: The name of the store.
    /// - `attributes`: The set of attributes that define the store.
    ///
    /// # Returns
    ///
    /// Returns a `Store` instance initialized with the given name and attributes.
    pub fn make_store(name: String, attributes: HashSet<String>) -> io::Result<Store> {
        Ok(Store {
            id: 0i64,
            name,
            attributes,
            values: HashMap::new(),
        })
    }

    /// Adds a new record to the store and validates its attributes.
    ///
    /// # Parameters
    /// - `record`: A `HashMap<String, Box<dyn Any>>` representing the record to add.
    ///
    /// # Returns
    ///
    /// Returns the ID of the newly added record.
    ///
    /// # Errors
    ///
    /// Returns an error if the record’s attributes are invalid.
    pub fn add_record(&mut self, record: HashMap<String, Value>) -> io::Result<i64> {
        let record_id = self.id;
        self.validate_attributes(record.keys().cloned().collect())?;
        self.values.insert(record_id, record);
        self.id += 1;
        Ok(record_id)
    }

    /// Deletes a record from the store using its ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the record to delete.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the record is successfully deleted.
    ///
    /// # Errors
    ///
    /// Returns an error if the record is not found.
    pub fn delete_record(&mut self, id: i64) -> io::Result<()> {
        if self.values.contains_key(&id) {
            self.values.remove(&id);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Record not found"))
        }
    }

    /// Updates an existing record by deleting the old one and adding the new one.
    ///
    /// # Parameters
    /// - `id`: The ID of the record to update.
    /// - `record`: The new record data to replace the existing one.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the record is successfully updated.
    ///
    /// # Errors
    ///
    /// Returns an error if the record is not found.
    pub fn update_record(&mut self, id: i64, record: HashMap<String, Value>) -> io::Result<()> {
        if self.values.contains_key(&id) {
            self.delete_record(id)?;
            self.values.insert(id, record);
            Ok(())
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Record not found"))
        }
    }

    /// Retrieves a record from the store by its ID.
    ///
    /// # Parameters
    /// - `id`: The ID of the record to retrieve.
    ///
    /// # Returns
    ///
    /// Returns a reference to the record if found.
    ///
    /// # Errors
    ///
    /// Returns an error if the record is not found.
    pub fn get_record(&mut self, id: i64) -> io::Result<&HashMap<String, Value>> {
        if self.values.contains_key(&id) {
            Ok(&self.values[&id])
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "Record not found"))
        }
    }

    /// Retrieves all records from the store.
    ///
    /// # Returns
    ///
    /// Returns a `HashMap<i64, HashMap<String, Box<dyn Any>>>` containing all records in the store.
    pub fn get_all_records(&mut self) -> io::Result<HashMap<i64, HashMap<String, Value>>> {
        let mut cloned_records = HashMap::new();

        for (id, record) in self.values.iter() {
            let mut cloned_record = HashMap::new();
            for (key, value) in record.iter() {
                cloned_record.insert(key.clone(), value.clone());
            }
            cloned_records.insert(*id, cloned_record);
        }
        Ok(cloned_records)
    }

    /// Validates the attributes of a record and adds any new attributes to the store.
    ///
    /// # Parameters
    /// - `map_attributes`: A vector of attribute names to validate.
    ///
    /// # Returns
    ///
    /// Returns `Ok(())` if the validation and insertion are successful.
    fn validate_attributes(&mut self, map_attributes: Vec<String>) -> io::Result<()> {
        map_attributes.iter().for_each(|item| {
            if !self.attributes.contains(item) {
                self.attributes.insert(item.clone());
            }
        });
        Ok(())
    }
}
