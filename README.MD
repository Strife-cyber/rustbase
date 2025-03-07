# Rustbase: A Simple In-Memory Database

**Rustbase** is a lightweight, in-memory database written in Rust, designed for managing structured data with a simple command-line interface (CLI). It supports creating stores (similar to tables), adding and manipulating records, filtering and querying data, sorting records, and exporting to JSON or SQL formats. Built with performance and usability in mind, Rustbase leverages Rust's type safety and memory management for a robust data management experience.

## Features

- **Store Management**: Create, delete, and switch between stores within a database.
- **Record Operations**: Add, delete, update, and retrieve records with flexible attribute-value pairs.
- **Filtering**: Filter records by single or multiple attributes.
- **Querying**: Query records using operators like eq, gt, contains, etc.
- **Sorting**: Sort records by attributes in ascending or descending order.
- **Persistence**: Save the database to JSON files and export to SQL scripts.
- **Interactive CLI**: Intuitive command-line interface for managing stores and records.

## Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (latest stable version recommended)
- Cargo (included with Rust)

### Steps

1. Clone the repository:

   bash

   CollapseWrapCopy

   `git clone https://github.com/yourusername/rustbase.git cd rustbase`

2. Build the project:

   bash

   CollapseWrapCopy

   `cargo build --release`

3. Run the executable:

   bash

   CollapseWrapCopy

   `cargo run --release`

## Usage

Rustbase provides a two-level CLI:

1. **Database Level**: Manage stores within the database.
2. **Store Level**: Manage records within a specific store.

### Database Commands

Run the program to enter the database command mode:

text

CollapseWrapCopy

```
> help Available commands:  help                - Show this help message  exit                - Exit the program  store <name>        - Move to or create a store  delete_store <name> - Delete a store  list_stores         - List all stores  save                - Save the database to a JSON file  export_sql          - Export the database to a SQL script
```

**Example**:

text

CollapseWrapCopy

```
> store users Store 'users' created. store>
```

### Store Commands

Once inside a store (e.g., store users), use these commands:

text

CollapseWrapCopy

```
store> help Available store commands:  help                                     - Show this help message  exit                                     - Exit this level  new_record <attribute:value,...>         - Create a new record (e.g., name:John, age:30)  delete_record <record_id>                - Delete a record by its ID  list_records                             - List all records  get_record <record_id>                   - Get a record by its ID  filter <attribute> <value>               - Filter records by attribute and value  filters <attributes> <values>            - Filter records by multiple attributes and values  operators                                - Display available query operators  query <attribute> <operator> <value>     - Query records (e.g., age gt 25)  sort <attribute> <asc/desc>              - Sort records by attribute (e.g., age asc)
```

**Example**:

text

CollapseWrapCopy

```
store> new_record name:John Doe, age:30 Record added with ID: 0 store> new_record name:Alice, age:25 Record added with ID: 1 store> list_records Records in store 'users': ID: 0 - {name: "John Doe", age: "30"} ID: 1 - {name: "Alice", age: "25"} store> query age gt 25 Query results: ID: 0 - {name: "John Doe", age: "30"} store> sort age desc Sorted records (age desc): ID: 0 - {name: "John Doe", age: "30"} ID: 1 - {name: "Alice", age: "25"} store> exit > save Database saved to 'my_database.json'. > export_sql Database exported to 'my_database.sql'.
```

## Project Structure

text

CollapseWrapCopy

```
rustbase/ ├── src/ │   ├── main.rs          # Entry point with CLI loop │   ├── memory/ │   │   ├── database.rs  # Database struct and methods │   │   ├── structure.rs # Store struct and methods │   └── lib.rs           # Module definitions ├── Cargo.toml           # Project dependencies and metadata └── README.md            # This file
```

## Dependencies

- serde_json: For JSON serialization and deserialization.
- std: Standard library for I/O and collections.

Add these to your Cargo.toml:

toml

CollapseWrapCopy

```
[dependencies] serde_json = "1.0" serde = { version = "1.0", features = ["derive"] }
```

## Building and Running Tests

To build:

bash

CollapseWrapCopy

```
cargo build
```

To run tests (if implemented):

bash

CollapseWrapCopy

```
cargo test
```

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch (git checkout -b feature/your-feature).
3. Commit your changes (git commit -m "Add your feature").
4. Push to the branch (git push origin feature/your-feature).
5. Open a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with Rust for performance and safety.
- Inspired by simple key-value and in-memory database systems.

------

### Customization Notes

- **Project Name**: I used "Rustbase" as a placeholder. Replace it with your actual project name.
- **Repository Link**: Update https://github.com/yourusername/rustbase.git with your actual repository URL.
- **Additional Features**: If your project has more features (e.g., networking, additional persistence options), add them to the "Features" section.
- **File Structure**: Adjust the "Project Structure" section if your directory layout differs.
- **License**: I assumed MIT, but change it to your preferred license.