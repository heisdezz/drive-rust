pub mod schema;
pub mod albums;
pub mod media;

use std::path::Path;
use rusqlite::{Connection, Result};
use crate::state::models::{Album, MediaFilterType, MediaItem, MediaSortBy, SortOrder};
pub use media::NewMediaItem;

/// Encapsulates a connection to a `.media_library.db` database on a drive.
pub struct LibraryDatabase {
    conn: Connection,
}

impl LibraryDatabase {
    /// Opens or creates a SQLite database at the specified path with WAL mode.
    pub fn open(db_path: &Path) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        conn.execute_batch("
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
        ")?;
        schema::init_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Opens an in-memory SQLite database, primarily for testing.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        schema::init_schema(&conn)?;
        Ok(Self { conn })
    }

    /// Direct reference to the underlying connection.
    pub fn conn(&self) -> &Connection {
        &self.conn
    }

    // --- Albums ---

    pub fn create_album(
        &self,
        name: &str,
        relative_path: &str,
        description: Option<&str>,
    ) -> Result<Album> {
        albums::create_album(&self.conn, name, relative_path, description)
    }

    pub fn get_albums(&self) -> Result<Vec<Album>> {
        albums::get_albums(&self.conn)
    }

    pub fn get_album_by_id(&self, album_id: i64) -> Result<Option<Album>> {
        albums::get_album_by_id(&self.conn, album_id)
    }

    pub fn rename_album(
        &self,
        album_id: i64,
        new_name: &str,
        new_relative_path: &str,
    ) -> Result<()> {
        albums::rename_album(&self.conn, album_id, new_name, new_relative_path)
    }

    pub fn delete_album(&self, album_id: i64, permanent: bool) -> Result<()> {
        albums::delete_album(&self.conn, album_id, permanent)
    }

    pub fn merge_albums(&self, source_album_id: i64, target_album_id: i64) -> Result<usize> {
        albums::merge_albums(&self.conn, source_album_id, target_album_id)
    }

    // --- Media ---

    pub fn insert_media(&self, item: &NewMediaItem) -> Result<i64> {
        media::insert_media(&self.conn, item)
    }

    pub fn get_media_items(
        &self,
        album_id: Option<i64>,
        limit: i64,
        offset: i64,
        filter: MediaFilterType,
        search: Option<&str>,
        sort_by: MediaSortBy,
        sort_order: SortOrder,
    ) -> Result<(Vec<MediaItem>, i64)> {
        media::get_media_items(&self.conn, album_id, limit, offset, filter, search, sort_by, sort_order)
    }

    pub fn get_media_item_with_neighbors(
        &self,
        item_id: i64,
        album_id: Option<i64>,
    ) -> Result<Option<(MediaItem, Option<i64>, Option<i64>)>> {
        media::get_media_item_with_neighbors(&self.conn, item_id, album_id)
    }

    pub fn move_media_to_album(&self, media_ids: &[i64], target_album_id: i64) -> Result<usize> {
        media::move_media_to_album(&self.conn, media_ids, target_album_id)
    }

