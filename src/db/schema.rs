use rusqlite::{Connection, Result};

pub const SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS media_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_hash TEXT NOT NULL,
    original_relative_path TEXT UNIQUE NOT NULL,
    current_relative_path TEXT UNIQUE NOT NULL,
    file_size INTEGER NOT NULL,
    mime_type TEXT NOT NULL,
    duration_seconds INTEGER DEFAULT NULL,
    metadata_json TEXT,
    album_id INTEGER,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS albums (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    relative_path TEXT UNIQUE NOT NULL,
    description TEXT,
    media_count INTEGER NOT NULL DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE IF NOT EXISTS scan_album_sources (
    source_relative_path TEXT PRIMARY KEY,
    album_id INTEGER NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS tags (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    color_hex TEXT NOT NULL DEFAULT '#3B82F6',
    category TEXT DEFAULT 'General'
);

CREATE TABLE IF NOT EXISTS media_tags (
    media_id INTEGER,
    tag_id INTEGER,
    PRIMARY KEY (media_id, tag_id),
    FOREIGN KEY (media_id) REFERENCES media_items(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

CREATE INDEX IF NOT EXISTS idx_media_album_created ON media_items(album_id, created_at);
CREATE INDEX IF NOT EXISTS idx_media_created ON media_items(created_at);
CREATE INDEX IF NOT EXISTS idx_media_mime ON media_items(mime_type);
CREATE INDEX IF NOT EXISTS idx_media_name ON media_items(original_relative_path, id);
CREATE INDEX IF NOT EXISTS idx_media_size ON media_items(file_size, id);
CREATE INDEX IF NOT EXISTS idx_media_hash ON media_items(file_hash);

CREATE TRIGGER IF NOT EXISTS album_count_insert AFTER INSERT ON media_items
WHEN new.album_id IS NOT NULL BEGIN
    UPDATE albums SET media_count = media_count + 1 WHERE id = new.album_id;
END;

CREATE TRIGGER IF NOT EXISTS album_count_delete AFTER DELETE ON media_items
WHEN old.album_id IS NOT NULL BEGIN
    UPDATE albums SET media_count = media_count - 1 WHERE id = old.album_id;
END;

CREATE TRIGGER IF NOT EXISTS album_count_update AFTER UPDATE OF album_id ON media_items
WHEN old.album_id IS NOT new.album_id BEGIN
    UPDATE albums SET media_count = media_count - 1 WHERE id = old.album_id;
    UPDATE albums SET media_count = media_count + 1 WHERE id = new.album_id;
END;
"#;

/// Initializes the database tables, indices, triggers, and creates default 'unknown' album.
pub fn init_schema(conn: &Connection) -> Result<()> {
    conn.execute_batch("
        PRAGMA foreign_keys = ON;
    ")?;

    conn.execute_batch(SCHEMA_SQL)?;

    // Ensure the default fallback 'unknown' album always exists
    conn.execute(
        "INSERT OR IGNORE INTO albums (name, relative_path, description) VALUES ('unknown', 'albums/unknown', 'Default album for unsorted media')",
        [],
    )?;

    Ok(())
}
