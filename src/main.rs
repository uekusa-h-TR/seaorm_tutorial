// main.rs

use futures::executor::block_on;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbErr, Statement};

// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "mysql://root:password@localhost:3306";
const DB_NAME: &str = "bakeries_db";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    let db = &match db.get_database_backend(){
        DbBackend::MySql => {
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("{}/{}", DATABASE_URL, DB_NAME),
            ))
            .await?;
            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Postgres => {
            db.execute(Statement::from_string(
                db.get_database_backend(), 
                format!("DROP DATABASE IF EXISTS {}", DB_NAME)
                )).await?;
            db.execute(Statement::from_string(
                db.get_database_backend(),
                format!("CREATE DATABASE \"{}\";", DB_NAME),
            )).await?;

            let url = format!("{}/{}", DATABASE_URL, DB_NAME);
            Database::connect(&url).await?
        }
        DbBackend::Sqlite => db,
    };
    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}