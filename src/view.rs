use rusqlite::{Connection, Result};

#[derive(Debug)]
struct Service {
    id: Option<i32>,
    service: String,
    nonce: Vec<u8>,
    encrypted_password: Vec<u8>,
    notes: Option<String>
}

fn main() -> Result<()> {
    let conn = Connection::open("/Users/ayush/Desktop/Rust-ML/Bastion/database.db")?;
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
//insert data
    // let me = Service {
    //     id: None,
    //     service: "Netflix".to_string(),
    //     nonce: vec![32,38,49],
    //     encrypted_password: vec![32,34,76,10,20,34],
    //     notes: None
    // };
    // conn.execute(
    //     "INSERT INTO passwords (id, service, nonce, encrypted_password, notes) VALUES (?1, ?2, ?3, ?4, ?5)",
    //     (&me.id, &me.service, &me.nonce, &me.encrypted_password, &me.notes),
    // )?;

//display data
    let mut stmt = conn.prepare("SELECT id, service, nonce, encrypted_password, notes FROM passwords")?;
    let person_iter = stmt.query_map([], |row| {
        Ok(Service {
            id: row.get(0)?,
            service: row.get(1)?,
            nonce: row.get(2)?,
            encrypted_password: row.get(3)?,
            notes: row.get(4)?
        })
    })?;

    for person in person_iter {
        println!("Found person {:?}", person.unwrap());
    }

    Ok(())
}
