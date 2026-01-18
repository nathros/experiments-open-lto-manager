use rusqlite::{Connection, Error};

pub trait Table<T> {
    fn create_table(db: &Connection) -> Result<(), Error>;
    fn update_table(current_version: isize, latest_version: isize) -> Result<(), Error>;

    fn insert_record(db: &Connection, record: &T) -> Result<usize, Error>;
    fn update_record(db: &Connection, record: &T) -> Result<usize, Error>;
}
