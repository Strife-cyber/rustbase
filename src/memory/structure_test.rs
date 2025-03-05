use super::structure::*;

#[cfg(test)]
mod tests {
    use super::*;
    use std::any::Any;
    use std::collections::HashMap;

    // Helper function to create a new store for testing
    fn new_store() -> Store {
        Store::new().unwrap()
    }

    // Test case to add a record
    #[test]
    fn test_add_record() {
        let mut store = new_store();
        let mut record = HashMap::new();
        record.insert("name".to_string(), Box::new("Alice".to_string()) as Box<dyn Any>);
        record.insert("age".to_string(), Box::new(30) as Box<dyn Any>);

        let record_id = store.add_record(record).unwrap();
        assert_eq!(record_id, 0);
    }

    // Test case to get a record by ID
    #[test]
    fn test_get_record() {
        let mut store = new_store();
        let mut record = HashMap::new();
        record.insert("name".to_string(), Box::new("Bob".to_string()) as Box<dyn Any>);
        record.insert("age".to_string(), Box::new(25) as Box<dyn Any>);

        let record_id = store.add_record(record).unwrap();
        let retrieved_record = store.get_record(record_id).unwrap();

        // Ensure the record is correct
        assert_eq!(retrieved_record.len(), 2);
        assert_eq!(retrieved_record.get("name").unwrap().downcast_ref::<String>().unwrap(), "Bob");
        assert_eq!(retrieved_record.get("age").unwrap().downcast_ref::<i32>().unwrap(), &25);
    }

    // Test case to delete a record
    #[test]
    fn test_delete_record() {
        let mut store = new_store();
        let mut record = HashMap::new();
        record.insert("name".to_string(), Box::new("Charlie".to_string()) as Box<dyn Any>);
        record.insert("age".to_string(), Box::new(22) as Box<dyn Any>);

        let record_id = store.add_record(record).unwrap();
        assert!(store.get_record(record_id).is_ok()); // Record should exist before deletion

        store.delete_record(record_id).unwrap();
        assert!(store.get_record(record_id).is_err()); // Record should be deleted and not found
    }

    // Test case to update a record
    #[test]
    fn test_update_record() {
        let mut store = new_store();
        let mut record = HashMap::new();
        record.insert("name".to_string(), Box::new("Dave".to_string()) as Box<dyn Any>);
        record.insert("age".to_string(), Box::new(35) as Box<dyn Any>);

        let record_id = store.add_record(record).unwrap();

        // Update the record
        let mut updated_record = HashMap::new();
        updated_record.insert("name".to_string(), Box::new("David".to_string()) as Box<dyn Any>);
        updated_record.insert("age".to_string(), Box::new(36) as Box<dyn Any>);

        store.update_record(record_id, updated_record).unwrap();

        // Ensure the updated record is correct
        let updated_record = store.get_record(record_id).unwrap();
        assert_eq!(updated_record.len(), 2);
        assert_eq!(updated_record.get("name").unwrap().downcast_ref::<String>().unwrap(), "David");
        assert_eq!(updated_record.get("age").unwrap().downcast_ref::<i32>().unwrap(), &36);
    }

    // Test case to get all records
    #[test]
    fn test_get_all_records() {
        let mut store = new_store();
        let mut record1 = HashMap::new();
        record1.insert("name".to_string(), Box::new("Eve".to_string()) as Box<dyn Any>);
        record1.insert("age".to_string(), Box::new(28) as Box<dyn Any>);

        let mut record2 = HashMap::new();
        record2.insert("name".to_string(), Box::new("Frank".to_string()) as Box<dyn Any>);
        record2.insert("age".to_string(), Box::new(40) as Box<dyn Any>);

        store.add_record(record1).unwrap();
        store.add_record(record2).unwrap();

        let all_records = store.get_all_records().unwrap();
        assert_eq!(all_records.len(), 2); // Two records added
    }
}