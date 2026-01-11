#[cfg(feature = "server")]
use std::io::ErrorKind;

use dioxus::{
    fullstack::serde::{Deserialize, Serialize},
    prelude::*,
};

#[cfg(feature = "server")]
use crate::backend::database::table_manufacturer::TableManufacturer;
#[cfg(feature = "server")]
use crate::backend::database::{table::Table, table_tape_type::TableTapeType};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct Tape {
    pub id: i32,
    pub barcode: String,
    pub worm: bool,
}

//static DB_VERSION_INIT: isize = 0;
static DB_VERSION_LATEST: isize = 0;

#[cfg(feature = "server")]
thread_local! {
    pub static DB: std::sync::LazyLock<rusqlite::Connection> = std::sync::LazyLock::new(|| {
        match std::fs::create_dir("database") {
            Ok(_) => {},
            Err(err) => {
                if err.kind() != ErrorKind::AlreadyExists {
                    // log::error!("Failed to create dir {e}"); FIXME
                }
            },
        }

        let conn = rusqlite::Connection::open("database/database.db").expect("Failed to open database");

        let current_database_version: isize = 0;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS tape (
                id INTEGER PRIMARY KEY,
                barcode VARCHAR(8),
                worm BOOLEAN
            );"
        )
        .unwrap();

        // FIXME handle errors
        if let Ok(_) = TableManufacturer::create_table(&conn) {
            let _ = TableManufacturer::update_table(current_database_version, DB_VERSION_LATEST);
        }
        if let Ok(_) = TableTapeType::create_table(&conn) {
            let _ = TableTapeType::update_table(current_database_version, DB_VERSION_LATEST);
        }

        //save_tape(Tape { id: 0, barcode: "first".to_string(), worm: true }).await;

        conn
    });
}

#[get("/api/tapes")]
pub async fn list_tapes() -> Result<Vec<(usize, Tape)>> {
    DB.with(|db| {
        Ok(db
            .prepare("SELECT id, barcode, worm FROM tape ORDER BY id LIMIT 10")?
            .query_map([], |row| {
                Ok((
                    row.get(0)?,
                    Tape {
                        id: row.get(0)?,
                        barcode: row.get(1)?,
                        worm: row.get(2)?,
                    },
                ))
            })?
            .collect::<Result<Vec<(usize, Tape)>, rusqlite::Error>>()?)
    })
}

#[post("/api/tapes")]
pub async fn save_tape(tape: Tape) -> Result<()> {
    DB.with(|db| {
        use rusqlite::params;
        db.execute(
            "INSERT INTO tape (barcode, worm) VALUES (?1, ?2)",
            params![&tape.barcode, &tape.worm],
        )
    })?;
    Ok(())
}
