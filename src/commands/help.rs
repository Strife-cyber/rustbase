pub fn print_help() {
    println!("Available commands:");
    println!("  exit                - exit the program");
    println!("  help                - display this help menu");
    println!("  database <name>     - switch to a database or load it");
}

pub fn print_database_help() {
    println!("Available database commands:");
    println!("  help                            - Show this help message");
    println!("  exit                            - Exit this level");
    println!("  new_store <name> <attributes>   - Create a new store (attributes comma-separated)");
    println!("  delete_store <name>             - Delete a store");
    println!("  list_stores                     - List all stores");
    println!("  save                            - Save the database to a JSON file");
    println!("  export_sql                      - Export the database to a SQL script");
    println!("  store <name>                    - Change to a store or it is created automatically if it does not exist");
}

pub fn print_store_help() {
    println!("Available store commands:");
    println!(" help                                     - Show this help message");
    println!(" exit                                     - Exit this level");
    println!(" new_record <record_map>                  - Create a new record (record map comma-separated Ex: name:John Doe, age: 30) ");
    println!(" delete_record <record_id>                - Delete a record using it's id");
    println!(" list_records                             - List all records");
    println!(" get_record <record_id>                   - Get a particular record using it's id");
    println!(" filter <attribute> <value>               - Filter the store for records using their attribute");
    println!(" filters <attributes> <values>            - Filters the store for records using their values in diverse attribute");
    println!(" operators                                - Display the operators of any query");
    println!(" query <attribute> <operator> <values>    - Query records from the store using a particular operator");
    println!(" sort <attribute> <asc/desc>              - Sort the different record in ascending or descending order");
}
