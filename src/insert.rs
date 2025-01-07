use rusqlite::{Connection, Result};

struct Service {
    id: Option<i32>,
    service: String,
    nonce: Vec<u8>,
    encrypted_password: Vec<u8>,
    notes: Option<String>
}
//WHEN INSERT ID NEED NOT BE INPUTTED AND THE NEXT ONE IS AUTOMATICALLY ASSIGNED
fn main() -> Result<()> { //implicity result fills in the second paramter from rusqlit error type
    let conn = Connection::open("/Users/ayush/Desktop/Rust-ML/Bastion/database.db")?;
    //MODIFY THE PATH LATER FOR USER
    let me = Service {
        id: None,
        service: "Netflix".to_string(),
        nonce: vec![32,38,49],
        encrypted_password: vec![32,34,76,10,20,34],
        notes: None
    };
    conn.execute(
        "INSERT INTO passwords (id, service, nonce, encrypted_password, notes) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&me.id, &me.service, &me.nonce, &me.encrypted_password, &me.notes),
    )?;

    Ok(())
}