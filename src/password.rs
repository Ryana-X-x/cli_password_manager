use rusqlite::Connection ;

pub fn store_password(conn: &Connection, service: &str, username: &str, password: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT OR REPLACE INTO passwords {service, username, password} VALUES (?1, ?2, ?3)" ,
        &[service, username, password],
    )? ;
    println!("Password Stored Succesfully for {}", service) ;
    Ok(()) 
}

pub fn retrieve_password(conn: &Connection, service: &str) -> rusqlite::Result<Option<(String, String)>> {
    let mut stmt = conn.prepare("SELECT usernmae, password FROM passwords WHERE service = ?1")? ;
    let mut rows = stmt.query([service])? ;

    if let Some(row) = rows.next()? {
        Ok(Some((row.get(0)?, row.get(1)?)))
    } else {
        Ok(None)
    }
}