---
title: Database Design & SQLite Layer
tags: [database, sqlite, rusqlite, schema, simple-design]
created: 2026-10-31
---

# Database Design & SQLite Layer

> [!NOTE] Design Principle
> As requested, **database logic is kept simple, transparent, and direct**. We avoid complex ORM layers and use straightforward `rusqlite` queries with prepared statements, explicit SQL, and atomic transactions.

---

## 1. Database Location & Portability

The SQLite database file (`.media_library.db`) resides directly on the designated album directory of the external drive:

```text
/Volumes/ExternalDrive/MyMedia/
├── .media_library.db          <-- SQLite catalog database
├── .backups/                  <-- Automatic database snapshots
└── albums/                    <-- Media assets organized by album
    ├── Summer_2026/
    └── unknown/
```

When a user switches active drives, the application cleanly closes the previous SQLite connection and opens a connection to the newly selected drive's `.media_library.db`.

---

## 2. Table Schemas & Triggers

```sql
-- 1. Media Items Table
CREATE TABLE IF NOT EXISTS media_items (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    file_hash TEXT NOT NULL,                     -- Fingerprint: name_size_mtime
    original_relative_path TEXT UNIQUE NOT NULL, -- Relative path prior to organization
    current_relative_path TEXT UNIQUE NOT NULL,  -- Current disk relative path
    file_size INTEGER NOT NULL,                  -- Size in bytes
    mime_type TEXT NOT NULL,                     -- e.g., image/jpeg, video/mp4
    duration_seconds INTEGER DEFAULT NULL,       -- Video/Audio length
    metadata_json TEXT,                          -- EXIF / Camera metadata
    album_id INTEGER,                            -- Foreign key to albums(id)
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 2. Albums Table
CREATE TABLE IF NOT EXISTS albums (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT UNIQUE NOT NULL,
    relative_path TEXT UNIQUE NOT NULL,
    description TEXT,
    media_count INTEGER NOT NULL DEFAULT 0,      -- Maintained via triggers
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- 3. Scanned Album Sources (Tracks original source folder -> album mappings)
CREATE TABLE IF NOT EXISTS scan_album_sources (
    source_relative_path TEXT PRIMARY KEY,
    album_id INTEGER NOT NULL UNIQUE
);

-- 4. Tags & Junction Table
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

-- Indices for rapid queries
CREATE INDEX IF NOT EXISTS idx_media_album_created ON media_items(album_id, created_at);
CREATE INDEX IF NOT EXISTS idx_media_created ON media_items(created_at);
CREATE INDEX IF NOT EXISTS idx_media_mime ON media_items(mime_type);
CREATE INDEX IF NOT EXISTS idx_media_name ON media_items(original_relative_path, id);
CREATE INDEX IF NOT EXISTS idx_media_size ON media_items(file_size, id);
CREATE INDEX IF NOT EXISTS idx_media_hash ON media_items(file_hash);

-- Triggers to maintain stored media_count on albums
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
```

---

## 3. Rust Data Models

```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaItem {
    pub id: i64,
    pub file_hash: String,
    pub original_relative_path: String,
    pub current_relative_path: String,
    pub file_size: i64,
    pub mime_type: String,
    pub duration_seconds: Option<i64>,
    pub metadata_json: Option<String>,
    pub album_id: Option<i64>,
    pub created_at: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Album {
    pub id: i64,
    pub name: String,
    pub relative_path: String,
    pub description: Option<String>,
    pub media_count: i64,
    pub created_at: String,
    pub preview_item: Option<MediaItem>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub color_hex: String,
    pub category: String,
}
```

---

## 4. Simple Database Connection Helper

```rust
use rusqlite::{Connection, Result};
use std::path::Path;

pub struct LibraryDatabase {
    conn: Connection,
}

impl LibraryDatabase {
    pub fn open(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        // Optimal PRAGMAs for removable drives
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA foreign_keys = ON;
        ")?;
        
        let db = Self { conn };
        db.init_schema()?;
        Ok(db)
    }

    pub fn init_schema(&self) -> Result<()> {
        self.conn.execute_batch(include_str!("schema.sql"))?;
        // Ensure fallback 'unknown' album exists
        self.conn.execute(
            "INSERT OR IGNORE INTO albums (name, relative_path, description) VALUES ('unknown', 'albums/unknown', 'Default album for unsorted media')",
            [],
        )?;
        Ok(())
    }
}
```

---

## 5. Typical Queries & Pagination

### Paginated Media Catalog
```rust
pub fn get_media_items(
    &self,
    limit: i64,
    offset: i64,
    filter_type: Option<&str>,
    search_query: Option<&str>,
) -> Result<(Vec<MediaItem>, i64)> {
    // Dynamic WHERE clause building with prepared parameters
    // Returns (items, total_count)
}
```

### Next / Previous Seek
```rust
pub fn get_media_item_with_neighbors(
    &self,
    item_id: i64,
    album_id: Option<i64>,
) -> Result<(MediaItem, Option<i64>, Option<i64>)> {
    // Fetches current item, previous ID, and next ID for seamless slideshow navigation
}
```