    pub fn delete_media_items(&self, media_ids: &[i64]) -> Result<usize> {
        media::delete_media_items(&self.conn, media_ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_schema_init_and_unknown_album() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB in memory");
        let albums = db.get_albums().expect("Failed to fetch albums");
        assert_eq!(albums.len(), 1);
        assert_eq!(albums[0].name, "unknown");
        assert_eq!(albums[0].media_count, 0);
    }

    #[test]
    fn test_album_creation_and_count_triggers() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB");

        let album = db.create_album("Summer_2026", "albums/Summer_2026", Some("Holiday photos"))
            .expect("Failed to create album");
        assert_eq!(album.name, "Summer_2026");
        assert_eq!(album.media_count, 0);

        // Insert media items
        let item1 = NewMediaItem {
            file_hash: "hash_001",
            original_relative_path: "raw/p1.jpg",
            current_relative_path: "albums/Summer_2026/p1.jpg",
            file_size: 1024,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        };
        let id1 = db.insert_media(&item1).expect("Failed to insert media 1");
        assert!(id1 > 0);

        let item2 = NewMediaItem {
            file_hash: "hash_002",
            original_relative_path: "raw/v1.mp4",
            current_relative_path: "albums/Summer_2026/v1.mp4",
            file_size: 2048,
            mime_type: "video/mp4",
            duration_seconds: Some(120),
            metadata_json: None,
            album_id: Some(album.id),
        };
        db.insert_media(&item2).expect("Failed to insert media 2");

        // Verify trigger updated media_count
        let updated_album = db.get_album_by_id(album.id).unwrap().unwrap();
        assert_eq!(updated_album.media_count, 2);

        // Test filtering
        let (images, img_count) = db.get_media_items(
            Some(album.id), 10, 0, MediaFilterType::Images, None, MediaSortBy::Date, SortOrder::Desc
        ).unwrap();
        assert_eq!(img_count, 1);
        assert_eq!(images.len(), 1);
        assert_eq!(images[0].mime_type, "image/jpeg");

        let (videos, vid_count) = db.get_media_items(
            Some(album.id), 10, 0, MediaFilterType::Videos, None, MediaSortBy::Date, SortOrder::Desc
        ).unwrap();
        assert_eq!(vid_count, 1);
        assert_eq!(videos.len(), 1);
        assert_eq!(videos[0].mime_type, "video/mp4");
    }

    #[test]
    fn test_move_media_and_trigger_balance() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB");
        let album_a = db.create_album("AlbumA", "albums/AlbumA", None).unwrap();
        let album_b = db.create_album("AlbumB", "albums/AlbumB", None).unwrap();

        let item = NewMediaItem {
            file_hash: "hash_move",
            original_relative_path: "p.jpg",
            current_relative_path: "albums/AlbumA/p.jpg",
            file_size: 500,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album_a.id),
        };
        let item_id = db.insert_media(&item).unwrap();

        assert_eq!(db.get_album_by_id(album_a.id).unwrap().unwrap().media_count, 1);
        assert_eq!(db.get_album_by_id(album_b.id).unwrap().unwrap().media_count, 0);

        // Move to AlbumB
        let moved = db.move_media_to_album(&[item_id], album_b.id).unwrap();
        assert_eq!(moved, 1);

        assert_eq!(db.get_album_by_id(album_a.id).unwrap().unwrap().media_count, 0);
        assert_eq!(db.get_album_by_id(album_b.id).unwrap().unwrap().media_count, 1);
    }

    #[test]
    fn test_media_neighbors_seeking() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB");
        let album = db.create_album("Seq", "albums/Seq", None).unwrap();

        let id1 = db.insert_media(&NewMediaItem {
            file_hash: "h1",
            original_relative_path: "1.jpg",
            current_relative_path: "albums/Seq/1.jpg",
            file_size: 10,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        }).unwrap();

        let id2 = db.insert_media(&NewMediaItem {
            file_hash: "h2",
            original_relative_path: "2.jpg",
            current_relative_path: "albums/Seq/2.jpg",
            file_size: 10,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        }).unwrap();

        let id3 = db.insert_media(&NewMediaItem {
            file_hash: "h3",
            original_relative_path: "3.jpg",
            current_relative_path: "albums/Seq/3.jpg",
            file_size: 10,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        }).unwrap();

        // Check middle item (id2)
        let (item, prev, next) = db.get_media_item_with_neighbors(id2, Some(album.id)).unwrap().unwrap();
        assert_eq!(item.id, id2);
        assert_eq!(prev, Some(id1));
        assert_eq!(next, Some(id3));

        // Check first item (id1)
        let (_, prev_first, next_first) = db.get_media_item_with_neighbors(id1, Some(album.id)).unwrap().unwrap();
        assert_eq!(prev_first, None);
        assert_eq!(next_first, Some(id2));

        // Check last item (id3)
        let (_, prev_last, next_last) = db.get_media_item_with_neighbors(id3, Some(album.id)).unwrap().unwrap();
        assert_eq!(prev_last, Some(id2));
        assert_eq!(next_last, None);
    }
}
