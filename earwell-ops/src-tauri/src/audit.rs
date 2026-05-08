use chrono::Utc;
use rusqlite::{params, Connection};
use serde::Serialize;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::error::{AppError, AppResult};

#[derive(Debug, Serialize)]
pub struct AuditEntry<'a> {
    pub action:    &'a str,
    pub actor_id:  Option<&'a str>,
    pub entity_id: Option<&'a str>,
    pub payload:   serde_json::Value,
}

/// Append a tamper-evident record. Hash is SHA-256(prev_hash || canonical-record).
pub fn append(conn: &Connection, e: &AuditEntry<'_>) -> AppResult<()> {
    let record_id   = Uuid::now_v7().to_string();
    let occurred_at = Utc::now().to_rfc3339();
    let payload_json = serde_json::to_string(&e.payload)
        .map_err(|err| AppError::Internal(format!("audit serialize: {err}")))?;

    let prev_hash: Vec<u8> = match conn.query_row(
        "SELECT hash FROM audit_log ORDER BY id DESC LIMIT 1",
        [],
        |r| r.get::<_, Vec<u8>>(0),
    ) {
        Ok(h) => h,
        Err(rusqlite::Error::QueryReturnedNoRows) => vec![0u8; 32], // genesis
        Err(other) => return Err(other.into()),
    };

    // Stable key ordering => stable hash.
    let canonical = serde_json::json!({
        "record_id":   record_id,
        "occurred_at": occurred_at,
        "actor_id":    e.actor_id,
        "action":      e.action,
        "entity_id":   e.entity_id,
        "payload":     e.payload,
    });
    let canonical_bytes = serde_json::to_vec(&canonical)
        .map_err(|err| AppError::Internal(format!("audit canonical: {err}")))?;

    let mut hasher = Sha256::new();
    hasher.update(&prev_hash);
    hasher.update(&canonical_bytes);
    let hash = hasher.finalize().to_vec();

    conn.execute(
        "INSERT INTO audit_log
            (record_id, occurred_at, actor_id, action, entity_id, payload_json, prev_hash, hash)
         VALUES (?1,?2,?3,?4,?5,?6,?7,?8)",
        params![record_id, occurred_at, e.actor_id, e.action, e.entity_id,
                payload_json, prev_hash, hash],
    )?;
    Ok(())
}

/// Walk the chain. Returns (verified_count, first_corrupt_id).
pub fn verify(conn: &Connection) -> AppResult<(u64, Option<i64>)> {
    let mut stmt = conn.prepare(
        "SELECT id, record_id, occurred_at, actor_id, action, entity_id,
                payload_json, prev_hash, hash
           FROM audit_log ORDER BY id ASC",
    )?;
    let mut rows = stmt.query([])?;

    let mut prev = vec![0u8; 32];
    let mut count = 0u64;
    while let Some(row) = rows.next()? {
        let id: i64                    = row.get(0)?;
        let record_id: String          = row.get(1)?;
        let occurred_at: String        = row.get(2)?;
        let actor_id: Option<String>   = row.get(3)?;
        let action: String             = row.get(4)?;
        let entity_id: Option<String>  = row.get(5)?;
        let payload_json: String       = row.get(6)?;
        let stored_prev: Vec<u8>       = row.get(7)?;
        let stored_hash: Vec<u8>       = row.get(8)?;

        if stored_prev != prev { return Ok((count, Some(id))); }

        let payload: serde_json::Value = serde_json::from_str(&payload_json)
            .map_err(|err| AppError::Internal(format!("audit verify parse: {err}")))?;
        let canonical = serde_json::json!({
            "record_id": record_id, "occurred_at": occurred_at, "actor_id": actor_id,
            "action": action, "entity_id": entity_id, "payload": payload,
        });
        let canonical_bytes = serde_json::to_vec(&canonical)
            .map_err(|err| AppError::Internal(format!("audit verify canonical: {err}")))?;
        let mut h = Sha256::new();
        h.update(&prev);
        h.update(&canonical_bytes);
        if h.finalize().to_vec() != stored_hash { return Ok((count, Some(id))); }

        prev = stored_hash;
        count += 1;
    }
    Ok((count, None))
}
