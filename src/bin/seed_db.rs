use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new().connect(&database_url).await?;

    // Clear existing data to avoid duplicates/mixing languages
    // Order matters due to foreign keys
    sqlx::query("DELETE FROM options").execute(&pool).await?;
    sqlx::query("DELETE FROM questions").execute(&pool).await?;
    sqlx::query("DELETE FROM events").execute(&pool).await?;
    sqlx::query("DELETE FROM eras").execute(&pool).await?;

    // Seed Eras (Bengali)
    sqlx::query(
        "INSERT INTO eras (name, description, start_date, end_date) VALUES 
        ('মাক্কী জীবন', 'রাসূল (সাঃ) এর মক্কা জীবন', '610-01-01', '622-01-01'),
        ('মাদানী জীবন', 'রাসূল (সাঃ) এর মাদানী জীবন', '622-01-01', '632-01-01')
    ",
    )
    .execute(&pool)
    .await?;

    let meccan_id = sqlx::query_scalar::<_, i64>("SELECT id FROM eras WHERE name = 'মাক্কী জীবন'")
        .fetch_one(&pool)
        .await?;
    let medinan_id = sqlx::query_scalar::<_, i64>("SELECT id FROM eras WHERE name = 'মাদানী জীবন'")
        .fetch_one(&pool)
        .await?;

    // Seed Events (Bengali)
    sqlx::query(
        "INSERT INTO events (era_id, title, description, event_date) VALUES 
        (?, 'প্রথম ওহী', 'হেরা গুহায় প্রথম ওহী নাজিল হয় (ইকরা বিসমি রাব্বিকা...)', '610-08-10'),
        (?, 'হিজরত', 'মক্কা থেকে মদীনায় হিজরত', '622-09-24')
    ",
    )
    .bind(meccan_id)
    .bind(medinan_id)
    .execute(&pool)
    .await?;

    let rev_id = sqlx::query_scalar::<_, i64>("SELECT id FROM events WHERE title = 'প্রথম ওহী'")
        .fetch_one(&pool)
        .await?;

    // Seed Questions (Bengali)
    sqlx::query(
        "INSERT INTO questions (event_id, question_text, explanation) VALUES 
        (?, 'প্রথম ওহী কোথায় নাজিল হয়েছিল?', 'এটি জাবাল আল-নূরের হেরা গুহায় নাজিল হয়েছিল।')
    ",
    )
    .bind(rev_id)
    .execute(&pool)
    .await?;

    let q_id = sqlx::query_scalar::<_, i64>(
        "SELECT id FROM questions WHERE question_text LIKE 'প্রথম ওহী%'",
    )
    .fetch_one(&pool)
    .await?;

    // Seed Options (Bengali)
    sqlx::query(
        "INSERT INTO options (question_id, option_text, is_correct) VALUES 
        (?, 'হেরা গুহায়', 1),
        (?, 'সাওর গুহায়', 0),
        (?, 'কাবা ঘরে', 0),
        (?, 'উহুদ পাহাড়ে', 0)
    ",
    )
    .bind(q_id)
    .bind(q_id)
    .bind(q_id)
    .bind(q_id)
    .execute(&pool)
    .await?;

    println!("Database seeded successfully with Bengali data!");
    Ok(())
}
