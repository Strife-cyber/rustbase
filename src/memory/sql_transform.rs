use std::fs::File;
use serde_json::Value;
use std::io::{self, Write};
use std::collections::HashMap;
use super::structure::Store;
use crate::memory::database::Database;

impl Store {
    /// Generates a SQL `CREATE TABLE` statement based on the store's attributes.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the SQL table.
    ///
    /// # Returns
    ///
    /// A `Result<String, io::Error>` containing the SQL statement for creating the table.
    pub fn to_sql_create_table(&self, table_name: &str) -> io::Result<String> {
        let mut columns: Vec<String> = self.attributes.iter()
            .map(|attr| format!("{} TEXT", attr))
            .collect();
        columns.sort();

        Ok(format!(
            "CREATE TABLE {} (id INTEGER PRIMARY KEY, {});",
            table_name,
            columns.join(", ")
        ))
    }

    /// Generates a list of SQL `INSERT` statements from the store's data.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the SQL table.
    ///
    /// # Returns
    ///
    /// A `Result<Vec<String>, io::Error>` containing SQL `INSERT` statements for each entry.
    pub fn to_sql_inserts(&self, table_name: &str) -> io::Result<Vec<String>> {
        let mut inserts = Vec::new();

        for (id, data) in &self.values {
            let mut sorted_columns: Vec<&str> = data.keys().map(|k| k.as_str()).collect();
            sorted_columns.sort();

            let values: Vec<String> = sorted_columns.iter()
                .map(|k| match &data[*k] {
                    Value::String(s) => format!("'{}'", s.replace("'", "''")),
                    v => format!("'{}'", v), // Ensure consistent quoting
                })
                .collect();

            let query = format!(
                "INSERT INTO {} (id, {}) VALUES ({}, {});",
                table_name,
                sorted_columns.join(", "),
                id,
                values.join(", ")
            );
            inserts.push(query);
        }

        Ok(inserts)
    }

    /// Generates a SQL `SELECT` statement.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the SQL table.
    /// * `columns` - An optional list of column names to retrieve. If `None`, selects all columns (`*`).
    ///
    /// # Returns
    ///
    /// A `String` containing the SQL `SELECT` statement.
    pub fn to_sql_select(&self, table_name: &str, columns: Option<Vec<&str>>) -> String {
        let selected_columns = columns.map_or("*".to_string(), |cols| cols.join(", "));
        format!("SELECT {} FROM {};", selected_columns, table_name)
    }

    /// Generates a SQL `DELETE` statement.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the SQL table.
    /// * `condition` - A SQL condition specifying which records to delete.
    ///
    /// # Returns
    ///
    /// A `String` containing the SQL `DELETE` statement.
    pub fn to_sql_delete(&self, table_name: &str, condition: &str) -> String {
        format!("DELETE FROM {} WHERE {};", table_name, condition)
    }

    /// Generates a SQL `UPDATE` statement.
    ///
    /// # Arguments
    ///
    /// * `table_name` - The name of the SQL table.
    /// * `id` - The primary key of the row to update.
    /// * `updates` - A hashmap containing column-value pairs to update.
    ///
    /// # Returns
    ///
    /// A `String` containing the SQL `UPDATE` statement.
    pub fn to_sql_update(&self, table_name: &str, id: i64, updates: &HashMap<String, Value>) -> String {
        let set_statements: Vec<String> = updates.iter()
            .map(|(key, value)| match value {
                Value::String(s) => format!("{} = '{}'", key, s.replace("'", "''")),
                _ => format!("{} = {}", key, value),
            })
            .collect();

        format!("UPDATE {} SET {} WHERE id = {};", table_name, set_statements.join(", "), id)
    }
}

impl Database {
    /// Generates a SQL `CREATE DATABASE` statement.
    ///
    /// # Returns
    ///
    /// A `Result<String, io::Error>` containing the SQL statement for creating the database.
    pub fn to_sql_create(&self) -> io::Result<String> {
        Ok(format!("CREATE DATABASE {};", self.name))
    }

    /// Generates a SQL `DROP DATABASE` statement.
    ///
    /// # Returns
    ///
    /// A `Result<String, io::Error>` containing the SQL statement for dropping the database.
    pub fn to_sql_drop(&self) -> io::Result<String> {
        Ok(format!("DROP DATABASE {};", self.name))
    }

    /// Converts the entire database structure into a SQL script.
    ///
    /// # Returns
    ///
    /// A `Result<String, io::Error>` containing the SQL script for creating and populating the database.
    pub fn convert_to_sql(&self) -> io::Result<String> {
        let mut script = String::new();

        script += &self.to_sql_create()?;
        script += "\n";

        for (name, store) in &self.stores {
            script += &store.to_sql_create_table(name)?;
            script += "\n";

            for insert in store.to_sql_inserts(name)? {
                script += &insert;
                script += "\n";
            }
        }

        script += &self.to_sql_drop()?;
        script += "\n";

        Ok(script)
    }

    /// Generates an SQL script file containing the database schema and data.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the file is successfully created and written.
    /// * `Err(io::Error)` if an error occurs during file creation or writing.
    ///
    /// # Behavior
    ///
    /// This function:
    /// 1. Creates a new `.sql` file named after the database.
    /// 2. Calls `convert_to_sql()` to generate SQL statements.
    /// 3. Writes the generated SQL script into the file.
    ///
    /// # Example
    ///
    /// ```rust
    /// let mut database = Database::new("my_database");
    /// database.generate_script().expect("Failed to generate SQL script");
    /// ```
    pub fn generate_script(&self) -> io::Result<()> {
        let mut file = File::create(format!("{}.sql", self.name))?;
        let script = self.convert_to_sql()?;
        file.write_all(script.as_bytes())?;
        Ok(())
    }
}
