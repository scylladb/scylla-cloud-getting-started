use scylla::{DeserializeRow, SerializeRow};
use uuid::Uuid;

use crate::datetime::DateTime;

#[derive(Debug, DeserializeRow, SerializeRow, Clone)]
pub struct Song {
    pub id: Uuid,
    pub title: String,
    pub album: String,
    pub artist: String,
    pub created_at: DateTime,
}
