use std::io;
use std::cmp::Ordering;
use super::structure::Store;
use serde_json::{json, Value};
use std::collections::{HashMap, HashSet};

/// Enumeration representing different query operators for filtering values.
#[derive(PartialEq, Debug, Eq)]
#[allow(dead_code)]
pub enum QueryOperator {
    /// Equal to
    Eq,
    /// Not equal to
    Neq,
    /// Greater than
    Gt,
    /// Less than
    Lt,
    /// Greater than or equal to
    Ge,
    /// Less than or equal to
    Le,
    /// Checks if a string contains a substring
    Contains,
}

impl Store {
    /// Filters values based on a single attribute and search value.
    ///
    /// # Arguments
    /// * `attribute` - The attribute to filter by.
    /// * `search_value` - The value to match.
    ///
    /// # Returns
    /// A `HashMap` containing the filtered results.
    pub fn filter(&mut self, attribute: &str, search_value: &str) -> io::Result<HashMap<i64, HashMap<String, Value>>> {
        let mut result = HashMap::new();
        if self.attributes.contains(attribute) {
            let all_values = self.values.clone();
            for (id, data) in all_values {
                if let Some(attr_value) = data.get(attribute) {
                    if attr_value == &json!(search_value) { // Fixed comparison
                        result.insert(id, data);
                    }
                }
            }
            Ok(result)
        } else {
            Err(io::Error::new(io::ErrorKind::NotFound, "attribute not found"))
        }
    }

    /// Filters values based on multiple attributes and their corresponding values.
    ///
    /// # Arguments
    /// * `attributes` - A set of attribute names.
    /// * `search_values` - A set of values corresponding to the attributes.
    ///
    /// # Returns
    /// A `HashMap` containing the filtered results.
    pub fn filter_attributes(&mut self, attributes: &HashSet<&str>, search_values: &HashSet<&str>) -> io::Result<HashMap<i64, HashMap<String, Value>>> {
        let mut result = HashMap::new();

        if attributes.len() != search_values.len() {
            return Err(io::Error::new(io::ErrorKind::InvalidInput, "attributes and values must have the same length"));
        }

        let all_values = self.values.clone();
        for (id, data) in all_values {
            let mut match_found = true;

            for (attribute, search_value) in attributes.iter().zip(search_values.iter()) {
                if let Some(attr_value) = data.get(*attribute) {
                    if attr_value != search_value {
                        match_found = false;
                        break;
                    }
                } else {
                    match_found = false;
                    break;
                }
            }

            if match_found {
                result.insert(id, data);
            }
        }

        Ok(result)
    }

    /// Sorts records based on a given attribute in ascending or descending order.
    ///
    /// # Arguments
    /// * `attribute` - The attribute to sort by.
    /// * `ascending` - If `true`, sorts in ascending order; otherwise, sorts in descending order.
    ///
    /// # Returns
    /// A sorted vector of records.
    pub fn sort_by(&mut self, attribute: &str, ascending: bool) -> io::Result<Vec<(i64, HashMap<String, Value>)>> {
        let mut records: Vec<(i64, HashMap<String, Value>)> = self.values.clone().into_iter().collect();

        records.sort_by(|a, b| {
            let val_a = a.1.get(attribute);
            let val_b = b.1.get(attribute);

            match (val_a, val_b) {
                (Some(Value::Number(num_a)), Some(Value::Number(num_b))) => {
                    let ord = num_a.as_f64().unwrap_or(0.0).partial_cmp(&num_b.as_f64().unwrap_or(0.0)).unwrap_or(Ordering::Equal);
                    if ascending { ord } else { ord.reverse() }
                },
                (Some(Value::String(str_a)), Some(Value::String(str_b))) => {
                    let ord = str_a.cmp(str_b);
                    if ascending { ord } else { ord.reverse() }
                },
                _ => Ordering::Equal,
            }
        });

        Ok(records)
    }

    /// Queries the dataset using various operators (e.g., equality, greater than, etc.).
    ///
    /// # Arguments
    /// * `attribute` - The attribute to query.
    /// * `operator` - The comparison operator.
    /// * `value` - The value to compare against.
    ///
    /// # Returns
    /// A `HashMap` containing the matching records.
    pub fn query(&mut self, attribute: &str, operator: QueryOperator, value: Value) -> io::Result<HashMap<i64, HashMap<String, Value>>> {
        let mut result = HashMap::new();

        for (id, data) in self.values.clone() {
            if let Some(attr_value) = data.get(attribute) {
                let condition_met = match (&operator, attr_value, &value) {
                    (QueryOperator::Eq, a, b) => a == b,
                    (QueryOperator::Neq, a, b) => a != b,
                    (QueryOperator::Gt, Value::Number(a), Value::Number(b)) => a.as_f64().unwrap_or(0.0) > b.as_f64().unwrap_or(0.0),
                    (QueryOperator::Lt, Value::Number(a), Value::Number(b)) => a.as_f64().unwrap_or(0.0) < b.as_f64().unwrap_or(0.0),
                    (QueryOperator::Ge, Value::Number(a), Value::Number(b)) => a.as_f64().unwrap_or(0.0) >= b.as_f64().unwrap_or(0.0),
                    (QueryOperator::Le, Value::Number(a), Value::Number(b)) => a.as_f64().unwrap_or(0.0) <= b.as_f64().unwrap_or(0.0),
                    (QueryOperator::Contains, Value::String(a), Value::String(b)) => a.contains(b),
                    _ => false,
                };

                if condition_met {
                    result.insert(id, data);
                }
            }
        }

        Ok(result)
    }
}
