-- Enable foreign key support
PRAGMA foreign_keys = ON;

-- Flexible category system (replaces eras)
CREATE TABLE IF NOT EXISTS categories (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    name_bn TEXT NOT NULL,             -- Bengali name
    category_type TEXT NOT NULL,        -- 'era', 'surah_group', 'prophet', 'topic'
    description TEXT,
    description_bn TEXT,
    icon TEXT,                          -- emoji icon
    sort_order INTEGER DEFAULT 0,
    parent_id INTEGER,                  -- for sub-categories
    FOREIGN KEY (parent_id) REFERENCES categories(id)
);

-- Events with category link
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    category_id INTEGER,
    title TEXT NOT NULL,
    title_bn TEXT,                      -- Bengali title
    description TEXT NOT NULL,
    description_bn TEXT,
    event_date TEXT,
    source TEXT,
    image_url TEXT,
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- Questions
CREATE TABLE IF NOT EXISTS questions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER,
    category_id INTEGER,               -- direct category link (optional)
    question_text TEXT NOT NULL,
    question_text_bn TEXT,
    explanation TEXT,
    explanation_bn TEXT,
    difficulty_level TEXT CHECK(difficulty_level IN ('Easy','Medium','Hard')) DEFAULT 'Medium',
    FOREIGN KEY (event_id) REFERENCES events(id),
    FOREIGN KEY (category_id) REFERENCES categories(id)
);

-- Options
CREATE TABLE IF NOT EXISTS options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_id INTEGER NOT NULL,
    option_text TEXT NOT NULL,
    option_text_bn TEXT,
    is_correct BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (question_id) REFERENCES questions(id)
);

-- Indexes for performance
CREATE INDEX IF NOT EXISTS idx_events_category ON events(category_id);
CREATE INDEX IF NOT EXISTS idx_questions_event ON questions(event_id);
CREATE INDEX IF NOT EXISTS idx_questions_category ON questions(category_id);
CREATE INDEX IF NOT EXISTS idx_options_question ON options(question_id);
CREATE INDEX IF NOT EXISTS idx_categories_type ON categories(category_type);
