use rusqlite::{params, Connection, OptionalExtension, Result};
use crate::state::models::{MediaFilterType, MediaItem, MediaSortBy, SortOrder};

#[derive(Debug, Clone)]
pub struct NewMediaItem<'a> {
    pub file_hash: &'a str,
    pub original_relative_path: &'a str,
    pub current_relative_path: &'a str,
    pub file_size: i64,
    pub mime_type: &'a str,
    pub duration_seconds: Option<i64>,
    pub metadata_json: Option<&'a str>,
    pub album_id: Option<i64>,
}

/// Inserts a new media catalog record.
pub fn insert_media(conn: &Connection, item: &NewMediaItem) -> Result<i64> {
    conn.execute(
        "INSERT INTO media_items (
            file_hash, original_relative_path, current_relative_path,
            file_size, mime_type, duration_seconds, metadata_json, album_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        params![
            item.file_hash,
            item.original_relative_path,
            item.current_relative_path,
            item.file_size,
            item.mime_type,
            item.duration_seconds,
            item.metadata_json,
            item.album_id,
        ],
    )?;

    Ok(conn.last_insert_rowid())
}

/// Retrieves a paginated list of media items with optional album, type filtering, and search.
pub fn get_media_items(
    conn: &Connection,
    album_id: Option<i64>,
    limit: i64,
    offset: i64,
    filter: MediaFilterType,
    search: Option<&str>,
    sort_by: MediaSortBy,
    sort_order: SortOrder,
) -> Result<(Vec<MediaItem>, i64)> {
    let mut conditions = Vec::new();
    let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(aid) = album_id {
        conditions.push("album_id = ?".to_string());
        params_vec.push(Box::new(aid));
    }

    match filter {
        MediaFilterType::All => {}
        MediaFilterType::Images => {
            conditions.push("mime_type LIKE 'image/%'".to_string());
        }
        MediaFilterType::Videos => {
            conditions.push("mime_type LIKE 'video/%'".to_string());
        }
    }

    if let Some(query) = search {
        let trimmed = query.trim();
        if !trimmed.is_empty() {
            conditions.push("(original_relative_path LIKE ? OR current_relative_path LIKE ?)".to_string());
            let pattern = format!("%{}%", trimmed);
            params_vec.push(Box::new(pattern.clone()));
            params_vec.push(Box::new(pattern));
        }
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count total matching items
    let count_query = format!("SELECT COUNT(*) FROM media_items {}", where_clause);
    let mut count_stmt = conn.prepare(&count_query)?;
    let total_count: i64 = count_stmt.query_row(
        rusqlite::params_from_iter(params_vec.iter().map(|p| p.as_ref())),
        |row| row.get(0),
    )?;

    // Sorting column and order
    let sort_col = match sort_by {
        MediaSortBy::Date => "created_at",
        MediaSortBy::Name => "original_relative_path",
        MediaSortBy::Size => "file_size",
    };
    let order_str = match sort_order {
        SortOrder::Asc => "ASC",
        SortOrder::Desc => "DESC",
    };

    let select_query = format!(
        "SELECT id, file_hash, original_relative_path, current_relative_path,
                file_size, mime_type, duration_seconds, metadata_json, album_id, created_at
         FROM media_items {} ORDER BY {} {} LIMIT ? OFFSET ?",
        where_clause, sort_col, order_str
    );

    let mut select_stmt = conn.prepare(&select_query)?;
    let mut select_params = params_vec;
    select_params.push(Box::new(limit));
    select_params.push(Box::new(offset));

    let rows = select_stmt.query_map(
        rusqlite::params_from_iter(select_params.iter().map(|p| p.as_ref())),
        |row| {
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
        },
    )?;

    let mut items = Vec::new();
    for row in rows {
        items.push(row?);
    }

    Ok((items, total_count))
}

/// Retrieves a media item and its previous and next neighbor IDs for slideshow navigation.
pub fn get_media_item_with_neighbors(
    conn: &Connection,
    item_id: i64,
    album_id: Option<i64>,
) -> Result<Option<(MediaItem, Option<i64>, Option<i64>)>> {
    let mut item_stmt = conn.prepare(
        "SELECT id, file_hash, original_relative_path, current_relative_path,
                file_size, mime_type, duration_seconds, metadata_json, album_id, created_at
         FROM media_items WHERE id = ?1",
    )?;

    let current: Option<MediaItem> = item_stmt.query_row(params![item_id], |row| {
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

    let item = match current {
        Some(item) => item,
        None => return Ok(None),
    };

    // Prev ID
    let prev_id: Option<i64> = if let Some(aid) = album_id {
        conn.query_row(
            "SELECT id FROM media_items WHERE album_id = ?1 AND id < ?2 ORDER BY id DESC LIMIT 1",
            params![aid, item_id],
            |row| row.get(0),
        ).optional()?
    } else {
        conn.query_row(
            "SELECT id FROM media_items WHERE id < ?1 ORDER BY id DESC LIMIT 1",
            params![item_id],
            |row| row.get(0),
        ).optional()?
    };

    // Next ID
    let next_id: Option<i64> = if let Some(aid) = album_id {
        conn.query_row(
            "SELECT id FROM media_items WHERE album_id = ?1 AND id > ?2 ORDER BY id ASC LIMIT 1",
            params![aid, item_id],
            |row| row.get(0),
        ).optional()?
    } else {
        conn.query_row(
            "SELECT id FROM media_items WHERE id > ?1 ORDER BY id ASC LIMIT 1",
            params![item_id],
            |row| row.get(0),
        ).optional()?
    };

    Ok(Some((item, prev_id, next_id)))
}

/// Relocates selected media items to a target album in the database.
pub fn move_media_to_album(
    conn: &Connection,
    media_ids: &[i64],
    target_album_id: i64,
) -> Result<usize> {
    if media_ids.is_empty() {
        return Ok(0);
    }

    let mut moved = 0;
    let mut stmt = conn.prepare("UPDATE media_items SET album_id = ?1 WHERE id = ?2")?;

    for &id in media_ids {
        moved += stmt.execute(params![target_album_id, id])?;
    }

    Ok(moved)
}

/// Updates both the album ID and current relative path for a media item.
pub fn update_media_album_and_path(
    conn: &Connection,
    item_id: i64,
    target_album_id: i64,
    new_relative_path: &str,
) -> Result<()> {
    conn.execute(
        "UPDATE media_items SET album_id = ?1, current_relative_path = ?2 WHERE id = ?3",
        params![target_album_id, new_relative_path, item_id],
    )?;
    Ok(())
}

/// Deletes media items by their primary keys.
pub fn delete_media_items(conn: &Connection, media_ids: &[i64]) -> Result<usize> {
    if media_ids.is_empty() {
        return Ok(0);
    }

    let mut deleted = 0;
    let mut stmt = conn.prepare("DELETE FROM media_items WHERE id = ?1")?;

    for &id in media_ids {
        deleted += stmt.execute(params![id])?;
    }

    Ok(deleted)
}

/// Retrieves (id, current_relative_path) pairs for all registered media items for filesystem audit.
pub fn get_all_media_for_audit(conn: &Connection) -> Result<Vec<(i64, String)>> {
    let mut stmt = conn.prepare("SELECT id, current_relative_path FROM media_items")?;
    let rows = stmt.query_map([], |row| {
        Ok((row.get(0)?, row.get(1)?))
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }
    Ok(result)
}

/// Checks whether a media item already exists with the given relative path.
pub fn media_exists_by_path(conn: &Connection, path: &str) -> Result<bool> {
    let mut stmt = conn.prepare("SELECT 1 FROM media_items WHERE current_relative_path = ?1 LIMIT 1")?;
    let exists = stmt.exists(params![path])?;
    Ok(exists)
}
