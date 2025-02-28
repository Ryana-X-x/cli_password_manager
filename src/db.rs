use rusqlite::{Connection, Result} ;

pub fn intialize_db() -> Result<Connection> {
    let conn = Connection::open("password.db")? ;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS master_key (
            id INTEGER PRIMARY KEY, 
        )",
        [],
    )? ;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY, 
            service TEXT NOT NULL UNIQUE, 
            username TEXT NOT FULL, 
            password TEXT NOT FULL
        )",
        [],
    )? ;

    Ok(conn)
}