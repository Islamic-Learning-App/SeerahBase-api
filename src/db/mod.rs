use libsql::{Builder, Database};
use std::env;
use std::sync::Arc;

pub type DbPool = Arc<Database>;

/// Manually sync the embedded replica after a write operation.
/// Safe to call on remote-only connections (it's a no-op).
pub async fn sync_db(db: &Database) {
    if let Err(e) = db.sync().await {
        eprintln!("Sync warning: {}", e);
    }
}

async fn try_embedded_replica(
    db_file: &str,
    url: &str,
    token: &str,
) -> Result<Database, Box<dyn std::error::Error>> {
    // Try V2 protocol first (faster), fall back to stable V1
    let db = match Builder::new_remote_replica(db_file, url.to_string(), token.to_string())
        .sync_protocol(libsql::SyncProtocol::V2)
        .build()
        .await
    {
        Ok(db) => match db.sync().await {
            Ok(_) => {
                println!("Embedded replica ready at: {} (V2, manual sync)", db_file);
                return Ok(db);
            }
            Err(e) => {
                eprintln!("V2 sync failed ({}), trying stable protocol...", e);
            }
        },
        Err(e) => {
            eprintln!("V2 build failed ({}), trying stable protocol...", e);
        }
    };

    // Fallback: stable protocol (V1/default)
    let _ = std::fs::remove_file(db_file); // clean up partial V2 files
    let db = Builder::new_remote_replica(db_file, url.to_string(), token.to_string())
        .build()
        .await?;
    db.sync().await?;
    println!("Embedded replica ready at: {} (stable, manual sync)", db_file);
    Ok(db)
}

pub async fn init_db() -> Result<DbPool, Box<dyn std::error::Error>> {
    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    // 1) Try user-specified DB_FILE
    let db_file = env::var("DB_FILE").unwrap_or_default();
    if !db_file.is_empty() {
        println!("Trying embedded replica: {}", db_file);
        if let Ok(db) = try_embedded_replica(&db_file, &url, &token).await {
            return Ok(Arc::new(db));
        }
        eprintln!("Failed with DB_FILE={}, trying /tmp fallback...", db_file);
    }

    // 2) Try /tmp (writable on most serverless platforms)
    let tmp_path = "/tmp/seerah.db";
    println!("Trying embedded replica: {}", tmp_path);
    if let Ok(db) = try_embedded_replica(tmp_path, &url, &token).await {
        return Ok(Arc::new(db));
    }

    // 3) Last resort: remote-only (every query hits network)
    eprintln!("All replica paths failed, using remote-only");
    let db = Builder::new_remote(url, token).build().await?;
    Ok(Arc::new(db))
}
