use libsql::{Builder, Connection, Database};
use std::env;
use std::sync::Arc;

pub type DbConnection = Connection;
pub type DbPool = Arc<Database>; // We'll pass the Database object around to create connections

pub async fn init_db() -> Result<DbPool, Box<dyn std::error::Error>> {
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    let db = if let Ok(db_file) = env::var("DB_FILE") {
        // Try embedded replica (needs writable filesystem)
        println!("Trying embedded replica with file: {}", db_file);
        match Builder::new_remote_replica(db_file, url.clone(), token.clone())
            .sync_interval(std::time::Duration::from_secs(300))
            .sync_protocol(libsql::SyncProtocol::V2)
            .build()
            .await
        {
            Ok(db) => {
                println!("Embedded replica initialized successfully");
                db
            }
            Err(e) => {
                eprintln!("Embedded replica failed ({}), falling back to remote-only", e);
                Builder::new_remote(url, token)
                    .build()
                    .await?
            }
        }
    } else {
        println!("Initializing remote-only connection");
        Builder::new_remote(url, token)
            .build()
            .await?
    };

    Ok(Arc::new(db))
}

