use chrono::{DateTime, Utc};
use scylla::{DeserializeRow, SerializeRow};
use uuid::Uuid;

#[derive(Debug, SerializeRow, DeserializeRow, Clone)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub created_at: DateTime<Utc>,
}
