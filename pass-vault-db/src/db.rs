use std::io;
use std::io::Write;
extern crate rusqlite;
use rusqlite::{Connection, Error};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct ServiceInfo {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: String, username: String, password: String) -> Self {
        ServiceInfo {
            id: None,
            service,
            username,
            password,
        }
    }
}

pub fn prompt(prompt: &str) -> String {
    println!("{}", prompt);
    //This line ensures that anything written to stdout is immediately displayed to the user.
    io::stdout().flush().unwrap();
    let mut input = String::new(); //initializes a mutable String to store the user's input.
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

pub fn init_database() -> Result<Connection, Error> {
    //This line attempts to open a connection to a SQLite database named "passwords". If the database does not exist, SQLite will create it.
    let conn = Connection::open("passwords")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT NOT NULL,
            username TEXT NOT NULL,
            password TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn write_password_to_db(
    conn: &Connection,
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO passwords (service,username,password) VALUES (?,?,?)",
        &[&service, &username, &password],
    )?;
    Ok(())
}

pub fn read_password_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT service, username, password FROM passwords")?;

    let entries = stmt //executes the prepared SQL statement.
        .query_map([], |row| {
            //The empty array [] indicates that there are no parameters to bind to the query. The function uses query_map to iterate over the rows returned by the query. For each row, it constructs a ServiceInfo object
            Ok(ServiceInfo::new(row.get(0)?, row.get(1)?, row.get(2)?)) //Retrieves the value from the first column (service) and handles potential errors with ?.
        })?
        .collect::<Result<Vec<_>, _>>()?; //This part of the function collects all the ServiceInfo objects created from the database rows into a vector.
    Ok(entries) //function returns the vector of ServiceInfo objects wrapped in an Ok.
}

pub fn search_service_by_name(
    conn: &Connection,
    search: &str,
) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt =
        conn.prepare("SELECT id, service, username, password FROM passwords WHERE service=?")?; //search name

    let result = stmt.query_row(&[search], |row| {
        Ok(ServiceInfo {
            id: Some(row.get(0)?),
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}
