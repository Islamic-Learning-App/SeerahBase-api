-- Enable foreign key support
PRAGMA foreign_keys = ON;

-- Eras table to categorize periods (e.g., Meccan, Medinan)
CREATE TABLE IF NOT EXISTS eras (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    description TEXT,
    start_date DATE,
    end_date DATE
);

-- Events table for historical events
CREATE TABLE IF NOT EXISTS events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    era_id INTEGER,
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    event_date DATE,
    source TEXT,
    FOREIGN KEY (era_id) REFERENCES eras(id)
);

-- Questions table for MCQs
CREATE TABLE IF NOT EXISTS questions (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_id INTEGER,
    question_text TEXT NOT NULL,
    explanation TEXT,
    difficulty_level TEXT CHECK(difficulty_level IN ('Easy', 'Medium', 'Hard')) DEFAULT 'Medium',
    FOREIGN KEY (event_id) REFERENCES events(id)
);

-- Options table for each question
CREATE TABLE IF NOT EXISTS options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    question_id INTEGER NOT NULL,
    option_text TEXT NOT NULL,
    is_correct BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (question_id) REFERENCES questions(id)
);
