use libsql::Builder;
use std::env;
use std::fs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let url = env::var("TURSO_DATABASE_URL").expect("TURSO_DATABASE_URL must be set");
    let token = env::var("TURSO_AUTH_TOKEN").expect("TURSO_AUTH_TOKEN must be set");

    println!("Connecting to Turso DB...");
    let db = Builder::new_remote(url, token).build().await?;
    let conn = db.connect()?;

    // 1. Initialize Schema
    println!("Applying schema...");
    let schema = fs::read_to_string("schema.sql")?;
    // split by ';' and execute each statement
    for statement in schema.split(';') {
        let stmt = statement.trim();
        if !stmt.is_empty() {
            conn.execute(stmt, ()).await?;
        }
    }

    // 2. Clear existing data
    println!("Clearing old data...");
    conn.execute("DELETE FROM options", ()).await?;
    conn.execute("DELETE FROM questions", ()).await?;
    conn.execute("DELETE FROM events", ()).await?;
    conn.execute("DELETE FROM categories", ()).await?;

    // Reset sequence? SQLite doesn't always need this unless we want ID 1 again.
    // conn.execute("DELETE FROM sqlite_sequence WHERE name IN ('categories', 'events', 'questions', 'options')", ()).await.ok();

    // 3. Seed Categories
    println!("Seeding 24 Categories...");

    let categories = vec![
        // Eras
        ("Makkah Period", "মক্কী জীবন", "era", "🕋", 1),
        ("Madinah Period", "মাদানী জীবন", "era", "🕌", 2),
        
        // Surah Groups
        ("Makki Surahs", "মক্কী সূরা", "surah_group", "📖", 3),
        ("Madani Surahs", "মাদানী সূরা", "surah_group", "📗", 4),

        // Prophets (10)
        ("Adam (AS)", "আদম (আঃ)", "prophet", "🌿", 5),
        ("Nuh (AS)", "নূহ (আঃ)", "prophet", "🚢", 6),
        ("Ibrahim (AS)", "ইবরাহীম (আঃ)", "prophet", "🔥", 7),
        ("Ismail (AS)", "ইসমাঈল (আঃ)", "prophet", "🏜️", 8),
        ("Yusuf (AS)", "ইউসুফ (আঃ)", "prophet", "⭐", 9),
        ("Musa (AS)", "মূসা (আঃ)", "prophet", "🌊", 10),
        ("Dawud (AS)", "দাউদ (আঃ)", "prophet", "🎵", 11),
        ("Sulaiman (AS)", "সুলাইমান (আঃ)", "prophet", "👑", 12),
        ("Isa (AS)", "ঈসা (আঃ)", "prophet", "☁️", 13),
        ("Muhammad (SAW)", "মুহাম্মদ (সাঃ)", "prophet", "✨", 14),

        // Topics (10)
        ("Major Battles", "প্রধান যুদ্ধসমূহ", "topic", "⚔️", 15),
        ("Lives of Sahabas", "সাহাবীদের জীবনী", "topic", "🌟", 16),
        ("Mothers of Believers", "উম্মাহাতুল মুমিনীন", "topic", "💎", 17),
        ("Hijrah", "হিজরতের ঘটনা", "topic", "🐪", 18),
        ("Pillars of Islam", "ইসলামের স্তম্ভসমূহ", "topic", "🏛️", 19),
        ("Branches of Iman", "ঈমানের শাখাসমূহ", "topic", "🌳", 20),
        ("Rashidun Caliphs", "খুলাফায়ে রাশেদীন", "topic", "🏆", 21),
        ("Stories from Quran", "কুরআনের কাহিনী", "topic", "📚", 22),
        ("Dua & Dhikr", "দোয়া ও যিকর", "topic", "🤲", 23),
        ("Hereafter", "আখিরাত ও হাশর", "topic", "⏳", 24),
    ];

    for (name, name_bn, cat_type, icon, order) in categories {
        conn.execute(
            "INSERT INTO categories (name, name_bn, category_type, icon, sort_order) VALUES (?1, ?2, ?3, ?4, ?5)",
            libsql::params![name, name_bn, cat_type, icon, order],
        )
        .await?;
    }

    // 4. Seed Sample Events (Meccan Period)
    println!("Seeding Events...");
    
    // Get Category IDs (simple fetch since we just inserted them in order, but let's query to be safe or just assume IDs 1 & 2 for eras)
    // Actually, AUTOINCREMENT IDs typically start at 1.
    let makki_id = 1; 
    let madani_id = 2;

    let events = vec![
        (makki_id, "First Revelation", "প্রথম ওহী", "Hira Cave...", "হেরা গুহায়... (বিস্তারিত)", "610 CE"),
        (makki_id, "Public Preaching", "প্রকাশ্যে দাওয়াত", "Safa Hill...", "সাফা পাহাড়ে...", "613 CE"),
        (madani_id, "Hijrah", "হিজরত", "Migration to Madinah...", "মদিনায় হিজরত...", "622 CE"),
        (madani_id, "Battle of Badr", "বদরের যুদ্ধ", "First battle...", "প্রথম যুদ্ধ...", "624 CE"),
    ];

    for (cat_id, title, title_bn, desc, desc_bn, date) in events {
        conn.execute(
            "INSERT INTO events (category_id, title, title_bn, description, description_bn, event_date) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            libsql::params![cat_id, title, title_bn, desc, desc_bn, date],
        )
        .await?;
    }

    // 5. Questions (Sample)
    println!("Seeding Questions...");
    // Need an event ID. Let's assume ID 1 (First Revelation).
    let event_id = 1;
    let cat_id = 1; // Makki

    // Insert a question
    conn.execute(
        "INSERT INTO questions (event_id, category_id, question_text, question_text_bn, explanation, explanation_bn, difficulty_level) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
        libsql::params![
            event_id, 
            cat_id, 
            "Where was the first revelation?", 
            "প্রথম ওহী কোথায় নাজিল হয়েছিল?",
            "It was in Hira Cave.",
            "এটি হেরা গুহায় নাজিল হয়েছিল।",
            "Easy"
        ],
    ).await?;

    // Get that question ID (should be 1)
    let q_id = 1;

    let options = vec![
        ("Cave Hira", "হেরা গুহায়", true),
        ("Cave Thawr", "সাওর গুহায়", false),
        ("Kaaba", "কাবা ঘরে", false),
        ("Uhud", "উহুদ পাহাড়ে", false),
    ];

    for (txt, txt_bn, is_correct) in options {
        conn.execute(
            "INSERT INTO options (question_id, option_text, option_text_bn, is_correct) VALUES (?1, ?2, ?3, ?4)",
            libsql::params![q_id, txt, txt_bn, is_correct],
        ).await?;
    }

    println!("Database seeded successfully!");
    Ok(())
}
