#![deny(clippy::all)]
use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};
//to handle result of the operations that may fail
use std::result::Result; //from standard lib which is commonly used for error handling.


//responsible for creating DB tables and enable foreign key constraints and everything 
//will take DB url as input 
async fn create_schema(db_url:&str) -> Result<SqliteQueryResult,sqlx::Error>{
//with the help of pool we will connect to DB
let pool= SqlitePool::connect(&db_url).await?;
let qry=
"PRAGMA foreign_keys = ON ;
CREATE TABLE IF NOT EXISTS settings
(
    settings_id     INTEGER PRIMARY KEY NOT NULL,
    description     TEXT                NOT NULL,
    created_on      DATETIME DEFAULT    (datetime('now','localtime')),
    updated_on      DATETIME DEFAULT    (datetime('now','localtime')),
    done            BOOLEAN             NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS project (
            project_id INTEGER PRIMARY KEY AUTOINCREMENT,
            project_name TEXT NOT NULL,
            created_on DATETIME DEFAULT (datetime('now', 'localtime')),
            updated_on DATETIME DEFAULT (datetime('now', 'localtime')),
            done BOOLEAN NOT NULL DEFAULT 0,
            image_directory TEXT NOT NULL,
            output_directory TEXT NOT NULL,
            status TEXT NOT NULL,
            settings_id INTEGER NOT NULL,
            FOREIGN KEY (settings_id) REFERENCES settings (settings_id) ON DELETE SET NULL
);";
let result = sqlx::query(&qry).execute(&pool).await;
pool.close().await;
return result
}

//or as we use async_std we can mark the main fync
//#[async_std::main]
#[tokio::main]
async fn main() {
    let db_url= String::from("sqlite://sqlite.db");
    //if file of db not exist we will create it
    if !Sqlite::database_exists(&db_url).await.unwrap_or(false){
        Sqlite::create_database(&db_url).await.unwrap();
        //if exists 
        match create_schema(&db_url).await{
            Ok(_) => println!("Database created successfully "),
            Err(e) => println!("{}",e)
        }
    }
    let instances = SqlitePool::connect(&db_url).await.unwrap();

    //to inert smth 

    //insert into table settings that has feild called description
    let qry = "INSERT INTO settings (description) VALUES($1)";
    //to run this query 
    //For example, if your query was INSERT INTO table (column) VALUES (?), the ? would be replaced with "testing".
    //execute() : This parameter specifies where the SQL query should be executed.
    let result  = sqlx::query(&qry).bind("testing").execute(&instances).await;

    //finally will close off the instances
    //whenever we created an instance(connection) we have to close them 
    instances.close().await;
    //print out the result 
    println!("{:?}",result);
  }
