pub mod schema;
pub mod albums;
pub mod media;

use std::path::Path;
use std::sync::{Mutex, MutexGuard};
use rusqlite::{Connection, Result};
use crate::state::models::{Album, MediaFilterType, MediaItem, MediaSortBy, SortOrder};
pub use media::NewMediaItem;

/// Encapsulates a thread-safe connection to a `.media_library.db` database on a drive.
pub struct LibraryDatabase {
    conn: Mutex<Connection>,
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
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Opens an in-memory SQLite database, primarily for testing.
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        schema::init_schema(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    /// Direct reference to the underlying connection guard.
    pub fn conn(&self) -> MutexGuard<'_, Connection> {
        self.conn.lock().unwrap()
    }

    // --- Albums ---

    pub fn create_album(
        &self,
        name: &str,
        relative_path: &str,
        description: Option<&str>,
    ) -> Result<Album> {
        let conn = self.conn.lock().unwrap();
        albums::create_album(&conn, name, relative_path, description)
    }

    pub fn get_albums(&self) -> Result<Vec<Album>> {
        let conn = self.conn.lock().unwrap();
        albums::get_albums(&conn)
    }

    pub fn get_album_by_id(&self, album_id: i64) -> Result<Option<Album>> {
        let conn = self.conn.lock().unwrap();
        albums::get_album_by_id(&conn, album_id)
    }

    pub fn rename_album(
        &self,
        album_id: i64,
        new_name: &str,
        new_relative_path: &str,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        albums::rename_album(&conn, album_id, new_name, new_relative_path)
    }

