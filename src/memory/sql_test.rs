use super::structure::Store;
use super::database::Database;

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;
    use serde_json::{json};
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_to_sql_create_table() {
        let store = Store::make_store("test".to_string(), vec!["name", "age"].into_iter().map(String::from).collect()).unwrap();
        let sql = store.to_sql_create_table("users").unwrap();
        assert_eq!(sql, "CREATE TABLE users (id INTEGER PRIMARY KEY, age TEXT, name TEXT);");
    }

    #[test]
    fn test_to_sql_inserts() {
        let user1 = HashMap::from([
            ("name".to_string(), json!("Alice")),
            ("age".to_string(), json!(25)),
        ]);

        let user2 = HashMap::from([
            ("name".to_string(), json!("Bob")),
            ("age".to_string(), json!(30)),
        ]);

        let mut store = Store::make_store("test".to_string(), vec!["name", "age"].into_iter().map(String::from).collect()).unwrap();
        store.add_record(user1).unwrap();
        store.add_record(user2).unwrap();

        let sql_inserts = store.to_sql_inserts("users").unwrap();
        assert_eq!(sql_inserts.len(), 2);

        assert!(sql_inserts.contains(&"INSERT INTO users (id, age, name) VALUES (0, '25', 'Alice');".to_string()));
        assert!(sql_inserts.contains(&"INSERT INTO users (id, age, name) VALUES (1, '30', 'Bob');".to_string()));
    }

    #[test]
    fn test_to_sql_select() {
        let store = Store::make_store("test".to_string(), vec!["name", "age"].into_iter().map(String::from).collect()).unwrap();

        let sql_all = store.to_sql_select("users", None);
        assert_eq!(sql_all, "SELECT * FROM users;");

        let sql_specific = store.to_sql_select("users", Some(vec!["name", "age"]));
        assert_eq!(sql_specific, "SELECT name, age FROM users;");
    }

    #[test]
    fn test_to_sql_delete() {
        let store = Store::make_store("test".to_string(), HashSet::new()).unwrap();
        let sql_delete = store.to_sql_delete("users", "id = 1");
        assert_eq!(sql_delete, "DELETE FROM users WHERE id = 1;");
    }

    #[test]
    fn test_to_sql_update() {
        let store = Store::make_store("test".to_string(), HashSet::new()).unwrap();
        let updates = HashMap::from([
            ("name".to_string(), json!("Charlie")),
            ("age".to_string(), json!(28)),
        ]);
        let sql_update = store.to_sql_update("users", 1, &updates);
        assert_eq!(sql_update, "UPDATE users SET name = 'Charlie', age = 28 WHERE id = 1;");
    }

    #[test]
    fn test_database_to_sql_create() {
        let db = Database { name: "test_db".to_string(), stores: HashMap::new() };
        let sql_create = db.to_sql_create().unwrap();
        assert_eq!(sql_create, "CREATE DATABASE test_db;");
    }

    #[test]
    fn test_database_to_sql_drop() {
        let db = Database { name: "test_db".to_string(), stores: HashMap::new() };
        let sql_drop = db.to_sql_drop().unwrap();
        assert_eq!(sql_drop, "DROP DATABASE test_db;");
    }

    #[test]
    fn test_database_convert_to_sql() {
        let store_values = HashMap::from([
            (1, HashMap::from([
                ("name".to_string(), json!("Alice")),
                ("age".to_string(), json!(25)),
            ])),
        ]);

        let store = Store {
            id: 0,
            name: "test_store".to_string(),
            attributes: vec!["name", "age"].into_iter().map(String::from).collect(),
            values: store_values,
        };

        let db = Database {
            name: "test_db".to_string(),
            stores: HashMap::from([("users".to_string(), store)]),
        };

        let sql_script = db.convert_to_sql().unwrap();
        assert!(sql_script.contains("CREATE DATABASE test_db;"));
        assert!(sql_script.contains("CREATE TABLE users (id INTEGER PRIMARY KEY, age TEXT, name TEXT);"));
        assert!(sql_script.contains("INSERT INTO users (id, age, name) VALUES (1, '25', 'Alice');"));
        assert!(sql_script.contains("DROP DATABASE test_db;"));
    }

    #[test]
    fn test_generate_script() {
        let db = Database { name: "test_db".to_string(), stores: HashMap::new() };
        db.generate_script().unwrap();

        let file_path = "test_db.sql";
        assert!(fs::metadata(file_path).is_ok());

        fs::remove_file(file_path).unwrap(); // Clean up test file
    }
}
