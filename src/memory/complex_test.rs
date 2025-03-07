use maplit::hashmap;
use serde_json::json;
use super::structure::Store;
use std::collections::{HashSet};
use super::complex::QueryOperator;

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_store() -> Store {
        let mut store = Store::make_store(
            "Test Store".to_string(),
            ["name".to_string(), "age".to_string(), "city".to_string()]
                .iter()
                .cloned()
                .collect(),
        )
            .unwrap();

        store.add_record(hashmap! {
            "name".to_string() => json!("Alice"),
            "age".to_string() => json!(30),
            "city".to_string() => json!("Paris")
        }).unwrap();

        store.add_record( hashmap! {
            "name".to_string() => json!("Bob"),
            "age".to_string() => json!(25),
            "city".to_string() => json!("London"),
        }).unwrap();

        store.add_record(hashmap! {
            "name".to_string() => json!("Charlie"),
            "age".to_string() => json!(35),
            "city".to_string() => json!("Paris"),
        }).unwrap();

        store
    }

    #[test]
    fn test_filter() {
        let mut store = create_test_store();
        let result = store.filter("city", "Paris").unwrap();
        assert_eq!(result.len(), 2);
    }

    #[test]
    fn test_filter_non_existent_attribute() {
        let mut store = create_test_store();
        let result = store.filter("country", "France");
        assert!(result.is_err());
    }

    #[test]
    fn test_filter_attributes() {
        let mut store = create_test_store();
        let attributes: HashSet<&str> = ["name", "city"].iter().cloned().collect();
        let values: HashSet<&str> = ["Alice", "Paris"].iter().cloned().collect();
        let result = store.filter_attributes(&attributes, &values).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_sort_by() {
        let mut store = create_test_store();
        let result = store.sort_by("age", true).unwrap();
        let ages: Vec<i64> = result.iter().map(|(_, v)| v["age"].as_i64().unwrap()).collect();
        assert_eq!(ages, vec![25, 30, 35]);
    }

    #[test]
    fn test_query_equal() {
        let mut store = create_test_store();
        let result = store.query("age", QueryOperator::Eq, json!(30)).unwrap();
        assert_eq!(result.len(), 1);
    }

    #[test]
    fn test_query_greater_than() {
        let mut store = create_test_store();
        let result = store.query("age", QueryOperator::Gt, json!(30)).unwrap();
        assert_eq!(result.len(), 1);
    }
}
