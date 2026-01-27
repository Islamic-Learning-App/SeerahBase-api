use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, ConnectOptions};
use std::str::FromStr;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    println!("Database URL: {}", database_url);

    // Create database file if it doesn't exist (sqlx requires this for SQLite sometimes, or we can use create_if_missing)
    let options = SqliteConnectOptions::from_str(&database_url)?
        .create_if_missing(true);

    let pool = SqlitePoolOptions::new()
        .connect_with(options)
        .await?;

    let schema = fs::read_to_string("schema.sql")?;
    
    // Split by semicolon to execute multiple statements if needed, 
    // but sqlx::query might handle multiple statements for sqlite depending on configuration.
    // Ideally, we execute the whole script.
    sqlx::query(&schema).execute(&pool).await?;

    println!("Database initialized successfully!");
    Ok(())
}
