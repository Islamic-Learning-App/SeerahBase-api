use libsql::{Builder, Database};
use std::env;
use std::sync::Arc;

pub type DbPool = Arc<Database>;

async fn try_embedded_replica(
    db_file: &str,
    url: &str,
    token: &str,
) -> Result<Database, Box<dyn std::error::Error>> {
    let db = Builder::new_remote_replica(db_file, url.to_string(), token.to_string())
        .sync_interval(std::time::Duration::from_secs(300))
        .sync_protocol(libsql::SyncProtocol::V2)
        .build()
        .await?;
    println!("Embedded replica ready at: {}", db_file);
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
    eprintln!("All replica paths failed, using remote-only (slower reads)");
    let db = Builder::new_remote(url, token).build().await?;
    Ok(Arc::new(db))
}

