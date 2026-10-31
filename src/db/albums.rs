use rusqlite::{params, Connection, OptionalExtension, Result};
use crate::state::models::{Album, MediaItem};

/// Inserts a new album directory entry.
pub fn create_album(
    conn: &Connection,
    name: &str,
    relative_path: &str,
    description: Option<&str>,
) -> Result<Album> {
    conn.execute(
        "INSERT INTO albums (name, relative_path, description) VALUES (?1, ?2, ?3)",
        params![name, relative_path, description],
    )?;

    let id = conn.last_insert_rowid();
    get_album_by_id(conn, id)?.ok_or_else(|| {
        rusqlite::Error::QueryReturnedNoRows
    })
}

/// Retrieves an existing album by name or creates it if not found.
pub fn get_or_create_album(
    conn: &Connection,
    name: &str,
    relative_path: &str,
) -> Result<Album> {
    let existing: Option<Album> = conn.query_row(
        "SELECT id, name, relative_path, description, media_count, created_at FROM albums WHERE name = ?1",
        params![name],
        |row| {
            Ok(Album {
                id: row.get(0)?,
                name: row.get(1)?,
                relative_path: row.get(2)?,
                description: row.get(3)?,
                media_count: row.get(4)?,
                created_at: row.get(5)?,
                preview_item: None,
            })
        },
    ).optional()?;

    if let Some(album) = existing {
        Ok(album)
    } else {
        create_album(conn, name, relative_path, None)
    }
}

/// Recalculates and synchronizes `media_count` for all albums based on current `media_items`.
pub fn recalculate_all_album_counts(conn: &Connection) -> Result<()> {
    conn.execute(
        "UPDATE albums SET media_count = (SELECT COUNT(*) FROM media_items WHERE media_items.album_id = albums.id)",
        [],
    )?;
    Ok(())
}

/// Retrieves a single album by its ID.
pub fn get_album_by_id(conn: &Connection, album_id: i64) -> Result<Option<Album>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, relative_path, description, media_count, created_at FROM albums WHERE id = ?1",
    )?;

    let album = stmt.query_row(params![album_id], |row| {
        Ok(Album {
            id: row.get(0)?,
            name: row.get(1)?,
            relative_path: row.get(2)?,
            description: row.get(3)?,
            media_count: row.get(4)?,
            created_at: row.get(5)?,
            preview_item: None,
        })
    }).optional()?;

    Ok(album)
}

/// Lists all albums with their stored count and newest preview media item.
pub fn get_albums(conn: &Connection) -> Result<Vec<Album>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, relative_path, description, media_count, created_at FROM albums ORDER BY name ASC",
    )?;

    let album_rows = stmt.query_map([], |row| {
        Ok(Album {
            id: row.get(0)?,
            name: row.get(1)?,
            relative_path: row.get(2)?,
            description: row.get(3)?,
            media_count: row.get(4)?,
            created_at: row.get(5)?,
            preview_item: None,
        })
    })?;

    let mut albums = Vec::new();
    for album in album_rows {
        let mut album = album?;
        // Query newest preview item for album cover
        let mut preview_stmt = conn.prepare(
            "SELECT id, file_hash, original_relative_path, current_relative_path, file_size, mime_type, duration_seconds, metadata_json, album_id, created_at
             FROM media_items WHERE album_id = ?1 ORDER BY created_at DESC LIMIT 1",
        )?;

        album.preview_item = preview_stmt.query_row(params![album.id], |row| {
            Ok(MediaItem {
                id: row.get(0)?,
                file_hash: row.get(1)?,
                original_relative_path: row.get(2)?,
                current_relative_path: row.get(3)?,
                file_size: row.get(4)?,
                mime_type: row.get(5)?,
                duration_seconds: row.get(6)?,
                metadata_json: row.get(7)?,
                album_id: row.get(8)?,
                created_at: row.get(9)?,
            })
        }).optional()?;

        albums.push(album);
    }

    Ok(albums)
}

/// Renames an album name and relative path.
pub fn rename_album(
    conn: &Connection,
    album_id: i64,
    new_name: &str,
    new_relative_path: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE albums SET name = ?1, relative_path = ?2 WHERE id = ?3",
        params![new_name, new_relative_path, album_id],
    )?;
    Ok(())
}

/// Deletes an album. If permanent is false, moves contained media to the 'unknown' album.
pub fn delete_album(conn: &Connection, album_id: i64, permanent: bool) -> Result<()> {
    if permanent {
        conn.execute("DELETE FROM media_items WHERE album_id = ?1", params![album_id])?;
    } else {
        // Find unknown album ID
        let unknown_id: Option<i64> = conn.query_row(
            "SELECT id FROM albums WHERE name = 'unknown'",
            [],
            |row| row.get(0),
        ).optional()?;

        if let Some(uid) = unknown_id {
            conn.execute(
                "UPDATE media_items SET album_id = ?1 WHERE album_id = ?2",
                params![uid, album_id],
            )?;
        }
    }

    conn.execute("DELETE FROM scan_album_sources WHERE album_id = ?1", params![album_id])?;
    conn.execute("DELETE FROM albums WHERE id = ?1", params![album_id])?;
    Ok(())
}

/// Merges media items from a source album into a target album.
pub fn merge_albums(conn: &Connection, source_album_id: i64, target_album_id: i64) -> Result<usize> {
    let affected = conn.execute(
        "UPDATE media_items SET album_id = ?1 WHERE album_id = ?2",
        params![target_album_id, source_album_id],
    )?;

    // Clean up source album
    conn.execute("DELETE FROM scan_album_sources WHERE album_id = ?1", params![source_album_id])?;
    conn.execute("DELETE FROM albums WHERE id = ?1", params![source_album_id])?;

    Ok(affected)
}
