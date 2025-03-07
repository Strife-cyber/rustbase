use std::io;
use serde_json::{json, Value};
use crate::memory::structure::Store;
use std::collections::{HashMap, HashSet};
use crate::memory::complex::QueryOperator;

pub fn handle_new_record(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 2 {
        println!("Usage: new_record <attribute:value,...>");
        println!("Example: new_record name:John Doe, age:30");
        return Ok(());
    }

    // Parse the record map from the input (parts[1] onwards)
    let record_input = parts[1..].join(" ");
    let record_pairs: Vec<&str> = record_input.split(',').map(|s| s.trim()).collect();

    let mut record: HashMap<String, Value> = HashMap::new();
    for pair in record_pairs {
        let parts: Vec<&str> = pair.splitn(2, ':').collect();
        if parts.len() != 2 {
            println!("Invalid record format: '{}'. Use 'attribute:value'.", pair);
            return Ok(());
        }

        let key = parts[0].trim().to_string();
        let value_str = parts[1].trim();

        // Attempt to infer the value type (string, number, boolean)
        let value = if let Ok(num) = value_str.parse::<i64>() {
            json!(num)
        } else if let Ok(float) = value_str.parse::<f64>() {
            json!(float)
        } else if value_str.to_lowercase() == "true" || value_str.to_lowercase() == "false" {
            json!(value_str.to_lowercase() == "true")
        } else {
            json!(value_str) // Default to string
        };

        record.insert(key, value);
    }

    // Add the record to the store
    match store.add_record(record) {
        Ok(id) => println!("Record added with ID: {}", id),
        Err(e) => println!("Failed to add record: {}", e),
    }

    Ok(())
}

pub fn handle_delete_record(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 2 {
        println!("Usage: delete_record <record_id>");
        return Ok(());
    }

    let record_id: i64 = match parts[1].parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid record ID: '{}'. Must be an integer.", parts[1]);
            return Ok(());
        }
    };

    match store.delete_record(record_id) {
        Ok(()) => println!("Record {} deleted successfully.", record_id),
        Err(e) => println!("Failed to delete record {}: {}", record_id, e),
    }

    Ok(())
}

pub fn handle_list_records(store: &mut Store) -> io::Result<()> {
    let records = store.get_all_records()?;

    if records.is_empty() {
        println!("No records found in store '{}'.", store.name);
    } else {
        println!("Records in store '{}':", store.name);
        for (id, record) in records {
            // Convert Box<dyn Any> back to Value for display (assuming we can downcast or use debug)
            let record_str: Vec<String> = record
                .iter()
                .map(|(k, v)| format!("{}: {:?}", k, v)) // Using Debug for simplicity
                .collect();
            println!("ID: {} - {{{}}}", id, record_str.join(", "));
        }
    }

    Ok(())
}

/// Retrieves and displays a specific record by its ID.
pub fn handle_get_record(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 2 {
        println!("Usage: get_record <record_id>");
        return Ok(());
    }

    let record_id: i64 = match parts[1].parse() {
        Ok(id) => id,
        Err(_) => {
            println!("Invalid record ID: '{}'. Must be an integer.", parts[1]);
            return Ok(());
        }
    };

    match store.get_record(record_id) {
        Ok(record) => {
            let record_str: Vec<String> = record
                .iter()
                .map(|(k, v)| format!("{}: {}", k, v))
                .collect();
            println!("Record {}: {{{}}}", record_id, record_str.join(", "));
        }
        Err(e) => println!("Failed to get record {}: {}", record_id, e),
    }

    Ok(())
}

pub fn handle_print_operators() {
    use QueryOperator::*;

    let operators = [Eq, Neq, Gt, Lt, Ge, Le, Contains];
    println!("{:#?}", operators);
}

/// Handles the "filter" command: Filters records by a single attribute and value.
pub fn handle_filter(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 3 {
        println!("Usage: filter <attribute> <value>");
        println!("Example: filter name John");
        return Ok(());
    }

    let attribute = parts[1];
    let value = parts[2];

    match store.filter(attribute, value) {
        Ok(filtered) => {
            if filtered.is_empty() {
                println!("No records found matching {}: '{}'.", attribute, value);
            } else {
                println!("Filtered records:");
                for (id, record) in filtered {
                    let record_str: Vec<String> = record
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect();
                    println!("ID: {} - {{{}}}", id, record_str.join(", "));
                }
            }
        }
        Err(e) => println!("Filter failed: {}", e),
    }

    Ok(())
}

