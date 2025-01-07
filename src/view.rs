use rusqlite::{Connection, Result};
use prettytable::{Table, row};
use std::env;

#[derive(Debug)] 
struct Service {
    id: Option<i32>,
    service: String,
    nonce: Vec<u8>,
    encrypted_password: Vec<u8>,
    notes: Option<String>
}

fn main() -> Result<()> {
    let curr_dir: std::path::PathBuf = env::current_dir().expect("???DIRECTORY NOT FOUND???");
    let path_database: std::path::PathBuf = curr_dir.join("database.db");
    let conn: Connection = Connection::open(path_database)?;
//maybe use .env 
//create table once
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER,
            service TEXT NOT NULL,
            nonce BLOB NOT NULL,
            encrypted_password BLOB NOT NULL,
            notes TEXT

        )",
        (), // empty list of parameters.
    )?;

//display data
    let mut stmt: rusqlite::Statement<'_> = conn.prepare("SELECT id, service, nonce, encrypted_password, notes FROM passwords")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Service {
            id: row.get(0)?,
            service: row.get(1)?,
            nonce: row.get(2)?,
            encrypted_password: row.get(3)?,
            notes: row.get(4)?
        })
    })?;

    let mut table: Table = Table::new();
    table.add_row(row!["id","service","nonce","encrypted_password","notes"]);

    for person in person_iter {
        let uwperson: Service = person.unwrap();

        let pid: i32 = uwperson.id.unwrap_or(-1);
        let pservice: String = uwperson.service;
        let pnonce: String = hex_to_string(uwperson.nonce).unwrap_or("MISSING".to_string());
        let pencrypted_password: String = hex_to_string(uwperson.encrypted_password).unwrap_or("MISSING".to_string());
        let pnotes: String = uwperson.notes.unwrap_or("None".to_string());

        table.add_row(row![pid,pservice,pnonce,pencrypted_password,pnotes]);
    }

    table.printstd();
    
    Ok(())
}

fn hex_to_string(binary: Vec<u8>) -> Option<String> {

    let mut hex: String = "x".to_string();
    for byte in binary{

        let add_hex: String = format!("{:x}", byte);
        hex = hex + &add_hex;
    
    }
    hex += "\n";
    
    Some(hex)
}