    pub fn delete_album(&self, album_id: i64, permanent: bool) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        albums::delete_album(&conn, album_id, permanent)
    }

    pub fn merge_albums(&self, source_album_id: i64, target_album_id: i64) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        albums::merge_albums(&conn, source_album_id, target_album_id)
    }

    // --- Media ---

    pub fn insert_media(&self, item: &NewMediaItem) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        media::insert_media(&conn, item)
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
        let conn = self.conn.lock().unwrap();
        media::get_media_items(&conn, album_id, limit, offset, filter, search, sort_by, sort_order)
    }

    pub fn get_media_item_with_neighbors(
        &self,
        item_id: i64,
        album_id: Option<i64>,
    ) -> Result<Option<(MediaItem, Option<i64>, Option<i64>)>> {
        let conn = self.conn.lock().unwrap();
        media::get_media_item_with_neighbors(&conn, item_id, album_id)
    }

    pub fn move_media_to_album(&self, media_ids: &[i64], target_album_id: i64) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        media::move_media_to_album(&conn, media_ids, target_album_id)
    }

    pub fn update_media_album_and_path(
        &self,
        item_id: i64,
        target_album_id: i64,
        new_relative_path: &str,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        media::update_media_album_and_path(&conn, item_id, target_album_id, new_relative_path)
    }

    pub fn delete_media_items(&self, media_ids: &[i64]) -> Result<usize> {
        let conn = self.conn.lock().unwrap();
        media::delete_media_items(&conn, media_ids)
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

        let item = NewMediaItem {
            file_hash: "test_hash_1",
            original_relative_path: "DCIM/100CANON/IMG_0001.JPG",
            current_relative_path: "albums/Summer_2026/IMG_0001.JPG",
            file_size: 1048576,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        };
        let media_id = db.insert_media(&item).expect("Failed to insert media");
        assert!(media_id > 0);

        // Verify count trigger incremented album count
        let fetched_album = db.get_album_by_id(album.id).expect("Failed to get album").unwrap();
        assert_eq!(fetched_album.media_count, 1);
    }

    #[test]
    fn test_move_media_and_trigger_balance() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB");

        let album_a = db.create_album("Album_A", "albums/Album_A", None).unwrap();
        let album_b = db.create_album("Album_B", "albums/Album_B", None).unwrap();

        let item = NewMediaItem {
            file_hash: "hash_move_1",
            original_relative_path: "orig.jpg",
            current_relative_path: "albums/Album_A/orig.jpg",
            file_size: 512,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album_a.id),
        };
        let mid = db.insert_media(&item).unwrap();

        assert_eq!(db.get_album_by_id(album_a.id).unwrap().unwrap().media_count, 1);
        assert_eq!(db.get_album_by_id(album_b.id).unwrap().unwrap().media_count, 0);

        // Move to Album B
        let moved = db.move_media_to_album(&[mid], album_b.id).unwrap();
        assert_eq!(moved, 1);

        // Verify counts after move
        assert_eq!(db.get_album_by_id(album_a.id).unwrap().unwrap().media_count, 0);
        assert_eq!(db.get_album_by_id(album_b.id).unwrap().unwrap().media_count, 1);

        // Test update_media_album_and_path
        db.update_media_album_and_path(mid, album_a.id, "albums/Album_A/new_path.jpg").unwrap();
        assert_eq!(db.get_album_by_id(album_a.id).unwrap().unwrap().media_count, 1);
        assert_eq!(db.get_album_by_id(album_b.id).unwrap().unwrap().media_count, 0);
    }

    #[test]
    fn test_media_neighbors_seeking() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB");
        let album = db.create_album("SeqAlbum", "albums/SeqAlbum", None).unwrap();

        let item1 = NewMediaItem {
            file_hash: "seq_1",
            original_relative_path: "1.jpg",
            current_relative_path: "albums/SeqAlbum/1.jpg",
            file_size: 100,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        };
        let id1 = db.insert_media(&item1).unwrap();

        let item2 = NewMediaItem {
            file_hash: "seq_2",
            original_relative_path: "2.jpg",
            current_relative_path: "albums/SeqAlbum/2.jpg",
            file_size: 200,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        };
        let id2 = db.insert_media(&item2).unwrap();

        let item3 = NewMediaItem {
            file_hash: "seq_3",
            original_relative_path: "3.jpg",
            current_relative_path: "albums/SeqAlbum/3.jpg",
            file_size: 300,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album.id),
        };
        let id3 = db.insert_media(&item3).unwrap();

        // Check middle item neighbors
        let res = db.get_media_item_with_neighbors(id2, Some(album.id)).unwrap().unwrap();
        assert_eq!(res.0.id, id2);
        assert_eq!(res.1, Some(id1)); // prev
        assert_eq!(res.2, Some(id3)); // next
    }

    #[test]
    fn test_album_merge_and_deletion() {
        let db = LibraryDatabase::open_in_memory().expect("Failed to open DB");

        let album_src = db.create_album("Source", "albums/Source", None).unwrap();
        let album_dst = db.create_album("Target", "albums/Target", None).unwrap();

        let item = NewMediaItem {
            file_hash: "merge_test_1",
            original_relative_path: "test.jpg",
            current_relative_path: "albums/Source/test.jpg",
            file_size: 1024,
            mime_type: "image/jpeg",
            duration_seconds: None,
            metadata_json: None,
            album_id: Some(album_src.id),
        };
        let mid = db.insert_media(&item).unwrap();

        // Merge Source into Target
        let merged_count = db.merge_albums(album_src.id, album_dst.id).unwrap();
        assert_eq!(merged_count, 1);

        // Source album is deleted
        assert!(db.get_album_by_id(album_src.id).unwrap().is_none());
        // Target album count is updated
        assert_eq!(db.get_album_by_id(album_dst.id).unwrap().unwrap().media_count, 1);

        // Safe delete Target album -> moves items to unknown
        db.delete_album(album_dst.id, false).unwrap();
        assert!(db.get_album_by_id(album_dst.id).unwrap().is_none());

        // Verify media item still exists and is in unknown album
        let unknown = db.get_albums().unwrap().into_iter().find(|a| a.name == "unknown").unwrap();
        assert_eq!(unknown.media_count, 1);

        // Permanent delete item
        db.delete_media_items(&[mid]).unwrap();
        let unknown_after = db.get_albums().unwrap().into_iter().find(|a| a.name == "unknown").unwrap();
        assert_eq!(unknown_after.media_count, 0);
    }
}
