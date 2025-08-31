use rusqlite::{params, Connection, Result as SqliteResult, NO_PARAMS};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

const CURRENT_SCHEMA_VERSION: u32 = 1;

#[derive(Debug)]
pub struct Storage {
    conn: Arc<Mutex<Connection>>,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoredDocument {
    pub id: String,
    pub data: Vec<u8>,
    pub metadata: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Storage {
    pub fn new<P: AsRef<Path>>(path: P) -> SqliteResult<Self> {
        let conn = Connection::open(&path)?;
        let storage = Storage {
            conn: Arc::new(Mutex::new(conn)),
            path: path.as_ref().to_string_lossy().to_string(),
        };
        
        storage.init_db()?;
        Ok(storage)
    }
    
    fn init_db(&self) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        
        conn.pragma_update(None, "journal_mode", &"WAL")?;
        conn.pragma_update(None, "synchronous", &"NORMAL")?;
        
        let tx = conn.unchecked_transaction()?;
        
        tx.execute_batch(
            r#"
            PRAGMA user_version = 1;
            
            CREATE TABLE IF NOT EXISTS documents (
                id TEXT PRIMARY KEY,
                data BLOB NOT NULL,
                metadata TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            
            CREATE INDEX IF NOT EXISTS idx_documents_created_at ON documents(created_at);
            CREATE INDEX IF NOT EXISTS idx_documents_updated_at ON documents(updated_at);
            "#,
        )?;
        
        tx.commit()?;
        Ok(())
    }
    
    pub fn store_document(&self, id: &str, data: &[u8], metadata: &str) -> SqliteResult<()> {
        let conn = self.conn.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        
        conn.execute(
            r#"
            INSERT INTO documents (id, data, metadata, created_at, updated_at)
            VALUES (?1, ?2, ?3, ?4, ?5)
            ON CONFLICT(id) DO UPDATE SET
                data = excluded.data,
                metadata = excluded.metadata,
                updated_at = excluded.updated_at
            "#,
            params![id, data, metadata, now, now],
        )?;
        
        Ok(())
    }
    
    pub fn get_document(&self, id: &str) -> SqliteResult<Option<StoredDocument>> {
        let conn = self.conn.lock().unwrap();
        
        let mut stmt = conn.prepare(
            "SELECT id, data, metadata, created_at, updated_at FROM documents WHERE id = ?",
        )?;
        
        let mut rows = stmt.query_map(params![id], |row| {
            Ok(StoredDocument {
                id: row.get(0)?,
                data: row.get(1)?,
                metadata: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        
        match rows.next() {
            Some(row) => Ok(Some(row?)),
            None => Ok(None),
        }
    }
    
    pub fn delete_document(&self, id: &str) -> SqliteResult<bool> {
        let conn = self.conn.lock().unwrap();
        let count = conn.execute("DELETE FROM documents WHERE id = ?", params![id])?;
        Ok(count > 0)
    }
    
    pub fn list_documents(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> SqliteResult<Vec<StoredDocument>> {
        let conn = self.conn.lock().unwrap();
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        
        let mut stmt = conn.prepare(
            "SELECT id, data, metadata, created_at, updated_at FROM documents ORDER BY updated_at DESC LIMIT ? OFFSET ?",
        )?;
        
        let rows = stmt.query_map(params![limit, offset], |row| {
            Ok(StoredDocument {
                id: row.get(0)?,
                data: row.get(1)?,
                metadata: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        
        let mut documents = Vec::new();
        for row in rows {
            documents.push(row?);
        }
        
        Ok(documents)
    }
    
    pub fn search_documents(
        &self,
        query: &str,
        limit: Option<i64>,
    ) -> SqliteResult<Vec<StoredDocument>> {
        let conn = self.conn.lock().unwrap();
        let limit = limit.unwrap_or(100);
        let query = format!("%{}%", query);
        
        let mut stmt = conn.prepare(
            "SELECT id, data, metadata, created_at, updated_at FROM documents 
             WHERE id LIKE ?1 OR metadata LIKE ?1 
             ORDER BY updated_at DESC 
             LIMIT ?2",
        )?;
        
        let rows = stmt.query_map(params![query, limit], |row| {
            Ok(StoredDocument {
                id: row.get(0)?,
                data: row.get(1)?,
                metadata: row.get(2)?,
                created_at: row.get(3)?,
                updated_at: row.get(4)?,
            })
        })?;
        
        let mut documents = Vec::new();
        for row in rows {
            documents.push(row?);
        }
        
        Ok(documents)
    }
}

impl Clone for Storage {
    fn clone(&self) -> Self {
        Storage {
            conn: self.conn.clone(),
            path: self.path.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use serde_json::json;
    
    #[test]
    fn test_storage_operations() -> SqliteResult<()> {
        let temp_dir = tempdir()?;
        let db_path = temp_dir.path().join("test.db");
        let storage = Storage::new(&db_path)?;
        
        let id = "test-doc";
        let data = b"test data";
        let metadata = json!({ "key": "value" }).to_string();
        
        storage.store_document(id, data, &metadata)?;
        
        let doc = storage.get_document(id)?.unwrap();
        assert_eq!(doc.id, id);
        assert_eq!(doc.data, data);
        assert_eq!(doc.metadata, metadata);
        
        let docs = storage.list_documents(Some(10), Some(0))?;
        assert!(!docs.is_empty());
        
        let search_results = storage.search_documents("test", Some(10))?;
        assert!(!search_results.is_empty());
        
        let deleted = storage.delete_document(id)?;
        assert!(deleted);
        
        let doc = storage.get_document(id)?;
        assert!(doc.is_none());
        
        Ok(())
    }
}
