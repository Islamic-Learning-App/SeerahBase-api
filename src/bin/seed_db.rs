use sqlx::sqlite::SqlitePoolOptions;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePoolOptions::new().connect(&database_url).await?;

    // Clear existing data
    sqlx::query("DELETE FROM options").execute(&pool).await?;
    sqlx::query("DELETE FROM questions").execute(&pool).await?;
    sqlx::query("DELETE FROM events").execute(&pool).await?;
    sqlx::query("DELETE FROM eras").execute(&pool).await?;

    // 1. Seed Eras (Bengali)
    println!("Seeding Eras...");
    sqlx::query(
        "INSERT INTO eras (name, description, start_date, end_date) VALUES 
        ('মাক্কী জীবন', 'নবুয়ত প্রাপ্তি থেকে হিজরত পর্যন্ত ১৩ বছর।', '610-01-01', '622-09-20'),
        ('মাদানী জীবন', 'হিজরতের পর থেকে ওফাত পর্যন্ত ১০ বছর।', '622-09-24', '632-06-08')
    ",
    )
    .execute(&pool)
    .await?;

    let meccan_id = sqlx::query_scalar::<_, i64>("SELECT id FROM eras WHERE name = 'মাক্কী জীবন'").fetch_one(&pool).await?;
    let medinan_id = sqlx::query_scalar::<_, i64>("SELECT id FROM eras WHERE name = 'মাদানী জীবন'").fetch_one(&pool).await?;

    // 2. Seed Events (Bengali) - Meccan Period
    println!("Seeding Events...");
    let meccan_events = vec![
        ("প্রথম ওহী", "হেরা গুহায় ধ্যানরত অবস্থায় রাসূলুল্লাহ (সাঃ) এর নিকট জিবরাঈল (আঃ) সূরা আলাকের প্রথম ৫ আয়াত নিয়ে আসেন। এটি ছিল নবুওয়াতের সূচনা। তিনি ভীত হয়ে খাদিজা (রাঃ) এর কাছে ফিরে আসেন এবং বলেন, 'আমাকে আবৃত কর'। খাদিজা (রাঃ) তাকে ওয়ারাকা বিন নওফেলের কাছে নিয়ে যান। \n\nবিস্তারিত পড়ুন [উইকিপিডিয়া](https://bn.wikipedia.org/wiki/%E0%A6%B9%E0%A7%87%E0%A6%B0%E0%A6%BE_%E0%A6%97%E0%A7%81%E0%A6%B9%E0%A6%BE) তে।", "610-08-10", "সহীহ বুখারী"),
        ("প্রকাশ্যে দাওয়াত", "সাফা পাহাড়ে উঠে কুরাইশদের প্রকাশ্যে সতর্ক করা।", "613-01-01", "আর-রাহীকুল মাখতূম"),
        ("হাবশায় হিজরত", "কুরাইশদের নির্যাতনে অতিষ্ঠ হয়ে মুসলমানদের একটি দলের হাবশায় হিজরত।", "615-05-01", "সীরাতে ইবনে হিশাম"),
        ("শিয়াবে আবী তালিব", "বনু হাশিম ও বনু মুত্তালিব ৩ বছর বয়কট অবস্থায় ছিল।", "617-01-01", "আর-রাহীকুল মাখতূম"),
        ("দুঃখের বছর", "চাচা আবু তালিব ও স্ত্রী খাদিজা (রাঃ) এর মৃত্যু।", "619-01-01", "সীরাতে ইবনে হিশাম"),
        ("ইসরা ও মেরাজ", "এক রাতে মক্কা থেকে বাইতুল মুকাদ্দাস এবং সেখান থেকে সপ্তাকাশ ভ্রমণ।", "620-02-27", "সহীহ বুখারী"),
    ];

    for (title, desc, date, src) in meccan_events {
        sqlx::query("INSERT INTO events (era_id, title, description, event_date, source) VALUES (?, ?, ?, ?, ?)")
            .bind(meccan_id)
            .bind(title)
            .bind(desc)
            .bind(date)
            .bind(src)
            .execute(&pool)
            .await?;
    }

    // 2. Seed Events (Bengali) - Medinan Period
    let medinan_events = vec![
        ("হিজরত", "মক্কা থেকে মদিনায় ঐতিহাসিক হিজরত।", "622-09-24", "সহীহ বুখারী"),
        ("বদরের যুদ্ধ", "সত্য ও মিথ্যার প্রথম চূড়ান্ত ফয়সালাকারী যুদ্ধ।", "624-03-17", "সূরা আল-আনফাল"),
        ("উহুদের যুদ্ধ", "মুসলিমদের সাময়িক পরাজয় ও শিক্ষা।", "625-03-23", "সীরাতে ইবনে হিশাম"),
        ("খন্দকের যুদ্ধ", "মদিনার চারপাশ ঘিরে পরিখা খনন করে প্রতিরক্ষা।", "627-03-31", "সূরা আল-আহযাব"),
        ("হুদায়বিয়ার সন্ধি", "প্রকাশ্য বিজয়ের সূচনা।", "628-03-01", "সূরা আল-ফাতহ"),
        ("মক্কা বিজয়", "রক্তপাতহীন ঐতিহাসিক বিজয়।", "630-01-11", "সহীহ বুখারী"),
        ("বিদায় হজ", "আরাফাতের ময়দানে রাসূল (সাঃ) এর চূড়ান্ত ভাষণ।", "632-03-06", "সহীহ মুসলিম"),
    ];

    for (title, desc, date, src) in medinan_events {
        sqlx::query("INSERT INTO events (era_id, title, description, event_date, source) VALUES (?, ?, ?, ?, ?)")
            .bind(medinan_id)
            .bind(title)
            .bind(desc) // fixed
            .bind(date)
            .bind(src)
            .execute(&pool)
            .await?;
    }

    // 3. Questions
    println!("Seeding Questions...");
    
    // Helper to add Q&A
    async fn add_q(pool: &sqlx::SqlitePool, event_title: &str, text: &str, expl: &str, diff: &str, options: Vec<(&str, bool)>) -> Result<(), Box<dyn std::error::Error>> {
        let event_id = sqlx::query_scalar::<_, i64>("SELECT id FROM events WHERE title = ?").bind(event_title).fetch_one(pool).await?;
        
        let q_id = sqlx::query_scalar::<_, i64>("INSERT INTO questions (event_id, question_text, explanation, difficulty_level) VALUES (?, ?, ?, ?) RETURNING id")
            .bind(event_id)
            .bind(text)
            .bind(expl)
            .bind(diff)
            .fetch_one(pool)
            .await?;

        for (opt_text, is_correct) in options {
            sqlx::query("INSERT INTO options (question_id, option_text, is_correct) VALUES (?, ?, ?)")
                .bind(q_id)
                .bind(opt_text)
                .bind(is_correct)
                .execute(pool)
                .await?;
        }
        Ok(())
    }

    add_q(&pool, "প্রথম ওহী", "প্রথম ওহী কোথায় নাজিল হয়েছিল?", "এটি জাবাল আল-নূরের হেরা গুহায় নাজিল হয়েছিল।", "Easy", vec![
        ("হেরা গুহায়", true), ("সাওর গুহায়", false), ("কাবা ঘরে", false), ("উহুদ পাহাড়ে", false)
    ]).await?;

    add_q(&pool, "ইসরা ও মেরাজ", "মেরাজ এর রাতে কত ওয়াক্ত সালাত ফরজ করা হয়?", "প্রাথমিক ভাবে ৫০ ওয়াক্ত, পরে কমিয়ে ৫ ওয়াক্ত করা হয়।", "Medium", vec![
        ("৫০ ওয়াক্ত", false), ("৫ ওয়াক্ত", true), ("১০ ওয়াক্ত", false), ("৩ ওয়াক্ত", false)
    ]).await?;

    add_q(&pool, "হিজরত", "হিজরতের সময় রাসূল (সাঃ) এর সঙ্গী কে ছিলেন?", "হিজরতের কঠিন সফরে হযরত আবু বকর (রাঃ) ছায়ার মতো সঙ্গী ছিলেন।", "Easy", vec![
        ("হযরত আলী (রাঃ)", false), ("হযরত উমর (রাঃ)", false), ("হযরত আবু বকর (রাঃ)", true), ("হযরত উসমান (রাঃ)", false)
    ]).await?;

    add_q(&pool, "বদরের যুদ্ধ", "বদরের যুদ্ধে মুসলিম সৈন্য সংখ্যা কত ছিল?", "৩১৩ জন সাহাবী ছিলেন।", "Hard", vec![
        ("১০০০", false), ("৩১৩", true), ("৩০০", false), ("৫০", false)
    ]).await?;
    
    add_q(&pool, "মক্কা বিজয়", "মক্কা বিজয়ের দিন রাসূল (সাঃ) কুরাইশদের সাথে কেমন আচরণ করেছিলেন?", "'আজ তোমাদের বিরুদ্ধে কোন অভিযোগ নেই, তোমরা মুক্ত।'", "Medium", vec![
        ("সবাইকে ক্ষমা করে দেন", true), ("নেতাদের হত্যা করেন", false), ("বন্দী করেন", false), ("মক্কা থেকে বের করে দেন", false)
    ]).await?;

    println!("Database seeded successfully with Real Bengali History data!");
    Ok(())
}
