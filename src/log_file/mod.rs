// Level associated with log entry
pub enum LogLevel {
    DEBUG,
    INFO,
    WARN,
    ERROR,
}

// Singe long Line or entry
pub struct Entry {
    pub level: LogLevel,
    pub content: String,
}

impl Entry {
    pub fn new(level: LogLevel, content: String) -> Entry {
        Entry {
            level: level,
            content: content,
        }
    }
}
