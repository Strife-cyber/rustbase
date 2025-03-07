use super::database::*;

#[cfg(test)]
mod database_tests {
    use super::*;
    use std::collections::{HashSet};

    fn setup_database() -> Database {
        Database::new("test_db").unwrap()
    }

    #[test]
    fn test_add_store() {
        let mut db = setup_database();
        let mut attributes = HashSet::new();
        attributes.insert("name".to_string());
        attributes.insert("age".to_string());

        assert!(db.add_store("users".to_string(), attributes).is_ok());
        assert!(db.get_store("users").is_some());
    }

    #[test]
    fn test_get_store() {
        let mut db = setup_database();
        let mut attributes = HashSet::new();
        attributes.insert("email".to_string());

        db.add_store("contacts".to_string(), attributes).unwrap();
        let store = db.get_store("contacts");
        assert!(store.is_some());
    }

    #[test]
    fn test_get_store_mut() {
        let mut db = setup_database();
        let mut attributes = HashSet::new();
        attributes.insert("score".to_string());

        db.add_store("leaderboard".to_string(), attributes).unwrap();
        let store_mut = db.get_store_mut("leaderboard");
        assert!(store_mut.is_some());
    }

    #[test]
    fn test_delete_store() {
        let mut db = setup_database();
        let mut attributes = HashSet::new();
        attributes.insert("location".to_string());

        db.add_store("places".to_string(), attributes).unwrap();
        assert!(db.get_store("places").is_some());

        db.delete_store("places");
        assert!(db.get_store("places").is_none());
    }

    #[test]
    fn test_store_and_load_database() {
        let mut db = setup_database();
        let mut attributes = HashSet::new();
        attributes.insert("username".to_string());

        db.add_store("accounts".to_string(), attributes).unwrap();
        db.store().unwrap();

        let loaded_db = Database::load("test_db").unwrap();
        assert!(loaded_db.get_store("accounts").is_some());
    }
}
