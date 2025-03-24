use std::fs;

use super::{config::codes::ResultCode, crypt::CryptService};
use crate::{AppResult, Failure, APP_NAME, CONFIG_NAME};
use records::{Record, RecordKey};
use rusqlite::{params, Connection};

const DB_FILE: &str = "sagetools.db";

pub mod records;

pub struct DBService {
    conn: Connection,
    crypt_service: CryptService,
}

impl DBService {
    pub fn new(crypt: Option<CryptService>) -> AppResult<Self> {
        let config_file_path =
            confy::get_configuration_file_path(APP_NAME, CONFIG_NAME).map_err(|e| Failure {
                message: "Failed to get configuration file path".to_string(),
                trace: format!("Reason: {}", e),
                code: ResultCode::PathError,
            })?;

        let config_dir = config_file_path
            .parent()
            .ok_or(Failure {
                message: "Failed to determine configuration directory".to_string(),
                trace: "Reason: Parent directory does not exist".to_string(),
                code: ResultCode::PathError,
            })?
            .to_path_buf();

        fs::create_dir_all(&config_dir).map_err(|e| Failure {
            message: "Failed to create config directory".to_string(),
            trace: format!("Reason: {}", e),
            code: ResultCode::PathError,
        })?;

        let db_path = config_dir.join(DB_FILE);
        let conn = Connection::open(db_path).map_err(|e| Failure {
            message: "Failed to open SQLite database".to_string(),
            trace: format!("Reason: {}", e),
            code: ResultCode::DbError,
        })?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS records (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL
            )",
            [],
        )
        .map_err(|e| Failure {
            message: "Failed to create table".to_string(),
            trace: format!("Reason: {}", e),
            code: ResultCode::DbError,
        })?;

        let crypt_service;
        match crypt {
            Some(g) => {
                crypt_service = g;
            }
            None => {
                crypt_service = CryptService::new()?;
            }
        }
        Ok(Self {
            conn,
            crypt_service,
        })
    }

    pub fn exists_record(&self, key: &RecordKey) -> AppResult<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT 1 FROM records WHERE id = ?1")
            .map_err(|e| Failure {
                message: "Failed to prepare statement".to_string(),
                trace: format!("Reason: {}", e),
                code: ResultCode::DbError,
            })?;

        let exists = stmt.exists(params![key.as_string()]).map_err(|e| Failure {
            message: "Failed to check record existence".to_string(),
            trace: format!("Reason: {}", e),
            code: ResultCode::DbError,
        })?;

        Ok(exists)
    }

    pub fn create_or_update_record(&self, record: &Record) -> AppResult<()> {
        let json_str = serde_json::to_string(record).map_err(|e| Failure {
            message: "Failed to serialize record".to_string(),
            trace: format!("Reason: {}", e),
            code: ResultCode::SerializationError,
        })?;
        let encrypted = self.crypt_service.encrypt(&json_str)?;

        self.conn
            .execute(
                "INSERT INTO records (id, data) VALUES (?1, ?2)
             ON CONFLICT(id) DO UPDATE SET data = excluded.data",
                params![record.key.as_string(), encrypted],
            )
            .map_err(|e| Failure {
                message: "Failed to insert or update record".to_string(),
                trace: format!("Reason: {}", e),
                code: ResultCode::DbError,
            })?;

        Ok(())
    }

    pub fn get_record(&self, key: &RecordKey) -> AppResult<Option<Record>> {
        let mut stmt = self
            .conn
            .prepare("SELECT data FROM records WHERE id = ?1")
            .map_err(|e| Failure {
                message: "Failed to prepare statement".to_string(),
                trace: format!("Reason: {}", e),
                code: ResultCode::DbError,
            })?;

        let encrypted_opt: Option<String> =
            match stmt.query_row(params![key.as_string()], |row| row.get(0)) {
                Ok(value) => Some(value),
                Err(rusqlite::Error::QueryReturnedNoRows) => None,
                Err(e) => {
                    return Err(Failure {
                        message: "Failed to fetch record".to_string(),
                        trace: format!("Reason: {}", e),
                        code: ResultCode::DbError,
                    });
                }
            };

        if let Some(encrypted) = encrypted_opt {
            let json_str = self.crypt_service.decrypt(&encrypted).map_err(|e| Failure {
                message: "Failed to decrypt record".to_string(),
                trace: format!("Reason: {}", e.message),
                code: ResultCode::SerializationError,
            })?;
            let record: Record = serde_json::from_str(&json_str).map_err(|e| Failure {
                message: "Failed to deserialize record".to_string(),
                trace: format!("Reason: {}", e),
                code: ResultCode::SerializationError,
            })?;
            Ok(Some(record))
        } else {
            Ok(None)
        }
    }

    pub fn delete_record(&self, key: &RecordKey) -> AppResult<()> {
        self.conn
            .execute("DELETE FROM records WHERE id = ?1", params![key.as_string()])
            .map_err(|e| Failure {
                message: "Failed to delete record".to_string(),
                trace: format!("Reason: {}", e),
                code: ResultCode::DbError,
            })?;
        Ok(())
    }
}
