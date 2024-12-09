// main.rs

use ::entity::{user, user::Entity as User};
use sea_orm::*;
use uuid::Uuid;
use chrono::Utc;
// use tokio;
// Change this according to your database implementation,
// or supply it as an environment variable.
// the whole database URL string follows the following format:
// "protocol://username:password@host:port/database"
// We put the database name (that last bit) in a separate variable simply for convenience.
const DATABASE_URL: &str = "postgres://root:password@localhost:5435/postgres";
// const DB_NAME: &str = "postgres";

async fn run() -> Result<(), DbErr> {
    let db = Database::connect(DATABASE_URL).await?;

    // create user
    let user = user::ActiveModel{
        name: Set("John Doe".to_string()),
        email: Set("john_doe@example.com".to_string()),
        password: Set("password".to_string()),
        pid: Set(Uuid::new_v4()),
        created_at: Set(Utc::now().into()),
        updated_at: Set(Utc::now().into()),
        ..Default::default()
    };
    let res: InsertResult<user::ActiveModel> = User::insert(user).exec(&db).await?;
    println!("User created: {:?}", res);
    Ok(())
}

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Error: {}", e);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_run() {
        run().await.unwrap();
    }
}