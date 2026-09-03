//! Cursor-based pagination shared by the feed, comments, followers, etc.
//!
//! A cursor encodes `(created_at, id)` of the last item seen, base64'd. This is
//! stable under inserts (unlike `OFFSET`) and cheap to index.

use base64::Engine;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::core::error::AppError;

/// Query params accepted by any paginated endpoint: `?limit=20&cursor=...`.
#[derive(Debug, Deserialize)]
pub struct PageParams {
    #[serde(default)]
    pub cursor: Option<String>,
    #[serde(default)]
    pub limit: Option<i64>,
}

impl PageParams {
    /// Clamp the caller's limit into `[1, 50]`, defaulting to 20.
    pub fn limit(&self) -> i64 {
        self.limit.unwrap_or(20).clamp(1, 50)
    }

    pub fn decode_cursor(&self) -> Result<Option<Cursor>, AppError> {
        self.cursor.as_deref().map(Cursor::decode).transpose()
    }
}

/// The `(created_at, id)` keyset position.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cursor {
    pub created_at: DateTime<Utc>,
    pub id: Uuid,
}

impl Cursor {
    pub fn encode(&self) -> String {
        let raw = format!("{}|{}", self.created_at.timestamp_micros(), self.id);
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(raw)
    }

    pub fn decode(s: &str) -> Result<Self, AppError> {
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(s)
            .map_err(|_| AppError::bad_request("invalid cursor"))?;
        let raw = String::from_utf8(bytes).map_err(|_| AppError::bad_request("invalid cursor"))?;
        let (ts, id) = raw
            .split_once('|')
            .ok_or_else(|| AppError::bad_request("invalid cursor"))?;
        let micros: i64 = ts
            .parse()
            .map_err(|_| AppError::bad_request("invalid cursor"))?;
        Ok(Cursor {
            created_at: DateTime::from_timestamp_micros(micros)
                .ok_or_else(|| AppError::bad_request("invalid cursor"))?,
            id: id
                .parse()
                .map_err(|_| AppError::bad_request("invalid cursor"))?,
        })
    }
}

/// Standard paginated response envelope.
#[derive(Debug, Serialize)]
pub struct Page<T> {
    pub items: Vec<T>,
    /// Pass back as `?cursor=` to fetch the next page. `None` = end of list.
    pub next_cursor: Option<String>,
}

impl<T> Page<T> {
    /// Build a page from `limit + 1` fetched rows: if the extra row is present,
    /// there's a next page. `cursor_of` extracts the keyset from an item.
    pub fn build(mut rows: Vec<T>, limit: i64, cursor_of: impl Fn(&T) -> Cursor) -> Self {
        let has_more = rows.len() as i64 > limit;
        if has_more {
            rows.truncate(limit as usize);
        }
        let next_cursor = has_more
            .then(|| rows.last().map(|r| cursor_of(r).encode()))
            .flatten();
        Page {
            items: rows,
            next_cursor,
        }
    }
}
