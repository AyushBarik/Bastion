use rusqlite::{Connection, Result};
use prettytable::{Table, row};
use std::{env, io};

mod common;

#[derive(Debug)] 
struct Service {
    id: Option<i32>,
    service: String,
    nonce: Vec<u8>,
    encrypted_password: Vec<u8>,
    notes: Option<String>
}

fn main() -> Result<()> {
    let conn = Connection::open("/Users/ayush/Desktop/Rust-ML/Bastion/database.db").unwrap();
    println!("path {:?}", conn);
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

        //asks to set the master password 

    )?;

    //Ask for and check password
    println!("Enter Master Password:");
    let mut mpassword = Default::default();
    io::stdin().read_line(&mut mpassword).expect("WRONG PASSWORD");

    //check aaginst database
    let result = common::hashpass(mpassword).expect("FAILED TO HASH");
    if !result.1 {
        println!("WRONG PASSWORD");
        return Ok(())
    }
    //if correct use mpassword to derive key

    //wait for abort view insert delete.... async

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
        let pnonce: String = common::bin_to_hexstring(uwperson.nonce).unwrap_or("MISSING".to_string());
        let pencrypted_password: String = common::bin_to_hexstring(uwperson.encrypted_password).unwrap_or("MISSING".to_string());
        let pnotes: String = uwperson.notes.unwrap_or("None".to_string());

        table.add_row(row![pid,pservice,pnonce,pencrypted_password,pnotes]);
    }

    table.printstd();
    
    Ok(())
}


