use rusqlite::{params, Connection, Error};

use super::models::model_manufacturer::RecordManufacturer;
use crate::backend::database::table::Table;

pub struct TableManufacturer {}

impl Table<RecordManufacturer> for TableManufacturer {
    fn create_table(db: &Connection) -> Result<(), Error> {
        match db.execute(
            "SELECT name FROM sqlite_master WHERE name='manufacturer'",
            (),
        ) {
            Ok(size) if size == 1 => return Ok(()), // Table already exists so no need to add records
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        if let Err(e) = db.execute(
            "CREATE TABLE IF NOT EXISTS manufacturer (
                id INTEGER PRIMARY KEY,
                name text
            );",
            (),
        ) {
            return Err(e); // Failed to create table
        }

        let manufacturers = vec![
            "Other",
            "Dell",
            "Fujifilm",
            "HP",
            "IBM",
            "Imation",
            "Maxell",
            "Overland Tandberg",
            "Quantum",
            "SONY",
            "Spectra",
            "TDK",
        ];

        for m_name in manufacturers.iter() {
            if let Err(e) = TableManufacturer::insert_record(
                db,
                &RecordManufacturer {
                    id: 0,
                    name: m_name.to_string(),
                },
            ) {
                return Err(e);
            }
        }

        return Ok(());
    }

    fn update_table(_current_version: isize, _latest_version: isize) -> Result<(), Error> {
        Ok(())
    }

    fn insert_record(db: &Connection, record: &RecordManufacturer) -> Result<usize, Error> {
        db.execute(
            "INSERT INTO manufacturer (name) VALUES (?1)",
            params![record.name],
        )
    }

    fn update_record(db: &Connection, record: &RecordManufacturer) -> Result<usize, Error> {
        db.execute(
            "UPDATE manufacturer SET name = ?1
                WHERE id = ?2",
            params![record.name, record.id],
        )
    }
}

impl TableManufacturer {
    pub fn get_all(db: &Connection) -> Result<Vec<RecordManufacturer>, rusqlite::Error> {
        db.prepare(
            "SELECT id, name FROM manufacturer ORDER BY
                    CASE id
                        WHEN 1 THEN 2
                    END,
                    name", // Order by name then "Other" [id=1] to be last
        )?
        .query_map([], |row| {
            Ok(RecordManufacturer {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })?
        .collect::<Result<Vec<RecordManufacturer>, rusqlite::Error>>()
    }
}
