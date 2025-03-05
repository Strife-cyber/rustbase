use std::io;
use super::database::Database;

impl Database {
    pub fn store() -> io::Result<()> {
        Ok(())
    }

    pub fn load() -> io::Result<Database> {

    }
}