/// Handles the "filters" command: Filters records by multiple attributes and values.
pub fn handle_filters(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 3 {
        println!("Usage: filters <attributes> <values>");
        println!("Example: filters name,age John,30");
        return Ok(());
    }

    let attributes: HashSet<&str> = parts[1].split(',').map(|s| s.trim()).collect();
    let values: HashSet<&str> = parts[2].split(',').map(|s| s.trim()).collect();

    if attributes.len() != values.len() {
        println!("Number of attributes ({}) and values ({}) must match.", attributes.len(), values.len());
        return Ok(());
    }

    match store.filter_attributes(&attributes, &values) {
        Ok(filtered) => {
            if filtered.is_empty() {
                println!("No records found matching the specified attributes and values.");
            } else {
                println!("Filtered records:");
                for (id, record) in filtered {
                    let record_str: Vec<String> = record
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect();
                    println!("ID: {} - {{{}}}", id, record_str.join(", "));
                }
            }
        }
        Err(e) => println!("Filters failed: {}", e),
    }

    Ok(())
}

/// Handles the "operators" command: Displays available query operators.
pub fn handle_operators() -> io::Result<()> {
    println!("Available query operators:");
    println!("  eq       - Equal to");
    println!("  neq      - Not equal to");
    println!("  gt       - Greater than");
    println!("  lt       - Less than");
    println!("  ge       - Greater than or equal to");
    println!("  le       - Less than or equal to");
    println!("  contains - Checks if a string contains a substring");
    Ok(())
}

/// Handles the "query" command: Queries records using an attribute, operator, and value.
pub fn handle_query(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 4 {
        println!("Usage: query <attribute> <operator> <value>");
        println!("Example: query age gt 25");
        return Ok(());
    }

    let attribute = parts[1];
    let operator_str = parts[2].to_lowercase();
    let value_str = parts[3];

    // Parse the value with type inference
    let value = if let Ok(num) = value_str.parse::<i64>() {
        json!(num)
    } else if let Ok(float) = value_str.parse::<f64>() {
        json!(float)
    } else if value_str.to_lowercase() == "true" || value_str.to_lowercase() == "false" {
        json!(value_str.to_lowercase() == "true")
    } else {
        json!(value_str)
    };

    let operator = match operator_str.as_str() {
        "eq" => QueryOperator::Eq,
        "neq" => QueryOperator::Neq,
        "gt" => QueryOperator::Gt,
        "lt" => QueryOperator::Lt,
        "ge" => QueryOperator::Ge,
        "le" => QueryOperator::Le,
        "contains" => QueryOperator::Contains,
        _ => {
            println!("Invalid operator: '{}'. Type 'operators' for a list.", operator_str);
            return Ok(());
        }
    };

    match store.query(attribute, operator, value) {
        Ok(results) => {
            if results.is_empty() {
                println!("No records found for query {} {} '{}'.", attribute, operator_str, value_str);
            } else {
                println!("Query results:");
                for (id, record) in results {
                    let record_str: Vec<String> = record
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect();
                    println!("ID: {} - {{{}}}", id, record_str.join(", "));
                }
            }
        }
        Err(e) => println!("Query failed: {}", e),
    }

    Ok(())
}

/// Handles the "sort" command: Sorts records by an attribute in ascending or descending order.
pub fn handle_sort(store: &mut Store, parts: &[&str]) -> io::Result<()> {
    if parts.len() < 3 {
        println!("Usage: sort <attribute> <asc/desc>");
        println!("Example: sort age asc");
        return Ok(());
    }

    let attribute = parts[1];
    let direction = parts[2].to_lowercase();
    let ascending = match direction.as_str() {
        "asc" => true,
        "desc" => false,
        _ => {
            println!("Invalid sort direction: '{}'. Use 'asc' or 'desc'.", direction);
            return Ok(());
        }
    };

    match store.sort_by(attribute, ascending) {
        Ok(sorted) => {
            if sorted.is_empty() {
                println!("No records to sort in store '{}'.", store.name);
            } else {
                println!("Sorted records ({} {}):", attribute, if ascending { "asc" } else { "desc" });
                for (id, record) in sorted {
                    let record_str: Vec<String> = record
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect();
                    println!("ID: {} - {{{}}}", id, record_str.join(", "));
                }
            }
        }
        Err(e) => println!("Sort failed: {}", e),
    }

    Ok(())
}
